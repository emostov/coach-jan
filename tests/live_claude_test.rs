//! Live integration test for Claude API.
//!
//! Only runs when ANTHROPIC_API_KEY is set.
//! Run with: cargo test --test live_claude_test -- --ignored --nocapture

use std::sync::Arc;

use coachjan::ai::client::{ClaudeClient, ContentBlock, Model};
use coachjan::ai::context::build_macrocycle_context;
use coachjan::ai::prompts::coach_jan_system_prompt;
use coachjan::ai::tools::generate_macrocycle_skeleton_tool;
use coachjan::db::profiles::{AthleteProfile, RaceGoal};

#[tokio::test]
#[ignore] // run with: cargo test -- --ignored
async fn test_real_claude_plan_generation() {
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping: ANTHROPIC_API_KEY not set");
            return;
        }
    };

    let client = Arc::new(ClaudeClient::new(api_key));

    // Create mock profile and race goal data
    let profile = AthleteProfile {
        id: 1,
        user_id: 1,
        name: "Test Runner".to_string(),
        age: 32,
        weight_kg: 70.0,
        resting_hr: 50,
        max_hr: 185,
        lthr: 168,
        ftpace_m_per_s: Some(3.7), // ~4:30/km
        current_weekly_volume_km: 40.0,
        experience_level: "intermediate".to_string(),
        sports_background: Some("cycling".to_string()),
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-01T00:00:00Z".to_string(),
    };

    let race_goal = RaceGoal {
        id: 1,
        user_id: 1,
        race_name: Some("Test Marathon".to_string()),
        distance_m: 42195.0,
        race_date: "2026-06-15".to_string(),
        target_time_seconds: Some(12600), // 3:30:00
        is_active: true,
        created_at: "2026-01-01T00:00:00Z".to_string(),
    };

    let ctl = 35.0;
    let weeks_until_race = 20; // ~5 months out

    // Build context
    let context = build_macrocycle_context(&profile, &race_goal, ctl, weeks_until_race, None);
    let system_prompt = coach_jan_system_prompt();
    let tool = generate_macrocycle_skeleton_tool();

    // Send request to Claude
    let response = client
        .send(
            Model::Haiku, // Use Haiku for speed and cost
            Some(&system_prompt),
            vec![coachjan::ai::client::Message::user(&context)],
            vec![tool],
            4096,
        )
        .await;

    match response {
        Ok(resp) => {
            eprintln!("Response model: {}", resp.model);
            eprintln!("Stop reason: {}", resp.stop_reason);
            eprintln!(
                "Usage: {} input, {} output tokens",
                resp.usage.input_tokens, resp.usage.output_tokens
            );

            assert_eq!(
                resp.stop_reason, "tool_use",
                "Expected Claude to use the tool"
            );

            // Find the tool_use block
            let tool_use = resp.content.iter().find_map(|block| match block {
                ContentBlock::ToolUse { name, input, .. } => {
                    if name == "generate_macrocycle_skeleton" {
                        Some(input)
                    } else {
                        None
                    }
                }
                _ => None,
            });

            let input = tool_use.expect("Expected generate_macrocycle_skeleton tool_use");

            // Verify basic structure
            assert!(
                input.get("target_ctl").is_some(),
                "Skeleton should have target_ctl"
            );
            assert!(
                input.get("mesocycles").is_some(),
                "Skeleton should have mesocycles"
            );

            let mesocycles = input["mesocycles"].as_array().expect("mesocycles should be array");
            assert!(
                !mesocycles.is_empty(),
                "Should have at least one mesocycle"
            );

            eprintln!("\nGenerated skeleton:");
            eprintln!("  Target CTL: {}", input["target_ctl"]);
            if let Some(msg) = input.get("coach_message") {
                eprintln!("  Coach message: {}", msg);
            }
            for (i, meso) in mesocycles.iter().enumerate() {
                eprintln!(
                    "  Mesocycle {}: phase={}, focus={}, load_weeks={}, recovery_weeks={}, volume={}km",
                    i + 1,
                    meso["phase"],
                    meso["focus"],
                    meso["load_weeks"],
                    meso["recovery_weeks"],
                    meso["target_volume_km"]
                );
            }

            // Verify mesocycle structure
            for meso in mesocycles {
                assert!(meso.get("sequence_number").is_some(), "Missing sequence_number");
                assert!(meso.get("phase").is_some(), "Missing phase");
                assert!(meso.get("focus").is_some(), "Missing focus");
                assert!(meso.get("load_weeks").is_some(), "Missing load_weeks");
                assert!(meso.get("recovery_weeks").is_some(), "Missing recovery_weeks");
                assert!(meso.get("target_volume_km").is_some(), "Missing target_volume_km");
            }

            eprintln!("\nLive test passed!");
        }
        Err(e) => {
            panic!("Claude API request failed: {}", e);
        }
    }
}
