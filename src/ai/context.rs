use std::collections::HashMap;

use crate::db::plans::PlannedWorkout;
use crate::db::profiles::{AthleteProfile, RaceGoal};
use crate::domain::workouts::WorkoutType;

/// Format a distance in meters as a human-readable race distance string.
fn format_distance(meters: f64) -> String {
    // Common race distances
    let m = meters.round() as i64;
    match m {
        5000 => "5K".to_string(),
        10000 => "10K".to_string(),
        15000 => "15K".to_string(),
        21097 | 21098 | 21100 => "Half Marathon (21.1 km)".to_string(),
        42195 | 42200 => "Marathon (42.2 km)".to_string(),
        _ => {
            if meters >= 1000.0 {
                format!("{:.1} km", meters / 1000.0)
            } else {
                format!("{:.0} m", meters)
            }
        }
    }
}

/// Build context for macrocycle skeleton generation.
/// This is sent as the user message when asking Claude to create the overall plan structure.
pub fn build_macrocycle_context(
    profile: &AthleteProfile,
    race_goal: &RaceGoal,
    ctl: f64,
    weeks_until_race: i64,
    workout_history: Option<&str>,
) -> String {
    let experience = &profile.experience_level;
    let (load_weeks, recovery_weeks) = match experience.as_str() {
        "beginner" => (2, 1),
        "intermediate" => (3, 1),
        "advanced" => (3, 1),
        _ => (3, 1),
    };

    let weekly_km = profile.current_weekly_volume_km;
    let ftpace_display = profile.ftpace_m_per_s.map(|p| {
        let min_per_km = 1000.0 / p / 60.0;
        let mins = min_per_km as u32;
        let secs = ((min_per_km - mins as f64) * 60.0) as u32;
        format!("{}:{:02}/km", mins, secs)
    }).unwrap_or_else(|| "not set".to_string());

    let distance_display = format_distance(race_goal.distance_m);

    let mut result = format!(
        r#"Create a periodized macrocycle for this athlete:

- Name: {name}
- Age: {age}
- Weight: {weight} kg
- Experience level: {experience}
- Current fitness: CTL={ctl:.0}, weekly volume={weekly_km:.0} km
- LTHR: {lthr} bpm
- FTPace: {ftpace} ({ftpace_ms:.2} m/s)
- Resting HR: {rhr} bpm, Max HR: {max_hr} bpm

Race goal:
- Race: {race_name}
- Distance: {race_distance}
- Date: {race_date} ({weeks_until_race} weeks away)

Mesocycle structure for {experience} athletes: {load_weeks} load weeks + {recovery_weeks} recovery week(s)

Follow Olbrecht's capacity → utilization → taper progression.
Build aerobic capacity first, then transition to race-specific work.
Consider this athlete's current fitness (CTL={ctl:.0}) when setting volume targets.
The final mesocycle should be a taper of 1-2 weeks."#,
        name = profile.name,
        age = profile.age,
        weight = profile.weight_kg,
        experience = experience,
        ctl = ctl,
        weekly_km = weekly_km,
        lthr = profile.lthr,
        ftpace = ftpace_display,
        ftpace_ms = profile.ftpace_m_per_s.unwrap_or(0.0),
        rhr = profile.resting_hr,
        max_hr = profile.max_hr,
        race_name = race_goal.race_name.as_deref().unwrap_or("Goal Race"),
        race_distance = distance_display,
        race_date = race_goal.race_date,
        weeks_until_race = weeks_until_race,
        load_weeks = load_weeks,
        recovery_weeks = recovery_weeks,
    );

    match workout_history {
        Some(history) if !history.is_empty() => {
            result.push_str("\n\nPrevious training history:\n");
            result.push_str(history);
        }
        _ => {
            result.push_str("\n\nNo previous training history available.");
        }
    }

    result
}

/// Build context for mesocycle day-by-day plan generation.
pub fn build_mesocycle_context(
    profile: &AthleteProfile,
    phase: &str,
    focus: &str,
    load_weeks: i64,
    recovery_weeks: i64,
    start_date: &str,
    end_date: &str,
    target_volume_km: f64,
    ctl: f64,
    workout_history: Option<&str>,
) -> String {
    let available_types: Vec<&str> = WorkoutType::all_running()
        .iter()
        .map(|wt| wt.as_str())
        .collect();

    let mut result = format!(
        r#"Generate day-by-day workout assignments for this mesocycle:

- Phase: {phase}
- Focus: {focus}
- Duration: {total_weeks} weeks ({load_weeks} load + {recovery_weeks} recovery)
- Dates: {start_date} to {end_date}
- Athlete level: {experience}
- Current CTL: {ctl:.0}
- Target volume: {target_volume_km:.0} km/week (load weeks)

Available workout types: {available_types}
Duration categories: short, medium, long

Rules:
- Max 3 intensity sessions per week
- Volume increase ≤ 10% week-over-week in load weeks
- Recovery weeks: reduce volume 30-60% from load weeks
- At least 1 rest day per week
- Max 1 long run per week
- Prefer hill repeats for anaerobic work (hills before flat)
- Include strides (aerobic_development) on 1-2 easy days per week
- Don't schedule strength on VO2max/speed/track days
- Moderate runs and steady runs are great for aerobic build phases
- Under/over intervals (30/30s) are good for VO2max development
- Use long_run_moderate (Zone 3-4 finish) in build phases, long_run_progression (Zone 4-5 finish) in race-specific phases

Assign one workout per day. Every day in the date range must have an assignment (including rest days)."#,
        phase = phase,
        focus = focus,
        total_weeks = load_weeks + recovery_weeks,
        load_weeks = load_weeks,
        recovery_weeks = recovery_weeks,
        start_date = start_date,
        end_date = end_date,
        experience = profile.experience_level,
        ctl = ctl,
        target_volume_km = target_volume_km,
        available_types = available_types.join(", "),
    );

    if let Some(history) = workout_history {
        if !history.is_empty() {
            result.push_str("\n\n");
            result.push_str(history);
            result.push_str("\n\nUse this history to inform workout progression and recovery decisions.");
        }
    }

    result
}

/// Format detailed workout history for mesocycle generation and coach notes.
/// Groups workouts by week, showing completion status, RPE, and athlete notes.
pub fn format_workout_history_detailed(
    workouts: &[PlannedWorkout],
    phase: &str,
    focus: &str,
    total_weeks: i64,
) -> String {
    if workouts.is_empty() {
        return String::new();
    }

    let completed = workouts.iter().filter(|w| w.is_completed == 1).count();
    let total = workouts.len();
    let completion_pct = if total > 0 { (completed as f64 / total as f64 * 100.0) as u32 } else { 0 };

    let rpe_values: Vec<f64> = workouts
        .iter()
        .filter_map(|w| w.rpe.map(|r| r as f64))
        .collect();
    let avg_rpe = if rpe_values.is_empty() {
        None
    } else {
        Some(rpe_values.iter().sum::<f64>() / rpe_values.len() as f64)
    };

    let mut result = format!(
        "Previous mesocycle ({} / {}, {} weeks):\n",
        phase, focus, total_weeks
    );

    if let Some(rpe) = avg_rpe {
        result.push_str(&format!(
            "Completion: {}/{} ({}%) | Avg RPE: {:.1}\n",
            completed, total, completion_pct, rpe
        ));
    } else {
        result.push_str(&format!(
            "Completion: {}/{} ({}%)\n",
            completed, total, completion_pct
        ));
    }

    // Group by week (7-day chunks based on sorted dates)
    let mut week_workouts: Vec<Vec<&PlannedWorkout>> = Vec::new();
    let mut current_week: Vec<&PlannedWorkout> = Vec::new();
    let mut week_start: Option<&str> = None;

    for w in workouts {
        if current_week.len() >= 7 {
            week_workouts.push(current_week);
            current_week = Vec::new();
            week_start = None;
        }
        if week_start.is_none() {
            week_start = Some(&w.scheduled_date);
        }
        current_week.push(w);
    }
    if !current_week.is_empty() {
        week_workouts.push(current_week);
    }

    for (i, week) in week_workouts.iter().enumerate() {
        let mut day_strs: Vec<String> = Vec::new();
        for w in week {
            let weekday = weekday_abbr(&w.scheduled_date);
            let dur = w.duration_category.as_deref().unwrap_or("");
            let dur_str = if dur.is_empty() { String::new() } else { format!("/{}", dur) };

            if w.is_completed == 1 {
                let mut parts = format!("{} {}{} [done", weekday, w.workout_type, dur_str);
                if let Some(rpe) = w.rpe {
                    parts.push_str(&format!(" RPE:{}", rpe));
                }
                if let Some(ref notes) = w.athlete_notes {
                    let truncated: String = notes.chars().take(30).collect();
                    parts.push_str(&format!(" \"{}\"", truncated));
                }
                parts.push(']');
                day_strs.push(parts);
            } else {
                day_strs.push(format!("{} {}{} [missed]", weekday, w.workout_type, dur_str));
            }
        }
        result.push_str(&format!("Wk{}: {}\n", i + 1, day_strs.join(" | ")));
    }

    result
}

/// Format summary workout history for macrocycle skeleton generation.
/// Compact overview: completion rate, RPE, workout type distribution, missed patterns.
pub fn format_workout_history_summary(
    workouts: &[PlannedWorkout],
    phase: &str,
    focus: &str,
    total_weeks: i64,
) -> String {
    if workouts.is_empty() {
        return String::new();
    }

    let completed = workouts.iter().filter(|w| w.is_completed == 1).count();
    let total = workouts.len();
    let completion_pct = if total > 0 { (completed as f64 / total as f64 * 100.0) as u32 } else { 0 };

    let rpe_values: Vec<f64> = workouts
        .iter()
        .filter_map(|w| w.rpe.map(|r| r as f64))
        .collect();
    let avg_rpe = if rpe_values.is_empty() {
        None
    } else {
        Some(rpe_values.iter().sum::<f64>() / rpe_values.len() as f64)
    };

    let mut result = format!(
        "Previous mesocycle ({} / {}, {} weeks):\n",
        phase, focus, total_weeks
    );

    if let Some(rpe) = avg_rpe {
        result.push_str(&format!(
            "Completion: {}/{} ({}%) | Avg RPE: {:.1}\n",
            completed, total, completion_pct, rpe
        ));
    } else {
        result.push_str(&format!(
            "Completion: {}/{} ({}%)\n",
            completed, total, completion_pct
        ));
    }

    // Count workout types
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for w in workouts {
        *type_counts.entry(w.workout_type.clone()).or_insert(0) += 1;
    }
    let mut sorted_types: Vec<_> = type_counts.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    let type_display: Vec<String> = sorted_types.iter().map(|(t, c)| format!("{}({})", t, c)).collect();
    result.push_str(&format!("Workout types used: {}\n", type_display.join(", ")));

    // Count missed workout types
    let mut missed_counts: HashMap<String, usize> = HashMap::new();
    for w in workouts.iter().filter(|w| w.is_completed == 0) {
        *missed_counts.entry(w.workout_type.clone()).or_insert(0) += 1;
    }
    if !missed_counts.is_empty() {
        let mut sorted_missed: Vec<_> = missed_counts.iter().collect();
        sorted_missed.sort_by(|a, b| b.1.cmp(a.1));
        let missed_display: Vec<String> = sorted_missed.iter().map(|(t, c)| format!("{} {}", c, t)).collect();
        result.push_str(&format!("Missed: {}\n", missed_display.join(", ")));
    }

    result
}

/// Helper to get 3-letter weekday abbreviation from a YYYY-MM-DD date string.
fn weekday_abbr(date_str: &str) -> &'static str {
    use chrono::NaiveDate;
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        match date.format("%a").to_string().as_str() {
            "Mon" => "Mon",
            "Tue" => "Tue",
            "Wed" => "Wed",
            "Thu" => "Thu",
            "Fri" => "Fri",
            "Sat" => "Sat",
            "Sun" => "Sun",
            _ => "???",
        }
    } else {
        "???"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::profiles::{AthleteProfile, RaceGoal};

    fn test_profile() -> AthleteProfile {
        AthleteProfile {
            id: 1,
            user_id: 1,
            name: "Test Runner".to_string(),
            age: 35,
            weight_kg: 75.0,
            resting_hr: 55,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: Some(3.5),
            current_weekly_volume_km: 40.0,
            experience_level: "intermediate".to_string(),
            sports_background: None,
            created_at: "2026-01-01".to_string(),
            updated_at: "2026-01-01".to_string(),
        }
    }

    fn test_race_goal() -> RaceGoal {
        RaceGoal {
            id: 1,
            user_id: 1,
            distance_m: 21097.0,
            race_date: "2026-06-01".to_string(),
            race_name: Some("Spring Half".to_string()),
            target_time_seconds: Some(5400),
            is_active: true,
            created_at: "2026-01-01".to_string(),
        }
    }

    #[test]
    fn macrocycle_context_includes_all_fields() {
        let profile = test_profile();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16, None);

        assert!(ctx.contains("Test Runner"));
        assert!(ctx.contains("35"));
        assert!(ctx.contains("75"));
        assert!(ctx.contains("intermediate"));
        assert!(ctx.contains("CTL=35"));
        assert!(ctx.contains("40 km"));
        assert!(ctx.contains("170"));
        assert!(ctx.contains("Spring Half"));
        assert!(ctx.contains("Half Marathon"));
        assert!(ctx.contains("16 weeks away"));
        assert!(ctx.contains("3 load weeks"));
        assert!(ctx.contains("Olbrecht"));
    }

    #[test]
    fn macrocycle_context_handles_missing_ftpace() {
        let mut profile = test_profile();
        profile.ftpace_m_per_s = None;
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16, None);

        assert!(ctx.contains("not set"));
        assert!(ctx.contains("0.00 m/s"));
    }

    #[test]
    fn macrocycle_context_handles_missing_race_name() {
        let profile = test_profile();
        let mut goal = test_race_goal();
        goal.race_name = None;
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16, None);

        assert!(ctx.contains("Goal Race"));
    }

    #[test]
    fn mesocycle_context_includes_rules() {
        let profile = test_profile();
        let ctx = build_mesocycle_context(
            &profile,
            "capacity",
            "aerobic_capacity",
            3,
            1,
            "2026-03-01",
            "2026-03-28",
            45.0,
            35.0,
            None,
        );

        assert!(ctx.contains("capacity"));
        assert!(ctx.contains("aerobic_capacity"));
        assert!(ctx.contains("4 weeks"));
        assert!(ctx.contains("3 load"));
        assert!(ctx.contains("1 recovery"));
        assert!(ctx.contains("45 km/week"));
        assert!(ctx.contains("CTL: 35"));
        assert!(ctx.contains("easy_run"));
        assert!(ctx.contains("moderate_run"));
        assert!(ctx.contains("under_over"));
        assert!(ctx.contains("Max 3 intensity"));
        assert!(ctx.contains("long_run_moderate"));
    }

    #[test]
    fn mesocycle_context_includes_all_running_types() {
        let profile = test_profile();
        let ctx = build_mesocycle_context(
            &profile,
            "capacity",
            "aerobic_capacity",
            3,
            1,
            "2026-03-01",
            "2026-03-28",
            45.0,
            35.0,
            None,
        );

        for wt in WorkoutType::all_running() {
            assert!(
                ctx.contains(wt.as_str()),
                "Missing workout type {} in mesocycle context",
                wt.as_str()
            );
        }
    }

    #[test]
    fn beginner_gets_2_plus_1_structure() {
        let mut profile = test_profile();
        profile.experience_level = "beginner".to_string();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 20.0, 20, None);
        assert!(ctx.contains("2 load weeks"));
    }

    #[test]
    fn advanced_gets_3_plus_1_structure() {
        let mut profile = test_profile();
        profile.experience_level = "advanced".to_string();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 50.0, 12, None);
        assert!(ctx.contains("3 load weeks"));
    }

    #[test]
    fn format_distance_common_races() {
        assert_eq!(format_distance(5000.0), "5K");
        assert_eq!(format_distance(10000.0), "10K");
        assert_eq!(format_distance(21097.0), "Half Marathon (21.1 km)");
        assert_eq!(format_distance(42195.0), "Marathon (42.2 km)");
    }

    #[test]
    fn format_distance_custom() {
        assert_eq!(format_distance(8000.0), "8.0 km");
        assert_eq!(format_distance(800.0), "800 m");
    }

    #[test]
    fn ftpace_display_calculation() {
        let profile = test_profile(); // ftpace_m_per_s = 3.5
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16, None);
        // 3.5 m/s => 1000/3.5/60 = 4.7619... min/km => 4:45/km
        assert!(ctx.contains("4:45/km"));
        assert!(ctx.contains("3.50 m/s"));
    }

    // -----------------------------------------------------------------------
    // Macrocycle context with history
    // -----------------------------------------------------------------------

    #[test]
    fn macrocycle_context_with_history() {
        let profile = test_profile();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(
            &profile, &goal, 35.0, 16,
            Some("Previous mesocycle (capacity / aerobic_capacity, 4 weeks):\nCompletion: 22/28 (79%)")
        );
        assert!(ctx.contains("Previous training history:"));
        assert!(ctx.contains("Completion: 22/28 (79%)"));
    }

    #[test]
    fn macrocycle_context_without_history() {
        let profile = test_profile();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16, None);
        assert!(ctx.contains("No previous training history available"));
    }

    // -----------------------------------------------------------------------
    // Mesocycle context with history
    // -----------------------------------------------------------------------

    #[test]
    fn mesocycle_context_with_history() {
        let profile = test_profile();
        let ctx = build_mesocycle_context(
            &profile, "utilization", "aerobic_utilization", 3, 1,
            "2026-03-29", "2026-04-25", 50.0, 40.0,
            Some("Previous mesocycle workouts:\nWk1: Mon easy_run [done]"),
        );
        assert!(ctx.contains("Previous mesocycle workouts:"));
        assert!(ctx.contains("Use this history to inform workout progression"));
    }

    #[test]
    fn mesocycle_context_without_history() {
        let profile = test_profile();
        let ctx = build_mesocycle_context(
            &profile, "capacity", "aerobic_capacity", 3, 1,
            "2026-03-01", "2026-03-28", 45.0, 35.0,
            None,
        );
        // Should not contain history-related text
        assert!(!ctx.contains("Previous mesocycle"));
        assert!(!ctx.contains("Use this history"));
    }

    // -----------------------------------------------------------------------
    // Workout history formatter tests
    // -----------------------------------------------------------------------

    fn make_test_workout(
        date: &str,
        workout_type: &str,
        is_completed: i64,
        rpe: Option<i64>,
        athlete_notes: Option<&str>,
        duration_category: Option<&str>,
    ) -> PlannedWorkout {
        PlannedWorkout {
            id: 1,
            mesocycle_id: 1,
            user_id: 1,
            scheduled_date: date.to_string(),
            workout_type: workout_type.to_string(),
            duration_min: Some(45),
            duration_category: duration_category.map(|s| s.to_string()),
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: Some(35.0),
            description: None,
            coach_notes: None,
            target_distance_km: None,
            is_completed,
            completed_workout_id: None,
            rpe,
            athlete_notes: athlete_notes.map(|s| s.to_string()),
            actual_duration_min: None,
            completed_at: if is_completed == 1 { Some("2026-03-03T12:00:00Z".to_string()) } else { None },
            created_at: "2026-03-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn detailed_history_empty_workouts() {
        let result = format_workout_history_detailed(&[], "capacity", "aerobic_capacity", 4);
        assert!(result.is_empty());
    }

    #[test]
    fn detailed_history_shows_completion_and_rpe() {
        let workouts = vec![
            make_test_workout("2026-03-03", "easy_run", 1, Some(4), None, Some("medium")),
            make_test_workout("2026-03-04", "vo2max_intervals", 1, Some(8), Some("legs heavy"), Some("short")),
            make_test_workout("2026-03-05", "rest", 1, None, None, None),
            make_test_workout("2026-03-06", "easy_run", 0, None, None, Some("medium")),
        ];

        let result = format_workout_history_detailed(&workouts, "capacity", "aerobic_capacity", 4);
        assert!(result.contains("capacity / aerobic_capacity"));
        assert!(result.contains("Completion: 3/4 (75%)"));
        assert!(result.contains("Avg RPE: 6.0"));
        assert!(result.contains("[done RPE:4]"));
        assert!(result.contains("[done RPE:8 \"legs heavy\"]"));
        assert!(result.contains("[missed]"));
    }

    #[test]
    fn summary_history_shows_type_counts() {
        let workouts = vec![
            make_test_workout("2026-03-03", "easy_run", 1, Some(4), None, Some("medium")),
            make_test_workout("2026-03-04", "easy_run", 1, Some(5), None, Some("medium")),
            make_test_workout("2026-03-05", "rest", 1, None, None, None),
            make_test_workout("2026-03-06", "tempo_run", 0, None, None, Some("short")),
        ];

        let result = format_workout_history_summary(&workouts, "capacity", "aerobic_capacity", 4);
        assert!(result.contains("Completion: 3/4 (75%)"));
        assert!(result.contains("Avg RPE: 4.5"));
        assert!(result.contains("easy_run(2)"));
        assert!(result.contains("rest(1)"));
        assert!(result.contains("Missed: 1 tempo_run"));
    }

    #[test]
    fn summary_history_empty_workouts() {
        let result = format_workout_history_summary(&[], "capacity", "aerobic_capacity", 4);
        assert!(result.is_empty());
    }
}
