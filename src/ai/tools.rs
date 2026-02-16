use serde_json::json;
use crate::ai::client::Tool;

/// Tool schema for generating a macrocycle skeleton.
/// Claude uses this to output the high-level periodization plan.
pub fn generate_macrocycle_skeleton_tool() -> Tool {
    Tool {
        name: "generate_macrocycle_skeleton".to_string(),
        description: "Generate a macrocycle skeleton with mesocycle phases, volume targets, and periodization structure based on the athlete's profile and race goal.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "target_ctl": {
                    "type": "number",
                    "description": "Target CTL at peak fitness"
                },
                "coach_message": {
                    "type": "string",
                    "description": "Coach Jan's overview message explaining the plan rationale"
                },
                "mesocycles": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "sequence_number": { "type": "integer" },
                            "phase": {
                                "type": "string",
                                "enum": ["capacity", "utilization", "taper", "recovery", "transition"]
                            },
                            "focus": {
                                "type": "string",
                                "enum": [
                                    "aerobic_capacity",
                                    "aerobic_utilization",
                                    "anaerobic_capacity",
                                    "anaerobic_utilization",
                                    "race_specific",
                                    "recovery"
                                ]
                            },
                            "load_weeks": { "type": "integer" },
                            "recovery_weeks": { "type": "integer" },
                            "target_volume_km": { "type": "number" }
                        },
                        "required": [
                            "sequence_number",
                            "phase",
                            "focus",
                            "load_weeks",
                            "recovery_weeks",
                            "target_volume_km"
                        ]
                    }
                }
            },
            "required": ["target_ctl", "coach_message", "mesocycles"]
        }),
    }
}

/// Tool schema for generating a mesocycle plan with day-by-day workouts.
/// Claude uses this to assign workout types and volumes for each day.
pub fn generate_mesocycle_plan_tool() -> Tool {
    Tool {
        name: "generate_mesocycle_plan".to_string(),
        description: "Generate a detailed mesocycle plan with weekly structure and daily workout assignments.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "mesocycle_overview": {
                    "type": "string",
                    "description": "Coach Jan's overview of this mesocycle's goals and approach"
                },
                "weeks": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "week_number": { "type": "integer" },
                            "week_type": {
                                "type": "string",
                                "enum": ["load", "recovery"]
                            },
                            "target_volume_km": { "type": "number" },
                            "target_weekly_tss": { "type": "number" },
                            "days": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "date": {
                                            "type": "string",
                                            "description": "YYYY-MM-DD"
                                        },
                                        "workout_type": {
                                            "type": "string",
                                            "description": "One of the available workout types"
                                        },
                                        "duration_category": {
                                            "type": "string",
                                            "enum": ["short", "medium", "long"],
                                            "description": "Required for running workouts, omit for rest"
                                        }
                                    },
                                    "required": ["date", "workout_type"]
                                }
                            }
                        },
                        "required": [
                            "week_number",
                            "week_type",
                            "target_volume_km",
                            "target_weekly_tss",
                            "days"
                        ]
                    }
                }
            },
            "required": ["mesocycle_overview", "weeks"]
        }),
    }
}

/// Tool schema for adding personalized coaching notes to workouts.
/// Claude uses this to provide per-day guidance and mesocycle summaries.
pub fn add_coach_notes_tool() -> Tool {
    Tool {
        name: "add_coach_notes".to_string(),
        description: "Add personalized coaching notes for each workout and an overall mesocycle summary.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "workout_notes": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "date": { "type": "string" },
                            "coach_note": { "type": "string" }
                        },
                        "required": ["date", "coach_note"]
                    }
                },
                "mesocycle_overview": {
                    "type": "string",
                    "description": "Coach Jan's summary of the mesocycle training approach"
                }
            },
            "required": ["workout_notes", "mesocycle_overview"]
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_schemas_are_valid_json() {
        let tools = vec![
            generate_macrocycle_skeleton_tool(),
            generate_mesocycle_plan_tool(),
            add_coach_notes_tool(),
        ];
        for tool in &tools {
            // Verify the schema is valid JSON by re-serializing
            let json = serde_json::to_string(&tool.input_schema).unwrap();
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn tool_names_are_correct() {
        assert_eq!(generate_macrocycle_skeleton_tool().name, "generate_macrocycle_skeleton");
        assert_eq!(generate_mesocycle_plan_tool().name, "generate_mesocycle_plan");
        assert_eq!(add_coach_notes_tool().name, "add_coach_notes");
    }

    #[test]
    fn system_prompt_contains_key_terms() {
        use crate::ai::prompts::COACH_JAN_SYSTEM_PROMPT;
        assert!(COACH_JAN_SYSTEM_PROMPT.contains("Olbrecht"));
        assert!(COACH_JAN_SYSTEM_PROMPT.contains("capacity"));
        assert!(COACH_JAN_SYSTEM_PROMPT.contains("aerobically limited"));
        assert!(COACH_JAN_SYSTEM_PROMPT.contains("easy_run"));
    }
}
