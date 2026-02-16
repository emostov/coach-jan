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

        // Easy Run
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

        // Long Run
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

        // Long Run Progression
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

        // Aerobic Development
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

        // Tempo Run
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

        // VO2max Intervals
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

        // Track 200m
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

        // Track 400m
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

        // Track 800m
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

        // Anaerobic Hills
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

        // Anaerobic Flat
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

        // Anaerobic Power
        templates.insert(WorkoutType::AnaerobicPower, WorkoutTemplate {
            workout_type: WorkoutType::AnaerobicPower,
            description: "Sustained hard efforts developing anaerobic power (5-30 min efforts)",
            target_hr_zones: vec![4, 5],
            target_pace_zones: vec![4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 3x5 min @ Zone 4-5 / 3 min jog + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 3x8 min @ Zone 4-5 / 3 min jog + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 90.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 65,
                    structure: "10 min warmup + 2x15 min @ Zone 4-5 / 5 min jog + 10 min cooldown",
                    expected_tss_min: 80.0, expected_tss_max: 110.0,
                }),
            ]),
        });

        // Race-Specific
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_workout_types_have_templates() {
        let registry = WorkoutRegistry::new();
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
