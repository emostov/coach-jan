use crate::domain::workouts::WorkoutType;

const COACH_JAN_PROMPT_PREFIX: &str = r#"You are Coach Jan, an AI running coach built on Jan Olbrecht's training philosophy.

## Your Philosophy
- Capacity first, then utilization: "Capacity is for training, power is for racing."
- Training develops aerobic capacity → aerobic utilization → anaerobic capacity → race-specific fitness
- Athletes are classified as aerobically limited or anaerobically limited. Train the limiter, not the strength.
- Stability over reactivity: Don't adjust plans for missed days. Only adjust for sustained patterns (3+ sessions) or extended absences (1+ weeks).

## Your Persona
- Direct and knowledgeable. You explain the physiology behind decisions.
- Use "we" language ("We're building your aerobic engine this block...")
- You understand that most distance runners are aerobically limited
- You prescribe easy running at truly easy effort (Zone 1-2)
- You value consistency and durability over individual workout performance

## Plan Design Rules
- 85-90% of training volume should be easy (Zone 1-2) during capacity phases
- Max 3 intensity sessions per week (includes tempo, VO2max, track, anaerobic, race-specific, fartlek, cruise intervals, lactate clearance, time trial)
- At least 1 rest or recovery day per week
- Max 1 long run per week
- Volume increase ≤ 10% week-over-week during load weeks
- Recovery weeks: reduce volume 30-60% from load weeks
- Prefer hill repeats for anaerobic development (better form, less injury risk)
- Hill sprints (8-12s neuromuscular) do NOT count as intensity — they can precede hard workout days
- Include strides (aerobic_development) or mixed_energy on 1-2 easy days per week
- Threshold/tempo work used sparingly — most runners overtrain this zone
- mixed_energy is the PRIMARY capacity builder (Olbrecht's "high and low") — use 1-2x/week during capacity phases

## Mesocycle Structure by Experience Level
- Beginner: 2 load weeks + 1 recovery week
- Intermediate: 3 load weeks + 1 recovery week
- Advanced: 3 load weeks + 1 recovery week (higher intensity density)

## Progression Within a Mesocycle
For each workout type, progress through duration categories week-over-week during load weeks:
- Week 1 of load: workout_type / short
- Week 2 of load: workout_type / medium
- Week 3 of load: workout_type / long (advanced only)
- Recovery week: workout_type / short OR switch to an easier workout type

Only one progression variable at a time. Do NOT simultaneously increase reps, duration, AND reduce recovery.

"#;

const COACH_JAN_PROMPT_SUFFIX: &str = r#"

## Duration Categories
Each running workout type has three duration categories: short, medium, long.
Choose based on the athlete's fitness level, the training week (load vs recovery), and the mesocycle progression.

## Athlete Context You Will Receive
When generating plans, you will receive the athlete's:
- Profile: age, weight, experience level, sports background, injury history
- Current fitness: CTL (chronic training load), weekly volume in km
- Zone calibration: LTHR, FTPace, resting HR, max HR
- Race goal: distance, date, target time
- Previous mesocycle performance (if available): actual vs planned volume, completion rate

Use ALL of this data to make decisions. A beginner with a soccer background (high anaerobic capacity) needs different training than a beginner from swimming (good aerobic base). An athlete with CTL of 20 cannot handle the same volume as one with CTL of 50.
"#;

pub fn coach_jan_system_prompt() -> String {
    let mut workout_section = String::from("## Workout Type Reference\n\n");

    // Group workouts by category for readability
    let categories: &[(&str, &[WorkoutType])] = &[
        ("Easy / Aerobic", &[
            WorkoutType::EasyRun, WorkoutType::LongRun, WorkoutType::LongRunProgression,
            WorkoutType::LongRunModerate, WorkoutType::AerobicDevelopment,
            WorkoutType::ModerateRun, WorkoutType::SteadyRun, WorkoutType::ProgressionRun,
        ]),
        ("Threshold / Utilization", &[
            WorkoutType::TempoRun, WorkoutType::CruiseIntervals,
            WorkoutType::FartlekStructured, WorkoutType::LactateClearance,
        ]),
        ("VO2max / Intervals", &[
            WorkoutType::Vo2maxIntervals, WorkoutType::UnderOver,
            WorkoutType::Track1200m, WorkoutType::Track1600m,
        ]),
        ("Mile / 3K Specific", &[
            WorkoutType::TrackMixed, WorkoutType::TrackMilePace, WorkoutType::TrackRaceCombo,
        ]),
        ("Speed / Anaerobic", &[
            WorkoutType::Track200m, WorkoutType::Track400m, WorkoutType::Track800m,
            WorkoutType::AnaerobicHills, WorkoutType::AnaerobicFlat, WorkoutType::AnaerobicPower,
        ]),
        ("Neuromuscular / Capacity", &[
            WorkoutType::HillSprints, WorkoutType::MixedEnergy,
            WorkoutType::FormDrills, WorkoutType::PlyoRunning,
        ]),
        ("Race / Assessment", &[
            WorkoutType::RaceSpecific, WorkoutType::TimeTrial,
        ]),
        ("Recovery", &[
            WorkoutType::ShakeoutRun, WorkoutType::RecoveryRun, WorkoutType::Rest,
        ]),
        ("Strength (Running Rewired)", &[
            WorkoutType::StrengthPrecision, WorkoutType::StrengthPerformance,
            WorkoutType::StrengthPower,
        ]),
    ];

    for (category_name, types) in categories {
        workout_section.push_str(&format!("### {}\n", category_name));
        for wt in *types {
            workout_section.push_str(&format!(
                "- **{}** (`{}`): {}\n",
                wt.display_name(),
                wt.as_str(),
                wt.coaching_guide(),
            ));
        }
        workout_section.push('\n');
    }

    format!(
        "{}{}{}",
        COACH_JAN_PROMPT_PREFIX,
        workout_section,
        COACH_JAN_PROMPT_SUFFIX,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_contains_all_workout_types() {
        let prompt = coach_jan_system_prompt();
        for wt in WorkoutType::all() {
            assert!(
                prompt.contains(wt.as_str()),
                "Prompt missing workout type: {}",
                wt.as_str()
            );
        }
    }

    #[test]
    fn prompt_contains_coaching_guides() {
        let prompt = coach_jan_system_prompt();
        // Spot-check a few coaching guides are present
        assert!(prompt.contains("ATP-PCr"), "Missing hill sprints energy system detail");
        assert!(prompt.contains("Olbrecht's signature"), "Missing mixed energy Olbrecht reference");
        assert!(prompt.contains("lactate is fuel"), "Missing lactate clearance physiology");
        assert!(prompt.contains("Running Rewired"), "Missing strength phase reference");
    }

    #[test]
    fn prompt_contains_progression_guidance() {
        let prompt = coach_jan_system_prompt();
        assert!(prompt.contains("Progression Within a Mesocycle"));
        assert!(prompt.contains("workout_type / short"));
        assert!(prompt.contains("workout_type / medium"));
        assert!(prompt.contains("workout_type / long"));
    }

    #[test]
    fn prompt_contains_athlete_context_section() {
        let prompt = coach_jan_system_prompt();
        assert!(prompt.contains("Athlete Context You Will Receive"));
        assert!(prompt.contains("CTL"));
        assert!(prompt.contains("LTHR"));
        assert!(prompt.contains("FTPace"));
    }
}
