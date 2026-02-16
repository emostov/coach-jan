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

    format!(
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
    )
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
) -> String {
    let available_types: Vec<&str> = WorkoutType::all_running()
        .iter()
        .map(|wt| wt.as_str())
        .collect();

    format!(
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
    )
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
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16);

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
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16);

        assert!(ctx.contains("not set"));
        assert!(ctx.contains("0.00 m/s"));
    }

    #[test]
    fn macrocycle_context_handles_missing_race_name() {
        let profile = test_profile();
        let mut goal = test_race_goal();
        goal.race_name = None;
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16);

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
        let ctx = build_macrocycle_context(&profile, &goal, 20.0, 20);
        assert!(ctx.contains("2 load weeks"));
    }

    #[test]
    fn advanced_gets_3_plus_1_structure() {
        let mut profile = test_profile();
        profile.experience_level = "advanced".to_string();
        let goal = test_race_goal();
        let ctx = build_macrocycle_context(&profile, &goal, 50.0, 12);
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
        let ctx = build_macrocycle_context(&profile, &goal, 35.0, 16);
        // 3.5 m/s => 1000/3.5/60 = 4.7619... min/km => 4:45/km
        assert!(ctx.contains("4:45/km"));
        assert!(ctx.contains("3.50 m/s"));
    }
}
