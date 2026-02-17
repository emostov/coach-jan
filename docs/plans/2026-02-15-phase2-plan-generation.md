# Phase 2: Training Plan Generation — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** After profile setup, Claude generates a periodized macrocycle. Athlete reviews the skeleton and confirms. Weekly plans are generated for the first mesocycle using a workout registry for deterministic detail fill.

**Architecture:** Three-layer generation: (1) Claude generates macrocycle skeleton, (2) Claude assigns workout types per day for entire mesocycle in one call, (3) server fills details from hardcoded workout registry + Claude adds coach notes. Raw reqwest client for Anthropic API with tool_use support.

**Tech Stack:** Rust/Axum backend, reqwest for Claude API, wiremock for API mocking in tests, React/TypeScript frontend with TanStack Query.

---

## Task 1: Database Migration for Training Plans

**Files:**
- Create: `migrations/003_create_training_plans.sql`

**Step 1: Write the migration**

```sql
-- migrations/003_create_training_plans.sql

CREATE TABLE macrocycles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    race_goal_id INTEGER NOT NULL REFERENCES race_goals(id) ON DELETE CASCADE,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    target_ctl REAL,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'abandoned')),
    coach_message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE mesocycles (
    id INTEGER PRIMARY KEY,
    macrocycle_id INTEGER NOT NULL REFERENCES macrocycles(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    phase TEXT NOT NULL CHECK (phase IN ('capacity', 'utilization', 'taper', 'recovery', 'transition')),
    focus TEXT NOT NULL CHECK (focus IN ('aerobic_capacity', 'aerobic_utilization', 'anaerobic_capacity', 'anaerobic_utilization', 'race_specific', 'recovery')),
    load_weeks INTEGER NOT NULL,
    recovery_weeks INTEGER NOT NULL,
    target_volume_km REAL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'active', 'completed')),
    evaluation_summary TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE planned_workouts (
    id INTEGER PRIMARY KEY,
    mesocycle_id INTEGER NOT NULL REFERENCES mesocycles(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scheduled_date TEXT NOT NULL,
    workout_type TEXT NOT NULL CHECK (workout_type IN (
        'easy_run', 'long_run', 'long_run_progression', 'aerobic_development',
        'tempo_run', 'vo2max_intervals', 'track_200m', 'track_400m', 'track_800m',
        'anaerobic_hills', 'anaerobic_flat', 'anaerobic_power',
        'race_specific', 'recovery_run', 'rest',
        'strength_precision', 'strength_performance', 'strength_power'
    )),
    duration_min INTEGER,
    duration_category TEXT CHECK (duration_category IN ('short', 'medium', 'long')),
    target_hr_zones TEXT,
    target_pace_zones TEXT,
    expected_tss REAL,
    description TEXT,
    coach_notes TEXT,
    is_completed INTEGER NOT NULL DEFAULT 0,
    completed_workout_id INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_planned_workouts_user_date ON planned_workouts(user_id, scheduled_date);
```

**Step 2: Verify migration runs**

Run: `cargo test -- --test integration_tests 2>&1 | head -20`
Expected: Tests still pass (migration runs successfully on in-memory DB)

**Step 3: Commit**

```bash
git add migrations/003_create_training_plans.sql
git commit -m "feat: add training plans migration (macrocycles, mesocycles, planned_workouts)"
```

---

## Task 2: Workout Registry

**Files:**
- Create: `src/domain/workouts.rs`
- Modify: `src/domain/mod.rs` (add `pub mod workouts;`)

The workout registry is a static data structure defining all workout templates with their parameters. Claude picks workout type + duration category; the server fills in specifics.

**Step 1: Write tests for the workout registry**

Add to `src/domain/workouts.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_workout_types_have_templates() {
        let registry = WorkoutRegistry::new();
        // Every running workout type should exist in the registry
        for wt in WorkoutType::all_running() {
            assert!(
                registry.get(&wt).is_some(),
                "Missing template for {:?}",
                wt
            );
        }
    }

    #[test]
    fn all_templates_have_three_duration_categories() {
        let registry = WorkoutRegistry::new();
        for wt in WorkoutType::all_running() {
            let template = registry.get(&wt).unwrap();
            assert!(template.durations.get(&DurationCategory::Short).is_some(),
                "{:?} missing Short", wt);
            assert!(template.durations.get(&DurationCategory::Medium).is_some(),
                "{:?} missing Medium", wt);
            assert!(template.durations.get(&DurationCategory::Long).is_some(),
                "{:?} missing Long", wt);
        }
    }

    #[test]
    fn tss_ranges_are_sensible() {
        let registry = WorkoutRegistry::new();
        for wt in WorkoutType::all_running() {
            let template = registry.get(&wt).unwrap();
            for (cat, params) in &template.durations {
                assert!(
                    params.expected_tss_min < params.expected_tss_max,
                    "{:?}/{:?}: min_tss >= max_tss",
                    wt, cat
                );
                assert!(
                    params.expected_tss_min > 0.0,
                    "{:?}/{:?}: tss_min should be positive",
                    wt, cat
                );
            }
        }
    }

    #[test]
    fn duration_increases_with_category() {
        let registry = WorkoutRegistry::new();
        for wt in WorkoutType::all_running() {
            let template = registry.get(&wt).unwrap();
            let short = template.durations[&DurationCategory::Short].total_duration_min;
            let medium = template.durations[&DurationCategory::Medium].total_duration_min;
            let long = template.durations[&DurationCategory::Long].total_duration_min;
            assert!(
                short <= medium && medium <= long,
                "{:?}: durations not increasing: {}/{}/{}",
                wt, short, medium, long
            );
        }
    }

    #[test]
    fn rest_and_strength_types_exist() {
        assert!(WorkoutType::from_str("rest").is_some());
        assert!(WorkoutType::from_str("strength_precision").is_some());
        assert!(WorkoutType::from_str("strength_performance").is_some());
        assert!(WorkoutType::from_str("strength_power").is_some());
    }

    #[test]
    fn workout_type_roundtrip() {
        for wt in WorkoutType::all() {
            let s = wt.as_str();
            let back = WorkoutType::from_str(s).unwrap_or_else(|| panic!("Failed to parse: {}", s));
            assert_eq!(back, wt);
        }
    }

    #[test]
    fn is_intensity_session_correct() {
        assert!(WorkoutType::Vo2maxIntervals.is_intensity());
        assert!(WorkoutType::Track200m.is_intensity());
        assert!(WorkoutType::Track400m.is_intensity());
        assert!(WorkoutType::Track800m.is_intensity());
        assert!(WorkoutType::AnaerobicHills.is_intensity());
        assert!(WorkoutType::AnaerobicFlat.is_intensity());
        assert!(WorkoutType::AnaerobicPower.is_intensity());
        assert!(WorkoutType::RaceSpecific.is_intensity());
        assert!(WorkoutType::TempoRun.is_intensity());
        assert!(!WorkoutType::EasyRun.is_intensity());
        assert!(!WorkoutType::LongRun.is_intensity());
        assert!(!WorkoutType::RecoveryRun.is_intensity());
        assert!(!WorkoutType::Rest.is_intensity());
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test domain::workouts -- -v 2>&1 | tail -5`
Expected: FAIL (module doesn't exist yet)

**Step 3: Implement the workout registry**

```rust
// src/domain/workouts.rs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// WorkoutType enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutType {
    EasyRun,
    LongRun,
    LongRunProgression,
    AerobicDevelopment,
    TempoRun,
    Vo2maxIntervals,
    Track200m,
    Track400m,
    Track800m,
    AnaerobicHills,
    AnaerobicFlat,
    AnaerobicPower,
    RaceSpecific,
    RecoveryRun,
    Rest,
    StrengthPrecision,
    StrengthPerformance,
    StrengthPower,
}

impl WorkoutType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EasyRun => "easy_run",
            Self::LongRun => "long_run",
            Self::LongRunProgression => "long_run_progression",
            Self::AerobicDevelopment => "aerobic_development",
            Self::TempoRun => "tempo_run",
            Self::Vo2maxIntervals => "vo2max_intervals",
            Self::Track200m => "track_200m",
            Self::Track400m => "track_400m",
            Self::Track800m => "track_800m",
            Self::AnaerobicHills => "anaerobic_hills",
            Self::AnaerobicFlat => "anaerobic_flat",
            Self::AnaerobicPower => "anaerobic_power",
            Self::RaceSpecific => "race_specific",
            Self::RecoveryRun => "recovery_run",
            Self::Rest => "rest",
            Self::StrengthPrecision => "strength_precision",
            Self::StrengthPerformance => "strength_performance",
            Self::StrengthPower => "strength_power",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "easy_run" => Some(Self::EasyRun),
            "long_run" => Some(Self::LongRun),
            "long_run_progression" => Some(Self::LongRunProgression),
            "aerobic_development" => Some(Self::AerobicDevelopment),
            "tempo_run" => Some(Self::TempoRun),
            "vo2max_intervals" => Some(Self::Vo2maxIntervals),
            "track_200m" => Some(Self::Track200m),
            "track_400m" => Some(Self::Track400m),
            "track_800m" => Some(Self::Track800m),
            "anaerobic_hills" => Some(Self::AnaerobicHills),
            "anaerobic_flat" => Some(Self::AnaerobicFlat),
            "anaerobic_power" => Some(Self::AnaerobicPower),
            "race_specific" => Some(Self::RaceSpecific),
            "recovery_run" => Some(Self::RecoveryRun),
            "rest" => Some(Self::Rest),
            "strength_precision" => Some(Self::StrengthPrecision),
            "strength_performance" => Some(Self::StrengthPerformance),
            "strength_power" => Some(Self::StrengthPower),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::EasyRun => "Easy Run",
            Self::LongRun => "Long Run",
            Self::LongRunProgression => "Long Run w/ Progression",
            Self::AerobicDevelopment => "Aerobic Development",
            Self::TempoRun => "Tempo Run",
            Self::Vo2maxIntervals => "VO2max Intervals",
            Self::Track200m => "Track 200m Repeats",
            Self::Track400m => "Track 400m Repeats",
            Self::Track800m => "Track 800m Repeats",
            Self::AnaerobicHills => "Anaerobic Hill Repeats",
            Self::AnaerobicFlat => "Anaerobic Repeats",
            Self::AnaerobicPower => "Anaerobic Power",
            Self::RaceSpecific => "Race-Specific",
            Self::RecoveryRun => "Recovery Run",
            Self::Rest => "Rest Day",
            Self::StrengthPrecision => "Strength (Precision)",
            Self::StrengthPerformance => "Strength (Performance)",
            Self::StrengthPower => "Strength (Power)",
        }
    }

    /// Whether this workout counts as a high-intensity session for plan validation.
    pub fn is_intensity(&self) -> bool {
        matches!(
            self,
            Self::Vo2maxIntervals
                | Self::Track200m
                | Self::Track400m
                | Self::Track800m
                | Self::AnaerobicHills
                | Self::AnaerobicFlat
                | Self::AnaerobicPower
                | Self::RaceSpecific
                | Self::TempoRun
        )
    }

    /// All workout types.
    pub fn all() -> Vec<Self> {
        vec![
            Self::EasyRun, Self::LongRun, Self::LongRunProgression,
            Self::AerobicDevelopment, Self::TempoRun, Self::Vo2maxIntervals,
            Self::Track200m, Self::Track400m, Self::Track800m,
            Self::AnaerobicHills, Self::AnaerobicFlat, Self::AnaerobicPower,
            Self::RaceSpecific, Self::RecoveryRun, Self::Rest,
            Self::StrengthPrecision, Self::StrengthPerformance, Self::StrengthPower,
        ]
    }

    /// Running workout types only (have templates in the registry).
    pub fn all_running() -> Vec<Self> {
        vec![
            Self::EasyRun, Self::LongRun, Self::LongRunProgression,
            Self::AerobicDevelopment, Self::TempoRun, Self::Vo2maxIntervals,
            Self::Track200m, Self::Track400m, Self::Track800m,
            Self::AnaerobicHills, Self::AnaerobicFlat, Self::AnaerobicPower,
            Self::RaceSpecific, Self::RecoveryRun,
        ]
    }
}

// ---------------------------------------------------------------------------
// DurationCategory
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationCategory {
    Short,
    Medium,
    Long,
}

impl DurationCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Medium => "medium",
            Self::Long => "long",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "short" => Some(Self::Short),
            "medium" => Some(Self::Medium),
            "long" => Some(Self::Long),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Workout Template
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct WorkoutTemplate {
    pub workout_type: WorkoutType,
    pub description: &'static str,
    pub target_hr_zones: Vec<u8>,
    pub target_pace_zones: Vec<u8>,
    pub durations: HashMap<DurationCategory, DurationParams>,
}

#[derive(Debug, Clone)]
pub struct DurationParams {
    pub total_duration_min: u16,
    pub structure: &'static str,
    pub expected_tss_min: f64,
    pub expected_tss_max: f64,
}

// ---------------------------------------------------------------------------
// Workout Registry
// ---------------------------------------------------------------------------

pub struct WorkoutRegistry {
    templates: HashMap<WorkoutType, WorkoutTemplate>,
}

impl WorkoutRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Easy Run: continuous @ Zone 1-2
        templates.insert(WorkoutType::EasyRun, WorkoutTemplate {
            workout_type: WorkoutType::EasyRun,
            description: "Easy continuous run at conversational pace",
            target_hr_zones: vec![1, 2],
            target_pace_zones: vec![1, 2],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 30,
                    structure: "30 min continuous @ Zone 1-2",
                    expected_tss_min: 25.0, expected_tss_max: 35.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "45 min continuous @ Zone 1-2",
                    expected_tss_min: 40.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 60,
                    structure: "60 min continuous @ Zone 1-2",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
            ]),
        });

        // Long Run: extended continuous @ Zone 1-2
        templates.insert(WorkoutType::LongRun, WorkoutTemplate {
            workout_type: WorkoutType::LongRun,
            description: "Extended easy run building aerobic endurance",
            target_hr_zones: vec![1, 2],
            target_pace_zones: vec![1, 2],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 70,
                    structure: "70 min continuous @ Zone 1-2",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 90,
                    structure: "90 min continuous @ Zone 1-2",
                    expected_tss_min: 85.0, expected_tss_max: 110.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 120,
                    structure: "120 min continuous @ Zone 1-2",
                    expected_tss_min: 110.0, expected_tss_max: 150.0,
                }),
            ]),
        });

        // Long Run Progression: last 2-3 miles at anaerobic power
        templates.insert(WorkoutType::LongRunProgression, WorkoutTemplate {
            workout_type: WorkoutType::LongRunProgression,
            description: "Long run with last 2-3 miles at anaerobic power effort",
            target_hr_zones: vec![1, 2, 4, 5],
            target_pace_zones: vec![1, 2, 4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 70,
                    structure: "55 min easy @ Zone 1-2, then 15 min @ Zone 4-5 (anaerobic power)",
                    expected_tss_min: 70.0, expected_tss_max: 95.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 90,
                    structure: "70 min easy @ Zone 1-2, then 20 min @ Zone 4-5 (anaerobic power)",
                    expected_tss_min: 95.0, expected_tss_max: 125.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 110,
                    structure: "85 min easy @ Zone 1-2, then 25 min @ Zone 4-5 (anaerobic power)",
                    expected_tss_min: 120.0, expected_tss_max: 160.0,
                }),
            ]),
        });

        // Aerobic Development: easy + strides
        templates.insert(WorkoutType::AerobicDevelopment, WorkoutTemplate {
            workout_type: WorkoutType::AerobicDevelopment,
            description: "Easy run with short fast strides for neuromuscular activation",
            target_hr_zones: vec![1, 2],
            target_pace_zones: vec![1, 2, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 30,
                    structure: "25 min easy with 4x20s strides",
                    expected_tss_min: 30.0, expected_tss_max: 40.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 40,
                    structure: "35 min easy with 6x20s strides",
                    expected_tss_min: 40.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 50,
                    structure: "45 min easy with 8x20s strides",
                    expected_tss_min: 50.0, expected_tss_max: 65.0,
                }),
            ]),
        });

        // Tempo Run: sustained threshold (rarely used per coach)
        templates.insert(WorkoutType::TempoRun, WorkoutTemplate {
            workout_type: WorkoutType::TempoRun,
            description: "Sustained threshold effort (used sparingly)",
            target_hr_zones: vec![3, 4],
            target_pace_zones: vec![3, 4],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 15 min @ Zone 3-4 + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 2x12 min @ Zone 3-4 / 3 min easy + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 75.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 25 min @ Zone 3-4 + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
            ]),
        });

        // VO2max Intervals: 1-3 min efforts @ Zone 5
        templates.insert(WorkoutType::Vo2maxIntervals, WorkoutTemplate {
            workout_type: WorkoutType::Vo2maxIntervals,
            description: "High-intensity intervals at VO2max effort (1-3 min)",
            target_hr_zones: vec![5, 6],
            target_pace_zones: vec![5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 5x1 min @ Zone 5 / 2 min jog + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 5x2 min @ Zone 5 / 2 min jog + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 6x3 min @ Zone 5 / 2.5 min jog + 10 min cooldown",
                    expected_tss_min: 75.0, expected_tss_max: 100.0,
                }),
            ]),
        });

        // Track 200m Repeats
        templates.insert(WorkoutType::Track200m, WorkoutTemplate {
            workout_type: WorkoutType::Track200m,
            description: "Track 200m repeats at near-max effort with full recovery",
            target_hr_zones: vec![6, 7],
            target_pace_zones: vec![6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 6x200m near-max / 200m walk + 10 min cooldown",
                    expected_tss_min: 35.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 8x200m near-max / 200m walk + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 10x200m near-max / 200m walk + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 70.0,
                }),
            ]),
        });

        // Track 400m Repeats
        templates.insert(WorkoutType::Track400m, WorkoutTemplate {
            workout_type: WorkoutType::Track400m,
            description: "Track 400m repeats at hard effort",
            target_hr_zones: vec![5, 6],
            target_pace_zones: vec![5, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 6x400m hard / 400m jog + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 8x400m hard / 400m jog + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 10x400m hard / 400m jog + 10 min cooldown",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Track 800m Repeats
        templates.insert(WorkoutType::Track800m, WorkoutTemplate {
            workout_type: WorkoutType::Track800m,
            description: "Track 800m repeats at VO2max-anaerobic effort",
            target_hr_zones: vec![5, 6],
            target_pace_zones: vec![5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 4x800m @ Zone 5 / 400m jog + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 70.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 5x800m @ Zone 5 / 400m jog + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 6x800m @ Zone 5 / 400m jog + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 95.0,
                }),
            ]),
        });

        // Anaerobic Hill Repeats: 15s-1:30 on hills (preferred)
        templates.insert(WorkoutType::AnaerobicHills, WorkoutTemplate {
            workout_type: WorkoutType::AnaerobicHills,
            description: "Short explosive hill repeats building anaerobic capacity",
            target_hr_zones: vec![6, 7],
            target_pace_zones: vec![6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "15 min warmup + 6x30s hill sprint / walk down + 10 min cooldown",
                    expected_tss_min: 35.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "15 min warmup + 8x45s hill sprint / walk down + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 50,
                    structure: "15 min warmup + 8x1:15 hill effort / walk down + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
            ]),
        });

        // Anaerobic Flat: 15s-1:30 on flat
        templates.insert(WorkoutType::AnaerobicFlat, WorkoutTemplate {
            workout_type: WorkoutType::AnaerobicFlat,
            description: "Short explosive flat repeats building anaerobic capacity",
            target_hr_zones: vec![6, 7],
            target_pace_zones: vec![6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "15 min warmup + 6x20s all-out / 2 min walk + 10 min cooldown",
                    expected_tss_min: 35.0, expected_tss_max: 45.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "15 min warmup + 8x30s all-out / 2.5 min walk + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "15 min warmup + 10x45s hard / 2.5 min walk + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
            ]),
        });

        // Anaerobic Power: 5-8 min (normal), 10-30 min (long events)
        templates.insert(WorkoutType::AnaerobicPower, WorkoutTemplate {
            workout_type: WorkoutType::AnaerobicPower,
            description: "Sustained hard efforts developing anaerobic power (5-30 min efforts)",
            target_hr_zones: vec![4, 5],
            target_pace_zones: vec![4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 6x5 min @ Zone 4-5 / 90s walk + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 6x8 min @ Zone 4-5 / 90s walk + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 90.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 65,
                    structure: "10 min warmup + 2x15 min @ Zone 4-5 / 90s walk + 10 min cooldown",
                    expected_tss_min: 80.0, expected_tss_max: 110.0,
                }),
            ]),
        });

        // Race-Specific: intervals at goal race pace
        templates.insert(WorkoutType::RaceSpecific, WorkoutTemplate {
            workout_type: WorkoutType::RaceSpecific,
            description: "Intervals at goal race pace",
            target_hr_zones: vec![3, 4, 5],
            target_pace_zones: vec![3, 4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 3x5 min @ race pace / 2 min jog + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 4x8 min @ race pace / 3 min jog + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 85.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 60,
                    structure: "10 min warmup + 3x12 min @ race pace / 3 min jog + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Recovery Run
        templates.insert(WorkoutType::RecoveryRun, WorkoutTemplate {
            workout_type: WorkoutType::RecoveryRun,
            description: "Very easy recovery effort",
            target_hr_zones: vec![1],
            target_pace_zones: vec![1],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 20,
                    structure: "20 min @ Zone 1",
                    expected_tss_min: 10.0, expected_tss_max: 15.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 25,
                    structure: "25 min @ Zone 1",
                    expected_tss_min: 15.0, expected_tss_max: 20.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 30,
                    structure: "30 min @ Zone 1",
                    expected_tss_min: 18.0, expected_tss_max: 25.0,
                }),
            ]),
        });

        Self { templates }
    }

    pub fn get(&self, workout_type: &WorkoutType) -> Option<&WorkoutTemplate> {
        self.templates.get(workout_type)
    }

    /// Resolve a workout type + duration category into a description with the
    /// athlete's actual zone values filled in.
    pub fn resolve(
        &self,
        workout_type: &WorkoutType,
        duration_category: &DurationCategory,
        hr_zones: &crate::domain::types::HrZones,
        pace_zones: Option<&crate::domain::types::PaceZones>,
    ) -> Option<ResolvedWorkout> {
        let template = self.templates.get(workout_type)?;
        let params = template.durations.get(duration_category)?;

        // Build HR zone string
        let hr_zone_str: Vec<String> = template.target_hr_zones.iter().map(|z| {
            if let Some(zone) = hr_zones.zones.iter().find(|hz| hz.zone == *z) {
                match zone.max_bpm {
                    Some(max) => format!("Z{} ({}-{} bpm)", z, zone.min_bpm, max),
                    None => format!("Z{} ({}+ bpm)", z, zone.min_bpm),
                }
            } else {
                format!("Z{}", z)
            }
        }).collect();

        Some(ResolvedWorkout {
            workout_type: *workout_type,
            duration_category: *duration_category,
            duration_min: params.total_duration_min,
            structure: params.structure.to_string(),
            description: template.description.to_string(),
            target_hr_zones: template.target_hr_zones.clone(),
            target_pace_zones: template.target_pace_zones.clone(),
            hr_zone_display: hr_zone_str.join(", "),
            expected_tss: (params.expected_tss_min + params.expected_tss_max) / 2.0,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedWorkout {
    pub workout_type: WorkoutType,
    pub duration_category: DurationCategory,
    pub duration_min: u16,
    pub structure: String,
    pub description: String,
    pub target_hr_zones: Vec<u8>,
    pub target_pace_zones: Vec<u8>,
    pub hr_zone_display: String,
    pub expected_tss: f64,
}
```

**Step 4: Update domain/mod.rs**

Add `pub mod workouts;` to `src/domain/mod.rs`.

**Step 5: Run tests**

Run: `cargo test domain::workouts -- -v`
Expected: All tests PASS

**Step 6: Commit**

```bash
git add src/domain/workouts.rs src/domain/mod.rs
git commit -m "feat: add workout registry with templates for all workout types"
```

---

## Task 3: Plan Validation Rules

**Files:**
- Create: `src/domain/validation.rs`
- Modify: `src/domain/mod.rs` (add `pub mod validation;`)

**Step 1: Write tests for validation rules**

Each rule gets its own test. The validation function takes a list of planned workout assignments for one week and returns validation errors.

```rust
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
```

**Step 2: Run tests to verify they fail**

Run: `cargo test domain::validation -- -v 2>&1 | tail -5`
Expected: FAIL

**Step 3: Implement validation**

```rust
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

    // Max 1 long run (LongRun or LongRunProgression)
    let long_run_count = week.days.iter().filter(|d| {
        matches!(d.workout_type, WorkoutType::LongRun | WorkoutType::LongRunProgression)
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
```

**Step 4: Update domain/mod.rs, run tests**

Add `pub mod validation;` to `src/domain/mod.rs`.

Run: `cargo test domain::validation -- -v`
Expected: All tests PASS

**Step 5: Commit**

```bash
git add src/domain/validation.rs src/domain/mod.rs
git commit -m "feat: add plan validation rules (intensity limit, rest days, volume, TSS)"
```

---

## Task 4: Claude API Client

**Files:**
- Create: `src/ai/mod.rs`, `src/ai/client.rs`
- Modify: `src/lib.rs` (add `pub mod ai;`)

This is the raw reqwest client wrapping the Anthropic Messages API.

**Step 1: Write tests**

```rust
// In src/ai/client.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_to_string() {
        assert_eq!(Model::Sonnet.as_str(), "claude-sonnet-4-5-20250929");
        assert_eq!(Model::Haiku.as_str(), "claude-haiku-4-5-20251001");
    }

    #[test]
    fn parse_tool_use_from_response() {
        let json = serde_json::json!({
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "id": "toolu_123",
                "name": "generate_macrocycle_skeleton",
                "input": {
                    "target_ctl": 65.0,
                    "mesocycles": []
                }
            }],
            "model": "claude-sonnet-4-5-20250929",
            "stop_reason": "tool_use",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        });

        let response: ClaudeResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.stop_reason, "tool_use");
        assert_eq!(response.content.len(), 1);
        match &response.content[0] {
            ContentBlock::ToolUse { name, input, .. } => {
                assert_eq!(name, "generate_macrocycle_skeleton");
                assert_eq!(input["target_ctl"], 65.0);
            }
            _ => panic!("Expected ToolUse"),
        }
    }

    #[test]
    fn parse_text_response() {
        let json = serde_json::json!({
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{ "type": "text", "text": "Hello!" }],
            "model": "claude-sonnet-4-5-20250929",
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 10, "output_tokens": 5 }
        });

        let response: ClaudeResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.stop_reason, "end_turn");
        match &response.content[0] {
            ContentBlock::Text { text } => assert_eq!(text, "Hello!"),
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn claude_error_display() {
        let err = ClaudeError::RateLimit;
        assert_eq!(format!("{}", err), "Rate limited by Anthropic API");
    }
}
```

**Step 2: Implement**

```rust
// src/ai/mod.rs
pub mod client;

// src/ai/client.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Model {
    Sonnet,
    Haiku,
    Opus,
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sonnet => "claude-sonnet-4-5-20250929",
            Self::Haiku => "claude-haiku-4-5-20251001",
            Self::Opus => "claude-opus-4-6",
        }
    }
}

// --- Request types ---

#[derive(Debug, Serialize)]
pub struct ClaudeRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentBlock>,
}

impl Message {
    pub fn user(text: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text: text.to_string() }],
        }
    }

    pub fn assistant_tool_use(id: &str, name: &str, input: Value) -> Self {
        Self {
            role: "assistant".to_string(),
            content: vec![ContentBlock::ToolUse {
                id: id.to_string(),
                name: name.to_string(),
                input,
            }],
        }
    }

    pub fn user_tool_result(tool_use_id: &str, content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::ToolResult {
                tool_use_id: tool_use_id.to_string(),
                content: content.to_string(),
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String },
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

// --- Response types ---

#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl ClaudeResponse {
    /// Extract the first tool_use block, if any.
    pub fn tool_use(&self) -> Option<(&str, &str, &Value)> {
        self.content.iter().find_map(|block| match block {
            ContentBlock::ToolUse { id, name, input } => Some((id.as_str(), name.as_str(), input)),
            _ => None,
        })
    }

    /// Extract the first text block, if any.
    pub fn text(&self) -> Option<&str> {
        self.content.iter().find_map(|block| match block {
            ContentBlock::Text { text } => Some(text.as_str()),
            _ => None,
        })
    }
}

// --- Error types ---

#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
    #[error("Rate limited by Anthropic API")]
    RateLimit,
    #[error("Request timed out")]
    Timeout,
    #[error("Invalid response from API: {0}")]
    InvalidResponse(String),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

// --- Client ---

pub struct ClaudeClient {
    http: reqwest::Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build HTTP client");
        Self { http, api_key }
    }

    pub async fn send(
        &self,
        model: Model,
        system: Option<&str>,
        messages: Vec<Message>,
        tools: Vec<Tool>,
        max_tokens: u32,
    ) -> Result<ClaudeResponse, ClaudeError> {
        let request = ClaudeRequest {
            model: model.as_str().to_string(),
            system: system.map(String::from),
            messages,
            tools,
            max_tokens,
        };

        let mut retries = 0;
        let max_retries = 3;

        loop {
            let result = self
                .http
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request)
                .send()
                .await;

            match result {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if status == 429 && retries < max_retries {
                        retries += 1;
                        let delay = Duration::from_secs(1 << retries);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    if status == 429 {
                        return Err(ClaudeError::RateLimit);
                    }
                    if status >= 400 {
                        let body = resp.text().await.unwrap_or_default();
                        return Err(ClaudeError::ApiError {
                            status,
                            message: body,
                        });
                    }

                    let response: ClaudeResponse = resp
                        .json()
                        .await
                        .map_err(|e| ClaudeError::InvalidResponse(e.to_string()))?;
                    return Ok(response);
                }
                Err(e) if e.is_timeout() => return Err(ClaudeError::Timeout),
                Err(e) => return Err(ClaudeError::Http(e)),
            }
        }
    }
}
```

**Step 3: Update lib.rs, run tests**

Add `pub mod ai;` to `src/lib.rs`.

Run: `cargo test ai::client -- -v`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add src/ai/mod.rs src/ai/client.rs src/lib.rs
git commit -m "feat: add Claude API client with reqwest (tool_use, retry, error handling)"
```

---

## Task 5: Tool Schemas & System Prompt

**Files:**
- Create: `src/ai/tools.rs`, `src/ai/prompts.rs`
- Modify: `src/ai/mod.rs`

**Step 1: Implement tool schemas**

Define the three tool schemas as functions returning `Tool` structs: `generate_macrocycle_skeleton`, `generate_mesocycle_plan`, `add_coach_notes`.

**Step 2: Implement system prompt**

The Coach Jan persona system prompt. Keep it in a constant string.

**Step 3: Tests**

- Verify tool schemas are valid JSON
- Verify system prompt is non-empty and contains key terms

**Step 4: Commit**

---

## Task 6: Context Assembly

**Files:**
- Create: `src/ai/context.rs`
- Modify: `src/ai/mod.rs`

**Step 1: Implement context builders**

- `build_macrocycle_context(profile, race_goal, ctl, weeks_until_race) -> String`
- `build_mesocycle_context(profile, mesocycle, ctl, available_types) -> String`

**Step 2: Tests**

- Verify context strings contain all required fields
- Verify date calculations (weeks until race)

**Step 3: Commit**

---

## Task 7: Plan Database Layer

**Files:**
- Create: `src/db/plans.rs`
- Modify: `src/db/mod.rs`

**Step 1: Write tests**

Test CRUD operations for macrocycles, mesocycles, planned_workouts using in-memory SQLite.

**Step 2: Implement**

- `create_macrocycle(pool, user_id, race_goal_id, start_date, end_date, target_ctl, coach_message) -> Macrocycle`
- `create_mesocycle(pool, macrocycle_id, ...) -> Mesocycle`
- `create_planned_workout(pool, ...) -> PlannedWorkout`
- `get_current_macrocycle(pool, user_id) -> Option<Macrocycle>`
- `get_mesocycles(pool, macrocycle_id) -> Vec<Mesocycle>`
- `get_planned_workouts(pool, mesocycle_id) -> Vec<PlannedWorkout>`
- `get_current_plan(pool, user_id) -> Option<(Macrocycle, Vec<Mesocycle>)>`

**Step 3: Run tests, commit**

---

## Task 8: Plan Generation Handlers

**Files:**
- Create: `src/ai/handlers.rs`
- Modify: `src/ai/mod.rs`

This is the orchestration layer that ties together the Claude client, tool schemas, context assembly, workout registry, validation, and database layer.

**Step 1: Implement `generate_skeleton`**

```rust
pub async fn generate_skeleton(
    client: &ClaudeClient,
    profile: &AthleteProfile,
    race_goal: &RaceGoal,
    ctl: f64,
) -> Result<MacroclyleSkeleton, PlanError>
```

**Step 2: Implement `confirm_and_generate_plan`**

```rust
pub async fn confirm_and_generate_plan(
    client: &ClaudeClient,
    pool: &SqlitePool,
    user_id: i64,
    skeleton: &MacroclyleSkeleton,
    profile: &AthleteProfile,
    race_goal: &RaceGoal,
    ctl: f64,
) -> Result<GeneratedPlan, PlanError>
```

Flow:
1. Persist macrocycle + mesocycles
2. Call Claude with `generate_mesocycle_plan` for first mesocycle
3. Parse, fill from registry
4. Call Claude with `add_coach_notes`
5. Validate
6. Retry on validation failure
7. Persist planned workouts

**Step 3: Tests with wiremock**

Mock the Claude API to return canned responses. Test the full flow.

**Step 4: Commit**

---

## Task 9: Plan API Endpoints

**Files:**
- Create: `src/api/plans.rs`
- Modify: `src/api/mod.rs`, `src/lib.rs`

**Step 1: Implement endpoints**

- `POST /api/plan/generate` — Takes `{ race_goal_id }`, calls `generate_skeleton`, returns skeleton
- `POST /api/plan/confirm` — Takes skeleton, calls `confirm_and_generate_plan`, returns full plan
- `GET /api/plan` — Returns current macrocycle with mesocycles

**Step 2: Wire into router**

Add `.nest("/api/plan", api::plans::router())` to `build_app()`.

Update `AppState` to include `ClaudeClient` (optional, for when API key is present).

**Step 3: Integration tests**

Test endpoints with wiremock for Claude API mocking.

**Step 4: Commit**

---

## Task 10: Frontend — Plan API Client & Hooks

**Files:**
- Create: `frontend/src/api/plan.ts`, `frontend/src/hooks/usePlan.ts`
- Modify: `frontend/src/api/types.ts`

**Step 1: Add types**

```typescript
// In types.ts
export interface MacrocycleSkeleton {
  target_ctl: number;
  coach_message: string;
  mesocycles: MesocycleSkeleton[];
}

export interface MesocycleSkeleton {
  sequence_number: number;
  phase: string;
  focus: string;
  load_weeks: number;
  recovery_weeks: number;
  target_volume_km: number;
}

export interface PlannedWorkout {
  id: number;
  scheduled_date: string;
  workout_type: string;
  duration_min: number | null;
  duration_category: string | null;
  description: string | null;
  coach_notes: string | null;
  expected_tss: number | null;
  target_hr_zones: string | null;
  target_pace_zones: string | null;
}

export interface GeneratedPlan {
  macrocycle: { id: number; start_date: string; end_date: string; target_ctl: number; };
  mesocycles: MesocycleSkeleton[];
  workouts: PlannedWorkout[];
}
```

**Step 2: API calls**

```typescript
// frontend/src/api/plan.ts
export async function generatePlan(raceGoalId: number): Promise<{ skeleton: MacrocycleSkeleton }> { ... }
export async function confirmPlan(skeleton: MacrocycleSkeleton): Promise<GeneratedPlan> { ... }
export async function getCurrentPlan(): Promise<GeneratedPlan | null> { ... }
```

**Step 3: Hooks**

```typescript
// frontend/src/hooks/usePlan.ts
export function useGeneratePlan() { ... }  // mutation
export function useConfirmPlan() { ... }   // mutation
export function useCurrentPlan() { ... }   // query
```

**Step 4: Commit**

---

## Task 11: Frontend — Onboarding Steps 6-7

**Files:**
- Modify: `frontend/src/pages/Onboarding.tsx`

**NOTE:** Before implementing, invoke `/frontend-design` skill for layout and UX decisions.

**Step 1: Add Step 6 — Plan Generation Loading**

After profile creation succeeds, transition to step 6 instead of redirecting to dashboard. Step 6 calls `POST /api/plan/generate` and shows a loading state with Coach Jan persona message.

**Step 2: Add Step 7 — Skeleton Review**

Display the macrocycle skeleton: list of mesocycles with phase, focus, duration, date ranges. Coach's overview message. "Confirm Plan" button → calls `POST /api/plan/confirm`. Second loading state while weekly plans generate. On success, redirect to dashboard.

**Step 3: Update step count and labels**

Update `STEP_LABELS` and `StepProgress` to include 7 steps.

**Step 4: Commit**

---

## Task 12: Frontend — Dashboard Plan Display

**Files:**
- Modify: `frontend/src/pages/Dashboard.tsx`

**NOTE:** Before implementing, invoke `/frontend-design` skill.

**Step 1: Wire up real plan data**

Replace the placeholder training plan card with actual data from `useCurrentPlan()`. Show the first week's workouts as a simple list: date, workout type display name, duration, target zones.

**Step 2: Wire up real race countdown**

Use the race goal data from the plan to show actual days until race.

**Step 3: Commit**

---

## Task 13: Integration Tests with Wiremock

**Files:**
- Modify: `tests/integration_tests.rs`
- Modify: `Cargo.toml` (add `wiremock` dev-dependency)

**Step 1: Add wiremock to dev-dependencies**

```toml
[dev-dependencies]
wiremock = "0.6"
```

**Step 2: Write test: full plan generation flow**

Test the full `POST /api/plan/generate` → `POST /api/plan/confirm` → `GET /api/plan` flow with mocked Claude API responses.

**Step 3: Write test: validation failure triggers retry**

Mock Claude to return an invalid plan first, then a valid one. Verify retry works.

**Step 4: Commit**

---

## Task 14: Live Integration Test

**Files:**
- Create: `tests/live_claude_test.rs`

**Step 1: Write live test**

```rust
// Only runs when ANTHROPIC_API_KEY is set
#[tokio::test]
#[ignore] // run with: cargo test -- --ignored
async fn test_real_claude_plan_generation() {
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => { eprintln!("Skipping: ANTHROPIC_API_KEY not set"); return; }
    };
    // Create client, send real request, verify response parses
}
```

**Step 2: Commit**

---

## Task 15: Phase Doc & Cleanup

**Files:**
- Create: `docs/phases/PHASE_2_PLAN_GENERATION.md`
- Modify: `docs/plans/2026-02-15-phase2-plan-generation-design.md` (update with corrected workout types)

**Step 1: Write phase completion doc**

Document what was built, how to test, completion checklist.

**Step 2: Update design doc**

Incorporate the corrected workout templates (VO2max 1-3min, anaerobic hills, track workouts, etc.)

**Step 3: Run full test suite**

Run: `cargo test && cd frontend && npm test`
Expected: All pass

**Step 4: Final commit**

---

## Task Summary

| Task | Component | Description |
|------|-----------|-------------|
| 1 | DB | Migration for macrocycles, mesocycles, planned_workouts |
| 2 | Domain | Workout registry with all templates |
| 3 | Domain | Plan validation rules |
| 4 | AI | Claude API client (reqwest) |
| 5 | AI | Tool schemas & system prompt |
| 6 | AI | Context assembly |
| 7 | DB | Plan CRUD operations |
| 8 | AI | Plan generation handlers (orchestration) |
| 9 | API | Plan endpoints (generate, confirm, get) |
| 10 | Frontend | Plan API client & hooks |
| 11 | Frontend | Onboarding steps 6-7 |
| 12 | Frontend | Dashboard plan display |
| 13 | Test | Integration tests with wiremock |
| 14 | Test | Live Claude integration test |
| 15 | Docs | Phase doc & cleanup |

**Dependencies:** Tasks 1-3 are independent. Task 4 is independent. Tasks 5-6 depend on 4. Task 7 depends on 1. Task 8 depends on 2-7. Task 9 depends on 8. Tasks 10-12 depend on 9. Tasks 13-14 depend on 9.
