use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use tracing::{info, warn};

use crate::ai::client::{ClaudeClient, ClaudeError, Message, Model};
use crate::ai::context::{build_macrocycle_context, build_mesocycle_context};
use crate::ai::prompts::COACH_JAN_SYSTEM_PROMPT;
use crate::ai::tools::{
    add_coach_notes_tool, generate_macrocycle_skeleton_tool, generate_mesocycle_plan_tool,
};
use crate::db::plans::{
    self, CreateMacrocycle, CreateMesocycle, CreatePlannedWorkout, Macrocycle, Mesocycle,
    PlannedWorkout,
};
use crate::db::profiles::{AthleteProfile, RaceGoal};
use crate::domain::validation::{
    validate_week_plan, PlannedDay, ValidationContext, WeekPlan, WeekType,
};
use crate::domain::workouts::{DurationCategory, WorkoutRegistry, WorkoutType};
use crate::domain::zones::{calculate_hr_zones, calculate_pace_zones};

const DEFAULT_STRENGTH_DURATION_MIN: u16 = 45;
const DEFAULT_STRENGTH_TSS: f64 = 30.0;

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacrocycleSkeleton {
    pub target_ctl: f64,
    pub coach_message: String,
    pub mesocycles: Vec<MesocycleSkeleton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MesocycleSkeleton {
    pub sequence_number: i64,
    pub phase: String,
    pub focus: String,
    pub load_weeks: i64,
    pub recovery_weeks: i64,
    pub target_volume_km: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedPlan {
    pub macrocycle: Macrocycle,
    pub mesocycles: Vec<Mesocycle>,
    pub workouts: Vec<PlannedWorkout>,
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("Claude API error: {0}")]
    Claude(#[from] ClaudeError),
    #[error("Database error: {0}")]
    Database(#[from] crate::error::AppError),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Validation failed after retries: {0}")]
    ValidationFailed(String),
}

// ---------------------------------------------------------------------------
// Intermediate parse types (from Claude's mesocycle plan response)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ClaudeMesocyclePlan {
    mesocycle_overview: String,
    weeks: Vec<ClaudeWeek>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ClaudeWeek {
    week_number: i64,
    week_type: String,
    target_volume_km: f64,
    target_weekly_tss: f64,
    days: Vec<ClaudeDay>,
}

#[derive(Debug, Clone, Deserialize)]
struct ClaudeDay {
    date: String,
    workout_type: String,
    duration_category: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ClaudeCoachNotes {
    workout_notes: Vec<ClaudeWorkoutNote>,
    #[allow(dead_code)]
    mesocycle_overview: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ClaudeWorkoutNote {
    date: String,
    coach_note: String,
}

// ---------------------------------------------------------------------------
// Orchestration: generate_skeleton
// ---------------------------------------------------------------------------

/// Call Claude to generate a macrocycle skeleton (high-level periodization).
pub async fn generate_skeleton(
    client: &ClaudeClient,
    profile: &AthleteProfile,
    race_goal: &RaceGoal,
    ctl: f64,
) -> Result<MacrocycleSkeleton, PlanError> {
    let today = chrono::Utc::now().date_naive();
    let race_date = NaiveDate::parse_from_str(&race_goal.race_date, "%Y-%m-%d").map_err(|e| {
        PlanError::InvalidResponse(format!("Invalid race date '{}': {}", race_goal.race_date, e))
    })?;
    let weeks_until_race = (race_date - today).num_weeks();

    let context = build_macrocycle_context(profile, race_goal, ctl, weeks_until_race);
    let messages = vec![Message::user(&context)];
    let tools = vec![generate_macrocycle_skeleton_tool()];

    info!(
        "Generating macrocycle skeleton for user_id={}, race_date={}, weeks_until_race={}",
        profile.user_id, race_goal.race_date, weeks_until_race
    );

    let response = client
        .send(
            Model::Sonnet,
            Some(COACH_JAN_SYSTEM_PROMPT),
            messages,
            tools,
            4096,
        )
        .await?;

    let (_id, name, input) = response.tool_use().ok_or_else(|| {
        PlanError::InvalidResponse("No tool_use in Claude response".to_string())
    })?;

    if name != "generate_macrocycle_skeleton" {
        return Err(PlanError::InvalidResponse(format!(
            "Expected generate_macrocycle_skeleton tool, got {}",
            name
        )));
    }

    let skeleton: MacrocycleSkeleton = serde_json::from_value(input.clone()).map_err(|e| {
        PlanError::InvalidResponse(format!("Failed to parse skeleton: {}", e))
    })?;

    info!(
        "Generated skeleton with {} mesocycles, target_ctl={}",
        skeleton.mesocycles.len(),
        skeleton.target_ctl
    );

    Ok(skeleton)
}

// ---------------------------------------------------------------------------
// Orchestration: confirm_and_generate_plan
// ---------------------------------------------------------------------------

/// Persist skeleton to DB, generate first mesocycle workouts via Claude,
/// add coach notes, validate, and persist workouts.
pub async fn confirm_and_generate_plan(
    client: &ClaudeClient,
    pool: &SqlitePool,
    user_id: i64,
    skeleton: &MacrocycleSkeleton,
    profile: &AthleteProfile,
    race_goal: &RaceGoal,
    ctl: f64,
) -> Result<GeneratedPlan, PlanError> {
    // --- Step 1: Persist macrocycle + mesocycles to DB ---
    let today = chrono::Utc::now().date_naive();

    // Calculate mesocycle dates
    let meso_dates = calculate_mesocycle_dates(today, &skeleton.mesocycles);

    // Overall end date is the last mesocycle's end date
    let end_date = meso_dates
        .last()
        .map(|(_, end)| end.to_string())
        .unwrap_or_else(|| today.to_string());

    let macrocycle = plans::create_macrocycle(
        pool,
        &CreateMacrocycle {
            user_id,
            race_goal_id: race_goal.id,
            start_date: today.to_string(),
            end_date,
            target_ctl: Some(skeleton.target_ctl),
            coach_message: Some(skeleton.coach_message.clone()),
        },
    )
    .await?;

    info!("Created macrocycle id={}", macrocycle.id);

    let mut db_mesocycles = Vec::new();
    for (i, meso_skel) in skeleton.mesocycles.iter().enumerate() {
        let (start, end) = meso_dates[i];
        let db_meso = plans::create_mesocycle(
            pool,
            &CreateMesocycle {
                macrocycle_id: macrocycle.id,
                sequence_number: meso_skel.sequence_number,
                phase: meso_skel.phase.clone(),
                focus: meso_skel.focus.clone(),
                load_weeks: meso_skel.load_weeks,
                recovery_weeks: meso_skel.recovery_weeks,
                target_volume_km: Some(meso_skel.target_volume_km),
                start_date: start.to_string(),
                end_date: end.to_string(),
            },
        )
        .await?;
        db_mesocycles.push(db_meso);
    }

    info!(
        "Created {} mesocycles for macrocycle id={}",
        db_mesocycles.len(),
        macrocycle.id
    );

    // --- Step 2: Generate first mesocycle day-by-day plan ---
    let first_meso = &db_mesocycles[0];
    let first_skel = &skeleton.mesocycles[0];

    let mesocycle_plan = generate_mesocycle_workouts(
        client,
        profile,
        &first_meso.phase,
        &first_meso.focus,
        first_skel.load_weeks,
        first_skel.recovery_weeks,
        &first_meso.start_date,
        &first_meso.end_date,
        first_skel.target_volume_km,
        ctl,
    )
    .await?;

    // --- Step 3: Fill details from workout registry ---
    let hr_zones = calculate_hr_zones(profile.lthr as u16);
    let pace_zones = profile
        .ftpace_m_per_s
        .map(calculate_pace_zones);

    let filled_workouts = fill_workouts_from_registry(
        &mesocycle_plan.weeks,
        &hr_zones,
        pace_zones.as_ref(),
    )?;

    // --- Step 4: Add coach notes ---
    let coach_notes = generate_coach_notes(
        client,
        profile,
        &first_meso.phase,
        &mesocycle_plan,
    )
    .await?;

    // Build a lookup map date -> coach_note
    let notes_map: std::collections::HashMap<String, String> = coach_notes
        .workout_notes
        .into_iter()
        .map(|n| (n.date, n.coach_note))
        .collect();

    // --- Step 5: Validate ---
    let week_plans = build_week_plans(&mesocycle_plan.weeks, &filled_workouts);
    let validation_ctx = ValidationContext {
        athlete_ctl: ctl,
        previous_week_volume_km: None,
    };

    let mut all_errors = Vec::new();
    for week in &week_plans {
        let errors = validate_week_plan(week, &validation_ctx);
        all_errors.extend(errors);
    }

    // --- Step 6: Retry on validation failure (max 2 retries) ---
    // For now, we log warnings but persist anyway if validation errors are soft.
    // Hard failures (e.g., too many intensity sessions) get retried by calling
    // Claude again with feedback about what was wrong.
    if !all_errors.is_empty() {
        warn!(
            "Validation found {} issues in first mesocycle plan: {:?}",
            all_errors.len(),
            all_errors
        );

        // Attempt retries for severe issues
        let severe_count = all_errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    crate::domain::validation::ValidationError::TooManyIntensitySessions { .. }
                        | crate::domain::validation::ValidationError::NoRestDay
                )
            })
            .count();

        if severe_count > 0 {
            info!(
                "Attempting retry due to {} severe validation errors",
                severe_count
            );

            // Retry up to 2 times
            let mut retry_result = None;
            for attempt in 1..=2 {
                info!("Retry attempt {}/2", attempt);

                let retry_plan = generate_mesocycle_workouts(
                    client,
                    profile,
                    &first_meso.phase,
                    &first_meso.focus,
                    first_skel.load_weeks,
                    first_skel.recovery_weeks,
                    &first_meso.start_date,
                    &first_meso.end_date,
                    first_skel.target_volume_km,
                    ctl,
                )
                .await?;

                let retry_filled = fill_workouts_from_registry(
                    &retry_plan.weeks,
                    &hr_zones,
                    pace_zones.as_ref(),
                )?;

                let retry_week_plans = build_week_plans(&retry_plan.weeks, &retry_filled);
                let mut retry_errors = Vec::new();
                for week in &retry_week_plans {
                    retry_errors.extend(validate_week_plan(week, &validation_ctx));
                }

                let retry_severe = retry_errors
                    .iter()
                    .filter(|e| {
                        matches!(
                            e,
                            crate::domain::validation::ValidationError::TooManyIntensitySessions {
                                ..
                            } | crate::domain::validation::ValidationError::NoRestDay
                        )
                    })
                    .count();

                if retry_severe == 0 {
                    info!("Retry attempt {} resolved severe validation errors", attempt);
                    retry_result = Some((retry_plan, retry_filled));
                    break;
                }

                warn!(
                    "Retry attempt {} still has {} severe errors",
                    attempt, retry_severe
                );
            }

            if let Some((retry_plan, retry_filled)) = retry_result {
                // Use retried results, regenerate coach notes
                let retry_notes = generate_coach_notes(
                    client,
                    profile,
                    &first_meso.phase,
                    &retry_plan,
                )
                .await?;

                let retry_notes_map: std::collections::HashMap<String, String> = retry_notes
                    .workout_notes
                    .into_iter()
                    .map(|n| (n.date, n.coach_note))
                    .collect();

                // Persist with retried data
                let workouts = persist_workouts(
                    pool,
                    first_meso.id,
                    user_id,
                    &retry_plan.weeks,
                    &retry_filled,
                    &retry_notes_map,
                )
                .await?;

                return Ok(GeneratedPlan {
                    macrocycle,
                    mesocycles: db_mesocycles,
                    workouts,
                });
            }

            // If retries didn't resolve, fail the operation
            let error_descriptions: Vec<String> =
                all_errors.iter().map(|e| format!("{:?}", e)).collect();
            return Err(PlanError::ValidationFailed(format!(
                "Plan has severe validation errors after 2 retries: {}",
                error_descriptions.join("; ")
            )));
        }
    }

    // --- Step 7: Persist planned workouts to DB ---
    let workouts = persist_workouts(
        pool,
        first_meso.id,
        user_id,
        &mesocycle_plan.weeks,
        &filled_workouts,
        &notes_map,
    )
    .await?;

    info!(
        "Persisted {} planned workouts for mesocycle id={}",
        workouts.len(),
        first_meso.id
    );

    Ok(GeneratedPlan {
        macrocycle,
        mesocycles: db_mesocycles,
        workouts,
    })
}

// ---------------------------------------------------------------------------
// Helper: calculate mesocycle dates from skeleton
// ---------------------------------------------------------------------------

pub fn calculate_mesocycle_dates(
    start: NaiveDate,
    mesocycles: &[MesocycleSkeleton],
) -> Vec<(NaiveDate, NaiveDate)> {
    let mut dates = Vec::new();
    let mut cursor = start;
    for meso in mesocycles {
        let total_weeks = meso.load_weeks + meso.recovery_weeks;
        let end = cursor + chrono::Duration::weeks(total_weeks);
        // end_date is the last day of this mesocycle (day before next starts)
        dates.push((cursor, end - chrono::Duration::days(1)));
        cursor = end;
    }
    dates
}

// ---------------------------------------------------------------------------
// Helper: call Claude to generate mesocycle workouts
// ---------------------------------------------------------------------------

async fn generate_mesocycle_workouts(
    client: &ClaudeClient,
    profile: &AthleteProfile,
    phase: &str,
    focus: &str,
    load_weeks: i64,
    recovery_weeks: i64,
    start_date: &str,
    end_date: &str,
    target_volume_km: f64,
    ctl: f64,
) -> Result<ClaudeMesocyclePlan, PlanError> {
    let context = build_mesocycle_context(
        profile,
        phase,
        focus,
        load_weeks,
        recovery_weeks,
        start_date,
        end_date,
        target_volume_km,
        ctl,
    );

    let messages = vec![Message::user(&context)];
    let tools = vec![generate_mesocycle_plan_tool()];

    let response = client
        .send(
            Model::Sonnet,
            Some(COACH_JAN_SYSTEM_PROMPT),
            messages,
            tools,
            8192,
        )
        .await?;

    let (_id, name, input) = response.tool_use().ok_or_else(|| {
        PlanError::InvalidResponse("No tool_use in mesocycle response".to_string())
    })?;

    if name != "generate_mesocycle_plan" {
        return Err(PlanError::InvalidResponse(format!(
            "Expected generate_mesocycle_plan tool, got {}",
            name
        )));
    }

    let plan: ClaudeMesocyclePlan = serde_json::from_value(input.clone()).map_err(|e| {
        PlanError::InvalidResponse(format!("Failed to parse mesocycle plan: {}", e))
    })?;

    Ok(plan)
}

// ---------------------------------------------------------------------------
// Helper: call Claude to generate coach notes
// ---------------------------------------------------------------------------

async fn generate_coach_notes(
    client: &ClaudeClient,
    profile: &AthleteProfile,
    phase: &str,
    plan: &ClaudeMesocyclePlan,
) -> Result<ClaudeCoachNotes, PlanError> {
    // Build a summary of the workouts for Claude to add notes to
    let mut workout_summary = String::new();
    for week in &plan.weeks {
        workout_summary.push_str(&format!(
            "\nWeek {} ({}):\n",
            week.week_number, week.week_type
        ));
        for day in &week.days {
            workout_summary.push_str(&format!(
                "  {} - {} ({})\n",
                day.date,
                day.workout_type,
                day.duration_category.as_deref().unwrap_or("n/a")
            ));
        }
    }

    let prompt = format!(
        r#"Add personalized coaching notes for this {} phase mesocycle plan.

Athlete: {} ({} level, CTL ~{})

Mesocycle overview: {}

Workout schedule:
{}

For each workout, provide a brief coaching note (1-2 sentences) explaining purpose,
key execution cues, or what to focus on. Use "we" language."#,
        phase,
        profile.name,
        profile.experience_level,
        profile.current_weekly_volume_km,
        plan.mesocycle_overview,
        workout_summary,
    );

    let messages = vec![Message::user(&prompt)];
    let tools = vec![add_coach_notes_tool()];

    let response = client
        .send(
            Model::Sonnet,
            Some(COACH_JAN_SYSTEM_PROMPT),
            messages,
            tools,
            8192,
        )
        .await?;

    let (_id, name, input) = response.tool_use().ok_or_else(|| {
        PlanError::InvalidResponse("No tool_use in coach notes response".to_string())
    })?;

    if name != "add_coach_notes" {
        return Err(PlanError::InvalidResponse(format!(
            "Expected add_coach_notes tool, got {}",
            name
        )));
    }

    let notes: ClaudeCoachNotes = serde_json::from_value(input.clone()).map_err(|e| {
        PlanError::InvalidResponse(format!("Failed to parse coach notes: {}", e))
    })?;

    Ok(notes)
}

// ---------------------------------------------------------------------------
// Helper: fill workout details from registry
// ---------------------------------------------------------------------------

/// A resolved workout assignment for a single day.
#[derive(Debug, Clone)]
pub struct FilledWorkout {
    pub date: String,
    pub workout_type: WorkoutType,
    pub duration_category: Option<DurationCategory>,
    pub duration_min: Option<u16>,
    pub structure: Option<String>,
    pub description: Option<String>,
    pub target_hr_zones: Vec<u8>,
    pub target_pace_zones: Vec<u8>,
    pub hr_zone_display: Option<String>,
    pub expected_tss: f64,
}

pub(crate) fn fill_workouts_from_registry(
    weeks: &[ClaudeWeek],
    hr_zones: &crate::domain::types::HrZones,
    pace_zones: Option<&crate::domain::types::PaceZones>,
) -> Result<Vec<FilledWorkout>, PlanError> {
    let registry = WorkoutRegistry::new();
    let mut filled = Vec::new();

    for week in weeks {
        for day in &week.days {
            let wt = WorkoutType::from_str(&day.workout_type).ok_or_else(|| {
                PlanError::InvalidResponse(format!(
                    "Unknown workout type: {}",
                    day.workout_type
                ))
            })?;

            if wt == WorkoutType::Rest {
                filled.push(FilledWorkout {
                    date: day.date.clone(),
                    workout_type: WorkoutType::Rest,
                    duration_category: None,
                    duration_min: None,
                    structure: None,
                    description: Some("Rest day".to_string()),
                    target_hr_zones: vec![],
                    target_pace_zones: vec![],
                    hr_zone_display: None,
                    expected_tss: 0.0,
                });
                continue;
            }

            // Strength types don't have registry entries
            if matches!(
                wt,
                WorkoutType::StrengthPrecision
                    | WorkoutType::StrengthPerformance
                    | WorkoutType::StrengthPower
            ) {
                filled.push(FilledWorkout {
                    date: day.date.clone(),
                    workout_type: wt,
                    duration_category: None,
                    duration_min: Some(DEFAULT_STRENGTH_DURATION_MIN),
                    structure: Some(format!("{} session", wt.display_name())),
                    description: Some(wt.display_name().to_string()),
                    target_hr_zones: vec![],
                    target_pace_zones: vec![],
                    hr_zone_display: None,
                    expected_tss: DEFAULT_STRENGTH_TSS,
                });
                continue;
            }

            let duration_cat = parse_duration_category(day.duration_category.as_deref());

            let resolved = registry
                .resolve(&wt, &duration_cat, hr_zones, pace_zones)
                .ok_or_else(|| {
                    PlanError::InvalidResponse(format!(
                        "No template for {:?}/{:?}",
                        wt, duration_cat
                    ))
                })?;

            filled.push(FilledWorkout {
                date: day.date.clone(),
                workout_type: wt,
                duration_category: Some(duration_cat),
                duration_min: Some(resolved.duration_min),
                structure: Some(resolved.structure),
                description: Some(resolved.description),
                target_hr_zones: resolved.target_hr_zones,
                target_pace_zones: resolved.target_pace_zones,
                hr_zone_display: Some(resolved.hr_zone_display),
                expected_tss: resolved.expected_tss,
            });
        }
    }

    Ok(filled)
}

/// Parse a duration category string, defaulting to Medium.
pub fn parse_duration_category(s: Option<&str>) -> DurationCategory {
    match s {
        Some("short") => DurationCategory::Short,
        Some("medium") => DurationCategory::Medium,
        Some("long") => DurationCategory::Long,
        _ => DurationCategory::Medium,
    }
}

// ---------------------------------------------------------------------------
// Helper: build WeekPlan structs for validation
// ---------------------------------------------------------------------------

fn build_week_plans(weeks: &[ClaudeWeek], filled: &[FilledWorkout]) -> Vec<WeekPlan> {
    let mut result = Vec::new();
    let mut filled_idx = 0;

    for week in weeks {
        let mut days = Vec::new();
        for _day in &week.days {
            if filled_idx < filled.len() {
                let f = &filled[filled_idx];
                days.push(PlannedDay {
                    date: f.date.clone(),
                    workout_type: f.workout_type,
                    duration_category: f.duration_category,
                    expected_tss: f.expected_tss,
                });
                filled_idx += 1;
            }
        }

        let week_type = match week.week_type.as_str() {
            "recovery" => WeekType::Recovery,
            _ => WeekType::Load,
        };

        result.push(WeekPlan {
            week_number: week.week_number as u32,
            week_type,
            target_volume_km: week.target_volume_km,
            target_weekly_tss: week.target_weekly_tss,
            days,
        });
    }

    result
}

// ---------------------------------------------------------------------------
// Helper: persist workouts to DB
// ---------------------------------------------------------------------------

async fn persist_workouts(
    pool: &SqlitePool,
    mesocycle_id: i64,
    user_id: i64,
    weeks: &[ClaudeWeek],
    filled: &[FilledWorkout],
    notes_map: &std::collections::HashMap<String, String>,
) -> Result<Vec<PlannedWorkout>, PlanError> {
    let mut persisted = Vec::new();
    let mut filled_idx = 0;

    for week in weeks {
        for _day in &week.days {
            if filled_idx >= filled.len() {
                break;
            }
            let f = &filled[filled_idx];
            filled_idx += 1;

            let coach_notes = notes_map.get(&f.date).cloned();

            let hr_zones_str = if f.target_hr_zones.is_empty() {
                None
            } else {
                Some(
                    f.target_hr_zones
                        .iter()
                        .map(|z| format!("Z{}", z))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            };

            let pace_zones_str = if f.target_pace_zones.is_empty() {
                None
            } else {
                Some(
                    f.target_pace_zones
                        .iter()
                        .map(|z| format!("Z{}", z))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            };

            let workout = plans::create_planned_workout(
                pool,
                &CreatePlannedWorkout {
                    mesocycle_id,
                    user_id,
                    scheduled_date: f.date.clone(),
                    workout_type: f.workout_type.as_str().to_string(),
                    duration_min: f.duration_min.map(|d| d as i64),
                    duration_category: f
                        .duration_category
                        .as_ref()
                        .map(|dc| dc.as_str().to_string()),
                    target_hr_zones: hr_zones_str,
                    target_pace_zones: pace_zones_str,
                    expected_tss: Some(f.expected_tss),
                    description: f.description.clone(),
                    coach_notes,
                },
            )
            .await?;

            persisted.push(workout);
        }
    }

    Ok(persisted)
}

// ---------------------------------------------------------------------------
// Helper: parse skeleton from a tool_use response value
// ---------------------------------------------------------------------------

pub fn parse_skeleton(input: &Value) -> Result<MacrocycleSkeleton, PlanError> {
    serde_json::from_value(input.clone())
        .map_err(|e| PlanError::InvalidResponse(format!("Failed to parse skeleton: {}", e)))
}

/// Parse a mesocycle plan from a tool_use response value.
#[allow(dead_code)] // Used in tests; will be used by API endpoints in Task 9
pub(crate) fn parse_mesocycle_plan(input: &Value) -> Result<ClaudeMesocyclePlan, PlanError> {
    serde_json::from_value(input.clone()).map_err(|e| {
        PlanError::InvalidResponse(format!("Failed to parse mesocycle plan: {}", e))
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::zones::calculate_hr_zones;
    use serde_json::json;

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    fn sample_skeleton_json() -> Value {
        json!({
            "target_ctl": 65.0,
            "coach_message": "We're going to build your aerobic engine first, then sharpen for race day.",
            "mesocycles": [
                {
                    "sequence_number": 1,
                    "phase": "capacity",
                    "focus": "aerobic_capacity",
                    "load_weeks": 3,
                    "recovery_weeks": 1,
                    "target_volume_km": 45.0
                },
                {
                    "sequence_number": 2,
                    "phase": "utilization",
                    "focus": "aerobic_utilization",
                    "load_weeks": 3,
                    "recovery_weeks": 1,
                    "target_volume_km": 50.0
                },
                {
                    "sequence_number": 3,
                    "phase": "taper",
                    "focus": "race_specific",
                    "load_weeks": 1,
                    "recovery_weeks": 1,
                    "target_volume_km": 30.0
                }
            ]
        })
    }

    fn sample_mesocycle_plan_json() -> Value {
        json!({
            "mesocycle_overview": "Building aerobic capacity with progressive volume.",
            "weeks": [
                {
                    "week_number": 1,
                    "week_type": "load",
                    "target_volume_km": 40.0,
                    "target_weekly_tss": 250.0,
                    "days": [
                        { "date": "2026-03-02", "workout_type": "easy_run", "duration_category": "medium" },
                        { "date": "2026-03-03", "workout_type": "vo2max_intervals", "duration_category": "short" },
                        { "date": "2026-03-04", "workout_type": "rest" },
                        { "date": "2026-03-05", "workout_type": "easy_run", "duration_category": "short" },
                        { "date": "2026-03-06", "workout_type": "anaerobic_hills", "duration_category": "medium" },
                        { "date": "2026-03-07", "workout_type": "recovery_run", "duration_category": "short" },
                        { "date": "2026-03-08", "workout_type": "long_run", "duration_category": "medium" }
                    ]
                }
            ]
        })
    }

    // -----------------------------------------------------------------------
    // Parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn parse_skeleton_from_json() {
        let input = sample_skeleton_json();
        let skeleton = parse_skeleton(&input).unwrap();

        assert_eq!(skeleton.target_ctl, 65.0);
        assert!(skeleton.coach_message.contains("aerobic engine"));
        assert_eq!(skeleton.mesocycles.len(), 3);
        assert_eq!(skeleton.mesocycles[0].phase, "capacity");
        assert_eq!(skeleton.mesocycles[0].focus, "aerobic_capacity");
        assert_eq!(skeleton.mesocycles[0].load_weeks, 3);
        assert_eq!(skeleton.mesocycles[0].recovery_weeks, 1);
        assert_eq!(skeleton.mesocycles[0].target_volume_km, 45.0);
        assert_eq!(skeleton.mesocycles[2].phase, "taper");
    }

    #[test]
    fn parse_skeleton_missing_field_fails() {
        let input = json!({
            "target_ctl": 65.0,
            // missing coach_message and mesocycles
        });
        let result = parse_skeleton(&input);
        assert!(result.is_err());
        match result.unwrap_err() {
            PlanError::InvalidResponse(msg) => {
                assert!(msg.contains("Failed to parse skeleton"));
            }
            other => panic!("Expected InvalidResponse, got: {:?}", other),
        }
    }

    #[test]
    fn parse_mesocycle_plan_from_json() {
        let input = sample_mesocycle_plan_json();
        let plan = parse_mesocycle_plan(&input).unwrap();

        assert_eq!(plan.mesocycle_overview, "Building aerobic capacity with progressive volume.");
        assert_eq!(plan.weeks.len(), 1);
        assert_eq!(plan.weeks[0].week_number, 1);
        assert_eq!(plan.weeks[0].week_type, "load");
        assert_eq!(plan.weeks[0].days.len(), 7);
        assert_eq!(plan.weeks[0].days[0].workout_type, "easy_run");
        assert_eq!(
            plan.weeks[0].days[0].duration_category.as_deref(),
            Some("medium")
        );
        assert_eq!(plan.weeks[0].days[2].workout_type, "rest");
        assert!(plan.weeks[0].days[2].duration_category.is_none());
    }

    #[test]
    fn parse_mesocycle_plan_invalid_json_fails() {
        let input = json!({ "invalid": true });
        let result = parse_mesocycle_plan(&input);
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Duration category parsing
    // -----------------------------------------------------------------------

    #[test]
    fn parse_duration_category_known_values() {
        assert_eq!(parse_duration_category(Some("short")), DurationCategory::Short);
        assert_eq!(parse_duration_category(Some("medium")), DurationCategory::Medium);
        assert_eq!(parse_duration_category(Some("long")), DurationCategory::Long);
    }

    #[test]
    fn parse_duration_category_defaults_to_medium() {
        assert_eq!(parse_duration_category(None), DurationCategory::Medium);
        assert_eq!(parse_duration_category(Some("invalid")), DurationCategory::Medium);
        assert_eq!(parse_duration_category(Some("")), DurationCategory::Medium);
    }

    // -----------------------------------------------------------------------
    // Fill from registry tests
    // -----------------------------------------------------------------------

    #[test]
    fn fill_workouts_easy_run() {
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-02".to_string(),
                workout_type: "easy_run".to_string(),
                duration_category: Some("medium".to_string()),
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&weeks, &hr_zones, None).unwrap();

        assert_eq!(filled.len(), 1);
        assert_eq!(filled[0].workout_type, WorkoutType::EasyRun);
        assert_eq!(filled[0].duration_category, Some(DurationCategory::Medium));
        assert_eq!(filled[0].duration_min, Some(45)); // Easy run medium = 45 min
        assert!(filled[0].expected_tss > 0.0);
        assert!(filled[0].structure.as_ref().unwrap().contains("Zone 1-2"));
        assert!(filled[0].hr_zone_display.is_some());
    }

    #[test]
    fn fill_workouts_rest_day() {
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-04".to_string(),
                workout_type: "rest".to_string(),
                duration_category: None,
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&weeks, &hr_zones, None).unwrap();

        assert_eq!(filled.len(), 1);
        assert_eq!(filled[0].workout_type, WorkoutType::Rest);
        assert_eq!(filled[0].duration_min, None);
        assert_eq!(filled[0].expected_tss, 0.0);
        assert!(filled[0].target_hr_zones.is_empty());
    }

    #[test]
    fn fill_workouts_strength_type() {
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-05".to_string(),
                workout_type: "strength_precision".to_string(),
                duration_category: None,
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&weeks, &hr_zones, None).unwrap();

        assert_eq!(filled.len(), 1);
        assert_eq!(filled[0].workout_type, WorkoutType::StrengthPrecision);
        assert_eq!(filled[0].duration_min, Some(45));
        assert_eq!(filled[0].expected_tss, 30.0);
    }

    #[test]
    fn fill_workouts_unknown_type_fails() {
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-02".to_string(),
                workout_type: "nonexistent_workout".to_string(),
                duration_category: None,
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let result = fill_workouts_from_registry(&weeks, &hr_zones, None);
        assert!(result.is_err());
        match result.unwrap_err() {
            PlanError::InvalidResponse(msg) => {
                assert!(msg.contains("Unknown workout type: nonexistent_workout"));
            }
            other => panic!("Expected InvalidResponse, got: {:?}", other),
        }
    }

    #[test]
    fn fill_workouts_full_week() {
        let plan_json = sample_mesocycle_plan_json();
        let plan: ClaudeMesocyclePlan = serde_json::from_value(plan_json).unwrap();

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&plan.weeks, &hr_zones, None).unwrap();

        assert_eq!(filled.len(), 7);
        // Check specific types
        assert_eq!(filled[0].workout_type, WorkoutType::EasyRun);
        assert_eq!(filled[1].workout_type, WorkoutType::Vo2maxIntervals);
        assert_eq!(filled[2].workout_type, WorkoutType::Rest);
        assert_eq!(filled[3].workout_type, WorkoutType::EasyRun);
        assert_eq!(filled[4].workout_type, WorkoutType::AnaerobicHills);
        assert_eq!(filled[5].workout_type, WorkoutType::RecoveryRun);
        assert_eq!(filled[6].workout_type, WorkoutType::LongRun);

        // Rest has 0 TSS, others have positive
        assert_eq!(filled[2].expected_tss, 0.0);
        assert!(filled[0].expected_tss > 0.0);
        assert!(filled[1].expected_tss > 0.0);
        assert!(filled[6].expected_tss > 0.0);
    }

    #[test]
    fn fill_workouts_default_duration_category() {
        // When duration_category is missing for a running workout, default to Medium
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-02".to_string(),
                workout_type: "easy_run".to_string(),
                duration_category: None, // missing
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&weeks, &hr_zones, None).unwrap();

        assert_eq!(filled[0].duration_category, Some(DurationCategory::Medium));
        assert_eq!(filled[0].duration_min, Some(45)); // medium easy_run
    }

    // -----------------------------------------------------------------------
    // Mesocycle date calculation tests
    // -----------------------------------------------------------------------

    #[test]
    fn mesocycle_dates_single() {
        let start = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let mesocycles = vec![MesocycleSkeleton {
            sequence_number: 1,
            phase: "capacity".to_string(),
            focus: "aerobic_capacity".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: 45.0,
        }];

        let dates = calculate_mesocycle_dates(start, &mesocycles);
        assert_eq!(dates.len(), 1);
        assert_eq!(dates[0].0, NaiveDate::from_ymd_opt(2026, 3, 1).unwrap());
        // 4 weeks from March 1 = March 29, end = March 28
        assert_eq!(dates[0].1, NaiveDate::from_ymd_opt(2026, 3, 28).unwrap());
    }

    #[test]
    fn mesocycle_dates_multiple() {
        let start = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let mesocycles = vec![
            MesocycleSkeleton {
                sequence_number: 1,
                phase: "capacity".to_string(),
                focus: "aerobic_capacity".to_string(),
                load_weeks: 3,
                recovery_weeks: 1,
                target_volume_km: 45.0,
            },
            MesocycleSkeleton {
                sequence_number: 2,
                phase: "utilization".to_string(),
                focus: "aerobic_utilization".to_string(),
                load_weeks: 3,
                recovery_weeks: 1,
                target_volume_km: 50.0,
            },
            MesocycleSkeleton {
                sequence_number: 3,
                phase: "taper".to_string(),
                focus: "race_specific".to_string(),
                load_weeks: 1,
                recovery_weeks: 1,
                target_volume_km: 30.0,
            },
        ];

        let dates = calculate_mesocycle_dates(start, &mesocycles);
        assert_eq!(dates.len(), 3);

        // Meso 1: Mar 1 - Mar 28 (4 weeks)
        assert_eq!(dates[0].0, NaiveDate::from_ymd_opt(2026, 3, 1).unwrap());
        assert_eq!(dates[0].1, NaiveDate::from_ymd_opt(2026, 3, 28).unwrap());

        // Meso 2: Mar 29 - Apr 25 (4 weeks)
        assert_eq!(dates[1].0, NaiveDate::from_ymd_opt(2026, 3, 29).unwrap());
        assert_eq!(dates[1].1, NaiveDate::from_ymd_opt(2026, 4, 25).unwrap());

        // Meso 3: Apr 26 - May 9 (2 weeks)
        assert_eq!(dates[2].0, NaiveDate::from_ymd_opt(2026, 4, 26).unwrap());
        assert_eq!(dates[2].1, NaiveDate::from_ymd_opt(2026, 5, 9).unwrap());
    }

    #[test]
    fn mesocycle_dates_contiguous() {
        let start = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let mesocycles = vec![
            MesocycleSkeleton {
                sequence_number: 1,
                phase: "capacity".to_string(),
                focus: "aerobic_capacity".to_string(),
                load_weeks: 2,
                recovery_weeks: 1,
                target_volume_km: 40.0,
            },
            MesocycleSkeleton {
                sequence_number: 2,
                phase: "utilization".to_string(),
                focus: "aerobic_utilization".to_string(),
                load_weeks: 2,
                recovery_weeks: 1,
                target_volume_km: 45.0,
            },
        ];

        let dates = calculate_mesocycle_dates(start, &mesocycles);
        // End of meso 1 + 1 day should equal start of meso 2
        let meso1_end = dates[0].1;
        let meso2_start = dates[1].0;
        assert_eq!(
            meso1_end + chrono::Duration::days(1),
            meso2_start,
            "Mesocycles should be contiguous"
        );
    }

    // -----------------------------------------------------------------------
    // Week plan building + validation tests
    // -----------------------------------------------------------------------

    #[test]
    fn build_week_plans_from_filled() {
        let plan_json = sample_mesocycle_plan_json();
        let plan: ClaudeMesocyclePlan = serde_json::from_value(plan_json).unwrap();
        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&plan.weeks, &hr_zones, None).unwrap();
        let week_plans = build_week_plans(&plan.weeks, &filled);

        assert_eq!(week_plans.len(), 1);
        assert_eq!(week_plans[0].week_number, 1);
        assert_eq!(week_plans[0].week_type, WeekType::Load);
        assert_eq!(week_plans[0].days.len(), 7);
        assert_eq!(week_plans[0].target_volume_km, 40.0);
        assert_eq!(week_plans[0].target_weekly_tss, 250.0);
    }

    #[test]
    fn valid_sample_week_passes_validation() {
        let plan_json = sample_mesocycle_plan_json();
        let plan: ClaudeMesocyclePlan = serde_json::from_value(plan_json).unwrap();
        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&plan.weeks, &hr_zones, None).unwrap();
        let week_plans = build_week_plans(&plan.weeks, &filled);

        let ctx = ValidationContext {
            athlete_ctl: 35.0,
            previous_week_volume_km: None,
        };

        for week in &week_plans {
            let errors = validate_week_plan(week, &ctx);
            assert!(errors.is_empty(), "Expected no validation errors, got: {:?}", errors);
        }
    }

    // -----------------------------------------------------------------------
    // PlanError display tests
    // -----------------------------------------------------------------------

    #[test]
    fn plan_error_display() {
        let err = PlanError::InvalidResponse("bad json".to_string());
        assert_eq!(format!("{}", err), "Invalid response: bad json");

        let err = PlanError::ValidationFailed("too many intensity".to_string());
        assert_eq!(
            format!("{}", err),
            "Validation failed after retries: too many intensity"
        );
    }

    // -----------------------------------------------------------------------
    // Skeleton serialization roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn skeleton_serde_roundtrip() {
        let skeleton = MacrocycleSkeleton {
            target_ctl: 65.0,
            coach_message: "Build aerobic base".to_string(),
            mesocycles: vec![
                MesocycleSkeleton {
                    sequence_number: 1,
                    phase: "capacity".to_string(),
                    focus: "aerobic_capacity".to_string(),
                    load_weeks: 3,
                    recovery_weeks: 1,
                    target_volume_km: 45.0,
                },
            ],
        };

        let json = serde_json::to_value(&skeleton).unwrap();
        let parsed: MacrocycleSkeleton = serde_json::from_value(json).unwrap();

        assert_eq!(parsed.target_ctl, skeleton.target_ctl);
        assert_eq!(parsed.coach_message, skeleton.coach_message);
        assert_eq!(parsed.mesocycles.len(), 1);
        assert_eq!(parsed.mesocycles[0].phase, "capacity");
    }

    // -----------------------------------------------------------------------
    // Fill workouts with pace zones
    // -----------------------------------------------------------------------

    #[test]
    fn fill_workouts_with_pace_zones() {
        let weeks = vec![ClaudeWeek {
            week_number: 1,
            week_type: "load".to_string(),
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![ClaudeDay {
                date: "2026-03-02".to_string(),
                workout_type: "tempo_run".to_string(),
                duration_category: Some("short".to_string()),
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let pace_zones = calculate_pace_zones(3.5);
        let filled =
            fill_workouts_from_registry(&weeks, &hr_zones, Some(&pace_zones)).unwrap();

        assert_eq!(filled.len(), 1);
        assert_eq!(filled[0].workout_type, WorkoutType::TempoRun);
        assert_eq!(filled[0].duration_category, Some(DurationCategory::Short));
        assert!(!filled[0].target_hr_zones.is_empty());
        assert!(!filled[0].target_pace_zones.is_empty());
        assert!(filled[0].hr_zone_display.is_some());
    }

    // -----------------------------------------------------------------------
    // All running workout types resolve in registry
    // -----------------------------------------------------------------------

    #[test]
    fn all_running_types_resolve_with_all_durations() {
        let hr_zones = calculate_hr_zones(170);
        let registry = WorkoutRegistry::new();

        for wt in WorkoutType::all_running() {
            for dc in [DurationCategory::Short, DurationCategory::Medium, DurationCategory::Long] {
                let result = registry.resolve(&wt, &dc, &hr_zones, None);
                assert!(
                    result.is_some(),
                    "Failed to resolve {:?}/{:?}",
                    wt,
                    dc
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Recovery week type parsing
    // -----------------------------------------------------------------------

    #[test]
    fn recovery_week_type_parsed_correctly() {
        let weeks = vec![ClaudeWeek {
            week_number: 4,
            week_type: "recovery".to_string(),
            target_volume_km: 25.0,
            target_weekly_tss: 150.0,
            days: vec![ClaudeDay {
                date: "2026-03-23".to_string(),
                workout_type: "rest".to_string(),
                duration_category: None,
            }],
        }];

        let hr_zones = calculate_hr_zones(170);
        let filled = fill_workouts_from_registry(&weeks, &hr_zones, None).unwrap();
        let week_plans = build_week_plans(&weeks, &filled);

        assert_eq!(week_plans[0].week_type, WeekType::Recovery);
    }
}
