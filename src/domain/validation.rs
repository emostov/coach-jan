// src/domain/validation.rs

use std::collections::HashSet;
use crate::domain::workouts::WorkoutType;

#[derive(Debug, Clone)]
pub struct PlannedDay {
    pub date: String,
    pub workout_type: WorkoutType,
    pub duration_category: Option<crate::domain::workouts::DurationCategory>,
    pub expected_tss: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeekType {
    Load,
    Recovery,
}

#[derive(Debug, Clone)]
pub struct WeekPlan {
    pub week_number: u32,
    pub week_type: WeekType,
    pub target_volume_km: f64,
    pub target_weekly_tss: f64,
    pub days: Vec<PlannedDay>,
}

pub struct ValidationContext {
    pub athlete_ctl: f64,
    pub previous_week_volume_km: Option<f64>,
}

#[derive(Debug)]
pub enum ValidationError {
    TooManyIntensitySessions { count: usize },
    NoRestDay,
    TooManyLongRuns { count: usize },
    DuplicateDate { date: String },
    VolumeIncreaseTooHigh { increase_pct: f64 },
    WeeklyTssOutOfRange { tss: f64, min: f64, max: f64 },
}

pub fn validate_week_plan(week: &WeekPlan, ctx: &ValidationContext) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Max 3 intensity sessions
    let intensity_count = week.days.iter().filter(|d| d.workout_type.is_intensity()).count();
    if intensity_count > 3 {
        errors.push(ValidationError::TooManyIntensitySessions { count: intensity_count });
    }

    // At least 1 rest or recovery day
    let rest_count = week.days.iter().filter(|d| {
        matches!(d.workout_type, WorkoutType::Rest | WorkoutType::RecoveryRun)
    }).count();
    if rest_count < 1 {
        errors.push(ValidationError::NoRestDay);
    }

    // Max 1 long run (LongRun, LongRunProgression, or LongRunModerate)
    let long_run_count = week.days.iter().filter(|d| {
        matches!(d.workout_type, WorkoutType::LongRun | WorkoutType::LongRunProgression | WorkoutType::LongRunModerate)
    }).count();
    if long_run_count > 1 {
        errors.push(ValidationError::TooManyLongRuns { count: long_run_count });
    }

    // No duplicate dates
    let mut seen_dates = HashSet::new();
    for day in &week.days {
        if !seen_dates.insert(&day.date) {
            errors.push(ValidationError::DuplicateDate { date: day.date.clone() });
        }
    }

    // Volume increase <= 10% (load weeks only)
    if week.week_type == WeekType::Load {
        if let Some(prev_vol) = ctx.previous_week_volume_km {
            if prev_vol > 0.0 {
                let increase_pct = (week.target_volume_km - prev_vol) / prev_vol * 100.0;
                if increase_pct > 10.0 {
                    errors.push(ValidationError::VolumeIncreaseTooHigh { increase_pct });
                }
            }
        }
    }

    // Weekly TSS range: 0.5x - 2.0x of CTL*7
    if ctx.athlete_ctl > 0.0 {
        let min_tss = ctx.athlete_ctl * 7.0 * 0.5;
        let max_tss = ctx.athlete_ctl * 7.0 * 2.0;
        if week.target_weekly_tss < min_tss || week.target_weekly_tss > max_tss {
            errors.push(ValidationError::WeeklyTssOutOfRange {
                tss: week.target_weekly_tss,
                min: min_tss,
                max: max_tss,
            });
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::workouts::{WorkoutType, DurationCategory};

    fn day(date: &str, wt: WorkoutType, cat: DurationCategory) -> PlannedDay {
        PlannedDay { date: date.to_string(), workout_type: wt, duration_category: Some(cat), expected_tss: 50.0 }
    }

    fn rest_day(date: &str) -> PlannedDay {
        PlannedDay { date: date.to_string(), workout_type: WorkoutType::Rest, duration_category: None, expected_tss: 0.0 }
    }

    #[test]
    fn valid_week_passes() {
        let week = WeekPlan {
            week_number: 1,
            week_type: WeekType::Load,
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![
                day("2026-03-02", WorkoutType::EasyRun, DurationCategory::Medium),
                day("2026-03-03", WorkoutType::Vo2maxIntervals, DurationCategory::Short),
                rest_day("2026-03-04"),
                day("2026-03-05", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-06", WorkoutType::AnaerobicHills, DurationCategory::Medium),
                rest_day("2026-03-07"),
                day("2026-03-08", WorkoutType::LongRun, DurationCategory::Medium),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: None };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn max_three_intensity_sessions() {
        let week = WeekPlan {
            week_number: 1,
            week_type: WeekType::Load,
            target_volume_km: 50.0,
            target_weekly_tss: 350.0,
            days: vec![
                day("2026-03-02", WorkoutType::Vo2maxIntervals, DurationCategory::Short),
                day("2026-03-03", WorkoutType::Track400m, DurationCategory::Short),
                day("2026-03-04", WorkoutType::AnaerobicHills, DurationCategory::Short),
                day("2026-03-05", WorkoutType::TempoRun, DurationCategory::Short),
                rest_day("2026-03-06"),
                day("2026-03-07", WorkoutType::EasyRun, DurationCategory::Medium),
                day("2026-03-08", WorkoutType::LongRun, DurationCategory::Medium),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: None };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::TooManyIntensitySessions { .. })));
    }

    #[test]
    fn must_have_rest_day() {
        let week = WeekPlan {
            week_number: 1,
            week_type: WeekType::Load,
            target_volume_km: 50.0,
            target_weekly_tss: 350.0,
            days: vec![
                day("2026-03-02", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-03", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-04", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-05", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-06", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-07", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-08", WorkoutType::LongRun, DurationCategory::Short),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: None };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::NoRestDay)));
    }

    #[test]
    fn max_one_long_run() {
        let week = WeekPlan {
            week_number: 1,
            week_type: WeekType::Load,
            target_volume_km: 50.0,
            target_weekly_tss: 350.0,
            days: vec![
                day("2026-03-02", WorkoutType::LongRun, DurationCategory::Medium),
                rest_day("2026-03-03"),
                day("2026-03-04", WorkoutType::EasyRun, DurationCategory::Short),
                rest_day("2026-03-05"),
                day("2026-03-06", WorkoutType::LongRun, DurationCategory::Medium),
                rest_day("2026-03-07"),
                day("2026-03-08", WorkoutType::EasyRun, DurationCategory::Short),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: None };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::TooManyLongRuns { .. })));
    }

    #[test]
    fn no_duplicate_dates() {
        let week = WeekPlan {
            week_number: 1,
            week_type: WeekType::Load,
            target_volume_km: 40.0,
            target_weekly_tss: 250.0,
            days: vec![
                day("2026-03-02", WorkoutType::EasyRun, DurationCategory::Short),
                day("2026-03-02", WorkoutType::Vo2maxIntervals, DurationCategory::Short),
                rest_day("2026-03-03"),
                rest_day("2026-03-04"),
                rest_day("2026-03-05"),
                rest_day("2026-03-06"),
                rest_day("2026-03-07"),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: None };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::DuplicateDate { .. })));
    }

    #[test]
    fn volume_increase_within_limit() {
        let week = WeekPlan {
            week_number: 2,
            week_type: WeekType::Load,
            target_volume_km: 55.0,  // 37.5% increase from 40
            target_weekly_tss: 350.0,
            days: vec![
                day("2026-03-09", WorkoutType::EasyRun, DurationCategory::Medium),
                rest_day("2026-03-10"),
                rest_day("2026-03-11"),
                rest_day("2026-03-12"),
                rest_day("2026-03-13"),
                rest_day("2026-03-14"),
                rest_day("2026-03-15"),
            ],
        };
        let ctx = ValidationContext { athlete_ctl: 40.0, previous_week_volume_km: Some(40.0) };
        let errors = validate_week_plan(&week, &ctx);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::VolumeIncreaseTooHigh { .. })));
    }
}
