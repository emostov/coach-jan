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
    LongRunModerate,
    AerobicDevelopment,
    ModerateRun,
    SteadyRun,
    TempoRun,
    Vo2maxIntervals,
    UnderOver,
    Track200m,
    Track400m,
    Track800m,
    Track1200m,
    Track1600m,
    TrackMixed,
    TrackMilePace,
    TrackRaceCombo,
    AnaerobicHills,
    AnaerobicFlat,
    AnaerobicPower,
    HillSprints,
    RaceSpecific,
    FartlekStructured,
    CruiseIntervals,
    ProgressionRun,
    LactateClearance,
    MixedEnergy,
    ShakeoutRun,
    TimeTrial,
    FormDrills,
    PlyoRunning,
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
            Self::LongRunModerate => "long_run_moderate",
            Self::AerobicDevelopment => "aerobic_development",
            Self::ModerateRun => "moderate_run",
            Self::SteadyRun => "steady_run",
            Self::TempoRun => "tempo_run",
            Self::Vo2maxIntervals => "vo2max_intervals",
            Self::UnderOver => "under_over",
            Self::Track200m => "track_200m",
            Self::Track400m => "track_400m",
            Self::Track800m => "track_800m",
            Self::Track1200m => "track_1200m",
            Self::Track1600m => "track_1600m",
            Self::TrackMixed => "track_mixed",
            Self::TrackMilePace => "track_mile_pace",
            Self::TrackRaceCombo => "track_race_combo",
            Self::AnaerobicHills => "anaerobic_hills",
            Self::AnaerobicFlat => "anaerobic_flat",
            Self::AnaerobicPower => "anaerobic_power",
            Self::HillSprints => "hill_sprints",
            Self::RaceSpecific => "race_specific",
            Self::FartlekStructured => "fartlek_structured",
            Self::CruiseIntervals => "cruise_intervals",
            Self::ProgressionRun => "progression_run",
            Self::LactateClearance => "lactate_clearance",
            Self::MixedEnergy => "mixed_energy",
            Self::ShakeoutRun => "shakeout_run",
            Self::TimeTrial => "time_trial",
            Self::FormDrills => "form_drills",
            Self::PlyoRunning => "plyo_running",
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
            "long_run_moderate" => Some(Self::LongRunModerate),
            "aerobic_development" => Some(Self::AerobicDevelopment),
            "moderate_run" => Some(Self::ModerateRun),
            "steady_run" => Some(Self::SteadyRun),
            "tempo_run" => Some(Self::TempoRun),
            "vo2max_intervals" => Some(Self::Vo2maxIntervals),
            "under_over" => Some(Self::UnderOver),
            "track_200m" => Some(Self::Track200m),
            "track_400m" => Some(Self::Track400m),
            "track_800m" => Some(Self::Track800m),
            "track_1200m" => Some(Self::Track1200m),
            "track_1600m" => Some(Self::Track1600m),
            "track_mixed" => Some(Self::TrackMixed),
            "track_mile_pace" => Some(Self::TrackMilePace),
            "track_race_combo" => Some(Self::TrackRaceCombo),
            "anaerobic_hills" => Some(Self::AnaerobicHills),
            "anaerobic_flat" => Some(Self::AnaerobicFlat),
            "anaerobic_power" => Some(Self::AnaerobicPower),
            "hill_sprints" => Some(Self::HillSprints),
            "race_specific" => Some(Self::RaceSpecific),
            "fartlek_structured" => Some(Self::FartlekStructured),
            "cruise_intervals" => Some(Self::CruiseIntervals),
            "progression_run" => Some(Self::ProgressionRun),
            "lactate_clearance" => Some(Self::LactateClearance),
            "mixed_energy" => Some(Self::MixedEnergy),
            "shakeout_run" => Some(Self::ShakeoutRun),
            "time_trial" => Some(Self::TimeTrial),
            "form_drills" => Some(Self::FormDrills),
            "plyo_running" => Some(Self::PlyoRunning),
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
            Self::LongRunModerate => "Long Run w/ Moderate Finish",
            Self::AerobicDevelopment => "Aerobic Development",
            Self::ModerateRun => "Moderate Run",
            Self::SteadyRun => "Steady Run",
            Self::TempoRun => "Tempo Run",
            Self::Vo2maxIntervals => "VO2max Intervals",
            Self::UnderOver => "Under/Over Intervals",
            Self::Track200m => "Track 200m Repeats",
            Self::Track400m => "Track 400m Repeats",
            Self::Track800m => "Track 800m Repeats",
            Self::Track1200m => "Track 1200m Repeats",
            Self::Track1600m => "Track 1600m Repeats",
            Self::TrackMixed => "Mixed Track Intervals",
            Self::TrackMilePace => "Track Mile Pace Repeats",
            Self::TrackRaceCombo => "Track Race Combo",
            Self::AnaerobicHills => "Anaerobic Hill Repeats",
            Self::AnaerobicFlat => "Anaerobic Repeats",
            Self::AnaerobicPower => "Anaerobic Power",
            Self::HillSprints => "Hill Sprints",
            Self::RaceSpecific => "Race-Specific",
            Self::FartlekStructured => "Structured Fartlek",
            Self::CruiseIntervals => "Cruise Intervals",
            Self::ProgressionRun => "Progression Run",
            Self::LactateClearance => "Lactate Clearance Fartlek",
            Self::MixedEnergy => "Mixed Energy System",
            Self::ShakeoutRun => "Shakeout Run",
            Self::TimeTrial => "Time Trial",
            Self::FormDrills => "Form Drills",
            Self::PlyoRunning => "Plyometric Running Circuit",
            Self::RecoveryRun => "Recovery Run",
            Self::Rest => "Rest Day",
            Self::StrengthPrecision => "Strength (Precision)",
            Self::StrengthPerformance => "Strength (Performance)",
            Self::StrengthPower => "Strength (Power)",
        }
    }

    /// Coaching guide for this workout type — purpose, energy system, phase usage, progression, and athlete impact.
    /// Used in the Claude system prompt so the AI understands each workout's role.
    pub fn coaching_guide(&self) -> &'static str {
        match self {
            Self::EasyRun => "Foundation aerobic development. Zone 1-2 throughout. Builds slow-twitch capillary density, mitochondrial volume, and fat oxidation. 60-70% of weekly volume should be easy runs. No progression needed — increase duration as weekly volume grows. Safe for every day; primary tool for aerobically limited athletes.",

            Self::LongRun => "Primary aerobic capacity builder. Zone 1-2 throughout. Extends time on feet beyond daily runs to stimulate aerobic adaptations (capillarization, glycogen storage, fat oxidation). Single best workout for aerobic capacity. Progress duration by 10-15 min/week during load phases. One per week maximum. Critical for all athletes, especially aerobically limited.",

            Self::LongRunProgression => "Long run finishing at Zone 4-5 (anaerobic power effort) for last 2-3 miles. Trains finishing speed on fatigued legs. Utilization phase workout — do NOT use during capacity phase. Counts as both a long run and an intensity session. Progress by extending the fast finish segment.",

            Self::LongRunModerate => "Long run finishing at Zone 3-4 (moderate effort) for last 2-3 miles. Less demanding than progression variant. Good transition between pure easy long runs and race-specific long runs. Late capacity or early utilization phase. Progress by extending the moderate segment.",

            Self::AerobicDevelopment => "Easy run with 4-8 strides (20s fast, full recovery) at the end. The strides activate fast-twitch fibers without significant fatigue. Good for 1-2x per week on easy days. No TSS impact from strides. A lighter version of mixed_energy — this is 'strides at end of run' vs mixed_energy's 'bursts distributed throughout.'",

            Self::ModerateRun => "Easy run finishing at Zone 3 for the last portion. Slightly harder than easy_run, slightly easier than steady_run. Trains aerobic system at a sustainable moderate effort. Good for mid-week volume in build phases. Not an intensity session.",

            Self::SteadyRun => "Sustained Zone 3-4 effort — 'comfortably uncomfortable.' Trains aerobic utilization without the recovery cost of threshold work. Good for aerobic build phases. Not classified as intensity because it's sub-threshold, but it's harder than easy running. Use 1-2x per week in build phases. AVOID for aerobically limited athletes during capacity phase.",

            Self::TempoRun => "Sustained threshold effort at Zone 3-4. Develops aerobic utilization — trains the body to use a higher percentage of VO2max. Olbrecht warns: threshold work trades capacity for power. Use ONLY in utilization phase (2-6 weeks before race). Intensity session. Progress from 15 min sustained to 25 min sustained or split into repeats.",

            Self::Vo2maxIntervals => "High-intensity intervals at Zone 5 (1-3 min efforts) with 2-2.5 min jog recovery. Develops VO2max and aerobic power. Intensity session. Use in utilization phase primarily. Progress by adding reps, extending interval duration, or reducing recovery. Classic VO2max intervals are 'power/utilization' in Olbrecht's framework.",

            Self::UnderOver => "Alternating hard/easy intervals. Short (30/30s Billat protocol), Medium (60/60s), Long (under/over threshold at 95%/105% FTPace). Accumulates more time at VO2max than continuous running. Intensity session. 30/30s is most accessible; under/over threshold is most demanding. Progress from short to medium to long within a mesocycle.",

            Self::Track200m => "Near-max effort 200m repeats with full recovery (200m walk). Develops neuromuscular power, running economy at speed, and anaerobic capacity. Intensity session. Very high neural cost, low metabolic cost per rep. 6-10 reps. Progress by adding 2 reps per week. Thorough warmup with strides required. HIGH priority for anaerobically limited athletes.",

            Self::Track400m => "Hard effort 400m repeats with 400m jog recovery. Develops speed endurance and anaerobic power. Intensity session. More metabolically demanding than 200m reps. 6-10 reps. Progress by adding reps. Bridges the gap between pure speed (200m) and VO2max (800m) work.",

            Self::Track800m => "VO2max-anaerobic effort 800m repeats with 400m jog recovery. Develops aerobic power and lactate tolerance. Intensity session. 4-6 reps. More aerobic than 400m work but still demanding. Good transition workout between capacity and utilization phases.",

            Self::Track1200m => "1200m repeats at VO2max/3K-race effort (Zone 5) with 400m jog recovery (~3 min). Intensity session. The bread-and-butter workout for 3K athletes — 1200m is 40% of race distance, long enough to sustain VO2max but short enough for quality accumulation. 3-5 reps. Daniels considers 1200m the upper limit for I-pace reps. Work:rest ratio ~1:0.85-1:1. Progress by adding reps. Utilization phase for 3K athletes; late capacity as general VO2max stimulus. More sustained aerobic power than 800m repeats, less total volume than mile repeats.",

            Self::Track1600m => "Mile (1600m) repeats at I pace / Zone 4-5 with 400m jog recovery (~4 min). Intensity session. Classic aerobic power workout — the most universally prescribed distance-based VO2max interval. 3-5 reps. For milers, this IS race simulation at goal pace. For 3K-10K athletes, builds sustained aerobic power. More aerobic and less anaerobic than 800m/1200m repeats. Longer recovery needed than shorter track reps because rep duration is 4-7 min. Progress by adding reps. Utilization phase primarily.",

            Self::TrackMixed => "Mixed-distance track intervals combining different rep lengths in one session — pyramids (200-400-800-1200-800-400-200), cut-downs (1200-800-400-200 getting faster), ladders, and alternating-distance sets (e.g., 3x(1200m+2x400m)). Intensity session. The MOST IMPORTANT track workout for mile/3K athletes. Trains multiple energy systems simultaneously: short reps (200-400m) build anaerobic capacity/speed (like Olbrecht's 'high'), long reps (800-1200m) build aerobic power/utilization. Also teaches pace-change ability critical for tactical racing. Simulates surges, kicks, and tactical moves. For milers: cut-downs and combos with reps at R and I pace. For 3K: pyramids and ladders at I pace. Late capacity through utilization phase. Progress from simple pyramids → complex cut-downs → race-specific combos.",

            Self::TrackMilePace => "Uniform-distance track repeats at mile/R pace (Daniels' Repetition pace = ~current mile race pace per 400m) with full recovery. Intensity session. Distances: 300m (6-8 reps), 400m (4-6 reps), 500m (3-5 reps), 600m (3-4 reps), or 800m (2-3 reps). Work:rest 1:2-1:3 — recovery is generous because the goal is PACE QUALITY, not fatigue accumulation. Total work volume: 1200-2400m (75-150% of mile race distance). This is the speed endurance zone — 40-90s efforts where the glycolytic system is maximally stressed. Distinct from track_200m (pure speed, <30s) and track_800m (VO2max, 2+ min). For milers: the primary race-specific workout. For 3K athletes: develops finishing speed and surge capacity. Utilization phase only. HIGH priority for anaerobically limited athletes targeting mile/3K.",

            Self::TrackRaceCombo => "Short mixed-distance track combos at race pace, often including a warm-up set. Intensity session. Examples: 3x400+2x200, 800+3x400+2x200, 600+400+300+200 cut-down at mile pace, 2x(400+200) sets. Total work volume: 1200-2400m. ALL reps at goal race pace or faster — this is race simulation, not multi-system training. Recovery between reps varies: 200m jog between short reps, 3-4 min between sets. For race simulations (broken mile), use SHORT rest (60-120s) to create race-like lactate. Distinct from track_mixed: lower total volume, all reps at race pace (not mixed I/R pace), and focused on mile/3K race preparation specifically. Late utilization phase and final sharpening (last 2-3 weeks before goal race). The warm-up set (e.g., 800m at 3K pace before the main set) primes the aerobic system before race-pace work.",

            Self::AnaerobicHills => "Hill repeats at 30s-1:15 duration (glycolytic system). Builds anaerobic capacity with reduced injury risk vs flat sprints. Intensity session. Hills reduce eccentric loading — the incline prevents top-end velocity that causes hamstring strains. Walk-down recovery. Progress by adding reps and extending duration. HIGH priority for anaerobically limited athletes, LOW for aerobically limited.",

            Self::AnaerobicFlat => "Flat sprint repeats at 20-45s (glycolytic system). Builds anaerobic capacity. Intensity session. Higher injury risk than hill version — hamstrings must decelerate the leg at high velocity. Use only when hills are unavailable or athlete is advanced. Progress by adding reps and extending duration.",

            Self::AnaerobicPower => "Sustained hard efforts (5-30 min) at Zone 4-5 developing anaerobic power. Intensity session. Longer than sprint work, shorter than threshold. Good for race-specific anaerobic preparation. Progress by extending interval duration or adding reps.",

            Self::HillSprints => "Short maximal hill sprints (8-12s) for neuromuscular power. Uses ATP-PCr system — does NOT produce lactate. NOT an intensity session. Can be done the day before hard workouts (Hudson/Canova protocol). Expands the recruitable fiber pool, making subsequent training more effective. Does NOT count toward the 10-15% high-intensity budget. Progress reps first (2→10), then duration (8s→12s), then gradient (6%→10%). Max 10 reps. 1-2x per week year-round.",

            Self::RaceSpecific => "Intervals at goal race pace. Trains race-day pacing, fueling, and mental rehearsal. Intensity session. Utilization phase only. Structure varies by race distance (marathon: longer reps, 5K: shorter reps at faster pace). Progress by extending total race-pace volume.",

            Self::FartlekStructured => "Continuous run with predetermined hard/easy segments (never stopping). Zone 4-5 hard / Zone 2 easy. Intensity session. Trains lactate clearance during movement, surge handling, mental toughness. Kenyan-style: classic formats are 8x1min/1min, 12x1min/1min, 15x2min/1min. Progress by adding reps or shifting work:recovery ratio from 1:1 to 2:1. Utilization phase primarily.",

            Self::CruiseIntervals => "Threshold-pace repeats (1000m-1 mile) with very brief recovery (60-90s jog). Intensity session. Key utilization workout. Accumulates more threshold-pace volume than sustained tempo because brief rests prevent premature fatigue. The short recovery is critical — lactate must NOT fully clear. Progress by adding reps, extending rep distance, or reducing recovery. Use ONLY in utilization phase (2-6 weeks before race).",

            Self::ProgressionRun => "Start easy, progressively increase pace. Three variants: 'thirds' (each third faster), 'fast finish' (easy 75% then tempo 25%), 'race simulation' (easy 85% then race pace 15%). NOT an intensity session. Less fatiguing than tempo while still providing utilization stimulus. Teaches finishing speed on tired legs. Late capacity through utilization phase. Progress by extending time at higher intensity.",

            Self::LactateClearance => "Surges at Zone 5-6 with moderate-pace recovery at Zone 3 (NOT easy). Intensity session. The moderate recovery is the key — it keeps lactate elevated, training the body's clearance mechanisms. Olbrecht: lactate is fuel, not waste. 'Create lactate, then use it as fuel.' Late utilization phase only (3-5 weeks before race). Requires strong aerobic base. Progress by adding reps or extending surge duration.",

            Self::MixedEnergy => "Olbrecht's signature 'high and low' capacity builder. Easy running (60%) with short explosive 30s bursts (40%) at Zone 6-7, distributed throughout the run. NOT an intensity session despite the Zone 6-7 bursts — bursts are too short for significant metabolic load. PRIMARY capacity-building workout. Activates mitochondria in BOTH slow and fast-twitch fibers simultaneously. 1-2x per week during capacity phase. Progress by adding bursts (6→10) or extending total session time.",

            Self::ShakeoutRun => "Very short, very easy run (Zone 1) for neuromuscular activation before a race or hard workout. 10-30 min. NOT an intensity session. No progression — fixed-purpose workout. Longer races get shorter shakeouts (marathon: 10 min), shorter races get longer ones with strides (5K: 20-30 min with strides). Race week only.",

            Self::TimeTrial => "All-out effort over 3K, 5K, or 30 min to measure current fitness and calibrate FTPace/LTHR. Intensity session. Assessment, not training stimulus. Schedule at mesocycle boundaries when athlete is fresh (during recovery phase). Every 4-8 weeks. After completion, prompt athlete for zone recalibration if results suggest FTPace/LTHR has changed.",

            Self::FormDrills => "Running-specific drills (A-skips, B-skips, high knees, butt kicks, bounding) for economy and neuromuscular coordination. NOT an intensity session. All phases, 1-2x per week. Bridges gap between Running Rewired strength and running-specific neural activation. NOT a replacement for strength sessions. Progress from 2 sets to 3 sets, add more dynamic exercises, add strides.",

            Self::PlyoRunning => "Hybrid session: easy running with plyometric exercises (box jumps, squat jumps, bounding, single-leg hops). Develops elastic energy storage and return — the 'spring' in running. NOT an intensity session. Build phase only (after 6-8 weeks of strength foundation). 1x per week, replaces one strength session. Progress by adding rounds and more challenging exercises.",

            Self::RecoveryRun => "Very easy Zone 1 run for active recovery. 20-30 min. NOT an intensity session. Promotes blood flow and speeds recovery between hard sessions. Slower than easy runs — should feel effortless. Good for day after intensity sessions.",

            Self::Rest => "Complete rest. No running. Essential for adaptation — fitness improves during rest, not during training. At least 1 rest day per week. Can be replaced by recovery_run for advanced athletes if fatigue markers are low.",

            Self::StrengthPrecision => "Running Rewired Phase 1 — precision and alignment exercises. Develops single-leg stability, hip alignment, and movement quality. Foundation for all subsequent strength work. Start here for first 4-6 weeks. 2-3x per week.",

            Self::StrengthPerformance => "Running Rewired Phase 2 — performance exercises. Builds on precision phase with more load and complexity. Develops running-specific strength. After 4-6 weeks of precision phase. 2x per week.",

            Self::StrengthPower => "Running Rewired Phase 3 — power and plyometric exercises. Develops rate of force development and elastic recoil. Most demanding strength phase. After establishing performance base. 1-2x per week. Don't schedule on same day as track/VO2max/speed work.",
        }
    }

    /// Whether this workout counts as a high-intensity session for plan validation.
    pub fn is_intensity(&self) -> bool {
        matches!(
            self,
            Self::Vo2maxIntervals
                | Self::UnderOver
                | Self::Track200m
                | Self::Track400m
                | Self::Track800m
                | Self::Track1200m
                | Self::Track1600m
                | Self::TrackMixed
                | Self::TrackMilePace
                | Self::TrackRaceCombo
                | Self::AnaerobicHills
                | Self::AnaerobicFlat
                | Self::AnaerobicPower
                | Self::RaceSpecific
                | Self::TempoRun
                | Self::FartlekStructured
                | Self::CruiseIntervals
                | Self::LactateClearance
                | Self::TimeTrial
        )
    }

    /// All workout types.
    pub fn all() -> Vec<Self> {
        vec![
            Self::EasyRun, Self::LongRun, Self::LongRunProgression, Self::LongRunModerate,
            Self::AerobicDevelopment, Self::ModerateRun, Self::SteadyRun,
            Self::TempoRun, Self::Vo2maxIntervals, Self::UnderOver,
            Self::Track200m, Self::Track400m, Self::Track800m,
            Self::Track1200m, Self::Track1600m, Self::TrackMixed,
            Self::TrackMilePace, Self::TrackRaceCombo,
            Self::AnaerobicHills, Self::AnaerobicFlat, Self::AnaerobicPower,
            Self::HillSprints, Self::RaceSpecific,
            Self::FartlekStructured, Self::CruiseIntervals, Self::ProgressionRun,
            Self::LactateClearance, Self::MixedEnergy,
            Self::ShakeoutRun, Self::TimeTrial, Self::FormDrills, Self::PlyoRunning,
            Self::RecoveryRun, Self::Rest,
            Self::StrengthPrecision, Self::StrengthPerformance, Self::StrengthPower,
        ]
    }

    /// Running workout types only (have templates in the registry).
    pub fn all_running() -> Vec<Self> {
        vec![
            Self::EasyRun, Self::LongRun, Self::LongRunProgression, Self::LongRunModerate,
            Self::AerobicDevelopment, Self::ModerateRun, Self::SteadyRun,
            Self::TempoRun, Self::Vo2maxIntervals, Self::UnderOver,
            Self::Track200m, Self::Track400m, Self::Track800m,
            Self::Track1200m, Self::Track1600m, Self::TrackMixed,
            Self::TrackMilePace, Self::TrackRaceCombo,
            Self::AnaerobicHills, Self::AnaerobicFlat, Self::AnaerobicPower,
            Self::HillSprints, Self::RaceSpecific,
            Self::FartlekStructured, Self::CruiseIntervals, Self::ProgressionRun,
            Self::LactateClearance, Self::MixedEnergy,
            Self::ShakeoutRun, Self::TimeTrial, Self::FormDrills, Self::PlyoRunning,
            Self::RecoveryRun,
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

        // Long Run with Moderate Finish: last 2-3 miles at Zone 3-4 (aerobic build)
        templates.insert(WorkoutType::LongRunModerate, WorkoutTemplate {
            workout_type: WorkoutType::LongRunModerate,
            description: "Long run with last 2-3 miles at moderate effort (Zone 3-4)",
            target_hr_zones: vec![1, 2, 3],
            target_pace_zones: vec![1, 2, 3],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 75,
                    structure: "60 min easy @ Zone 1-2, then 15 min @ Zone 3 (moderate)",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 95,
                    structure: "75 min easy @ Zone 1-2, then 20 min @ Zone 3 (moderate)",
                    expected_tss_min: 90.0, expected_tss_max: 115.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 120,
                    structure: "95 min easy @ Zone 1-2, then 25 min @ Zone 3-4 (moderate-steady)",
                    expected_tss_min: 115.0, expected_tss_max: 150.0,
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

        // Moderate Run: progressive easy run finishing last portion at Zone 3
        templates.insert(WorkoutType::ModerateRun, WorkoutTemplate {
            workout_type: WorkoutType::ModerateRun,
            description: "Easy run with last portion at moderate effort (Zone 3)",
            target_hr_zones: vec![2, 3],
            target_pace_zones: vec![2, 3],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "25 min easy @ Zone 2, then 10 min @ Zone 3",
                    expected_tss_min: 30.0, expected_tss_max: 42.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "30 min easy @ Zone 2, then 20 min @ Zone 3",
                    expected_tss_min: 48.0, expected_tss_max: 62.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 60,
                    structure: "35 min easy @ Zone 2, then 25 min @ Zone 3",
                    expected_tss_min: 58.0, expected_tss_max: 75.0,
                }),
            ]),
        });

        // Steady Run: sustained moderate-hard effort at Zone 3-4
        templates.insert(WorkoutType::SteadyRun, WorkoutTemplate {
            workout_type: WorkoutType::SteadyRun,
            description: "Sustained moderate-hard effort (Zone 3-4) — comfortably uncomfortable",
            target_hr_zones: vec![3, 4],
            target_pace_zones: vec![3, 4],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "35 min sustained @ Zone 3-4",
                    expected_tss_min: 42.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "45 min sustained @ Zone 3-4",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "55 min sustained @ Zone 3-4",
                    expected_tss_min: 68.0, expected_tss_max: 85.0,
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
                    expected_tss_min: 65.0, expected_tss_max: 80.0,
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

        // Under/Over Intervals: alternating hard/easy (30/30, 60/60, under/over threshold)
        templates.insert(WorkoutType::UnderOver, WorkoutTemplate {
            workout_type: WorkoutType::UnderOver,
            description: "Alternating fast/slow intervals (30/30s, 60/60s) at VO2max effort",
            target_hr_zones: vec![4, 5, 6],
            target_pace_zones: vec![4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 15x30s hard Zone 5-6 / 30s easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 20x30s hard Zone 5-6 / 30s easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 15x60s hard Zone 5 / 60s easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 90.0,
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

        // Track 1200m
        templates.insert(WorkoutType::Track1200m, WorkoutTemplate {
            workout_type: WorkoutType::Track1200m,
            description: "Track 1200m repeats at VO2max/3K-race effort",
            target_hr_zones: vec![5],
            target_pace_zones: vec![5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 3x1200m @ Zone 5 / 400m jog (~3 min) + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 4x1200m @ Zone 5 / 400m jog (~3.5 min) + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 5x1200m @ Zone 5 / 400m jog (~3.5 min) + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 95.0,
                }),
            ]),
        });

        // Track 1600m (Mile Repeats)
        templates.insert(WorkoutType::Track1600m, WorkoutTemplate {
            workout_type: WorkoutType::Track1600m,
            description: "Track mile (1600m) repeats at I pace / VO2max effort",
            target_hr_zones: vec![4, 5],
            target_pace_zones: vec![4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 3x1600m @ Zone 4-5 / 400m jog (~4 min) + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 60,
                    structure: "10 min warmup + 4x1600m @ Zone 4-5 / 400m jog (~4 min) + 10 min cooldown",
                    expected_tss_min: 75.0, expected_tss_max: 95.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 70,
                    structure: "10 min warmup + 5x1600m @ Zone 4-5 / 400m jog (~4 min) + 10 min cooldown",
                    expected_tss_min: 85.0, expected_tss_max: 110.0,
                }),
            ]),
        });

        // Track Mixed (Ladders/Cut-Downs/Pyramids)
        templates.insert(WorkoutType::TrackMixed, WorkoutTemplate {
            workout_type: WorkoutType::TrackMixed,
            description: "Mixed-distance track intervals — pyramids, cut-downs, ladders, alternating sets",
            target_hr_zones: vec![4, 5, 6],
            target_pace_zones: vec![4, 5, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 200-400-800-400-200 @ descending pace (R→I→R), 200m jog between + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 400-800-1200-800-400 @ I pace (short reps faster), 400m jog between + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 60,
                    structure: "10 min warmup + 200-400-800-1200-800-400-200 full pyramid, 200-400m jog between + 10 min cooldown",
                    expected_tss_min: 70.0, expected_tss_max: 95.0,
                }),
            ]),
        });

        // Track Mile Pace Repeats
        templates.insert(WorkoutType::TrackMilePace, WorkoutTemplate {
            workout_type: WorkoutType::TrackMilePace,
            description: "Uniform-distance track repeats at mile/R pace with generous recovery for pace quality",
            target_hr_zones: vec![5, 6],
            target_pace_zones: vec![5, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 6x300m @ R/mile pace / 300m jog (~2 min) + 10 min cooldown",
                    expected_tss_min: 35.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 4x400m @ R/mile pace / 400m jog (~3 min) + 10 min cooldown",
                    expected_tss_min: 40.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 2x800m @ mile pace / 5 min rest + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 65.0,
                }),
            ]),
        });

        // Track Race Combo
        templates.insert(WorkoutType::TrackRaceCombo, WorkoutTemplate {
            workout_type: WorkoutType::TrackRaceCombo,
            description: "Short mixed-distance track combos at race pace with optional warm-up set",
            target_hr_zones: vec![5, 6],
            target_pace_zones: vec![5, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 3x400 + 2x200 @ mile pace, 400m/200m jog + 10 min cooldown",
                    expected_tss_min: 35.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 800m @ 3K pace + 3x400 + 2x200 @ mile pace + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 65.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 800m @ 3K pace + 800-600-400-200 cut-down @ mile→faster + 10 min cooldown",
                    expected_tss_min: 50.0, expected_tss_max: 70.0,
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

        // Hill Sprints: 8-12s neuromuscular (ATP-PCr), distinct from anaerobic_hills
        templates.insert(WorkoutType::HillSprints, WorkoutTemplate {
            workout_type: WorkoutType::HillSprints,
            description: "Short maximal hill sprints for neuromuscular power (ATP-PCr, not glycolytic)",
            target_hr_zones: vec![6, 7],
            target_pace_zones: vec![6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 25,
                    structure: "10 min easy warmup + 4x8s max hill sprint / 2 min walk back + 5 min easy cooldown",
                    expected_tss_min: 10.0, expected_tss_max: 15.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 30,
                    structure: "10 min easy warmup + 6x10s max hill sprint / 2 min walk back + 5 min easy cooldown",
                    expected_tss_min: 15.0, expected_tss_max: 25.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min easy warmup + 8-10x10s max hill sprint / 2 min walk back + 5 min easy cooldown",
                    expected_tss_min: 20.0, expected_tss_max: 30.0,
                }),
            ]),
        });

        // Structured Fartlek: continuous run with predetermined hard/easy segments
        templates.insert(WorkoutType::FartlekStructured, WorkoutTemplate {
            workout_type: WorkoutType::FartlekStructured,
            description: "Continuous run with structured hard/easy segments (never stopping)",
            target_hr_zones: vec![2, 3, 4, 5],
            target_pace_zones: vec![2, 3, 4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 40,
                    structure: "10 min warmup + 8x1 min hard Zone 4-5 / 1 min easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "10 min warmup + 12x1 min hard Zone 4-5 / 1 min easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 60.0, expected_tss_max: 80.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 65,
                    structure: "10 min warmup + 15x2 min hard Zone 4-5 / 1 min easy Zone 2 + 10 min cooldown",
                    expected_tss_min: 80.0, expected_tss_max: 105.0,
                }),
            ]),
        });

        // Cruise Intervals: threshold-pace repeats with brief recovery
        templates.insert(WorkoutType::CruiseIntervals, WorkoutTemplate {
            workout_type: WorkoutType::CruiseIntervals,
            description: "Repeated threshold-pace segments with brief recovery (60-90s jog)",
            target_hr_zones: vec![4],
            target_pace_zones: vec![3, 4],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 3x1000m @ Zone 4 / 60s jog + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 4x1200m @ Zone 4 / 75s jog + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 5x1 mile @ Zone 4 / 90s jog + 10 min cooldown",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Progression Run: start easy, progressively increase pace
        templates.insert(WorkoutType::ProgressionRun, WorkoutTemplate {
            workout_type: WorkoutType::ProgressionRun,
            description: "Start at easy effort and progressively increase pace throughout the run",
            target_hr_zones: vec![2, 3, 4],
            target_pace_zones: vec![2, 3, 4],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "25 min Zone 2 -> 10 min Zone 3",
                    expected_tss_min: 35.0, expected_tss_max: 45.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 50,
                    structure: "20 min Zone 2 -> 15 min Zone 3 -> 15 min Zone 4",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 60,
                    structure: "25 min Zone 2 -> 20 min Zone 3 -> 15 min Zone 4",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Lactate Clearance Fartlek: surges with moderate-pace recovery
        templates.insert(WorkoutType::LactateClearance, WorkoutTemplate {
            workout_type: WorkoutType::LactateClearance,
            description: "Surges that elevate lactate with moderate-pace recovery to train clearance",
            target_hr_zones: vec![3, 5, 6],
            target_pace_zones: vec![3, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min warmup + 6x(30s hard Zone 5-6 + 90s moderate Zone 3) + 10 min cooldown",
                    expected_tss_min: 45.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min warmup + 8x(40s hard Zone 5-6 + 2 min moderate Zone 3) + 10 min cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 70.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "10 min warmup + 10x(45s hard Zone 5-6 + 2 min moderate Zone 3) + 10 min cooldown",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Mixed Energy System: Olbrecht's "high and low" capacity builder
        templates.insert(WorkoutType::MixedEnergy, WorkoutTemplate {
            workout_type: WorkoutType::MixedEnergy,
            description: "Olbrecht's 'high and low' — easy running with short explosive 30s bursts distributed throughout",
            target_hr_zones: vec![1, 2, 6, 7],
            target_pace_zones: vec![1, 2, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 30,
                    structure: "18 min easy Zone 1-2 + 6x30s hard Zone 6-7 with 2.5 min easy between",
                    expected_tss_min: 30.0, expected_tss_max: 45.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 40,
                    structure: "24 min easy Zone 1-2 + 8x30s hard Zone 6-7 with 2.5 min easy between",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 50,
                    structure: "30 min easy Zone 1-2 + 10x30s hard Zone 6-7 with 2.5 min easy between",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
            ]),
        });

        // Shakeout Run: very short, very easy pre-race/pre-workout activation
        templates.insert(WorkoutType::ShakeoutRun, WorkoutTemplate {
            workout_type: WorkoutType::ShakeoutRun,
            description: "Very short, very easy run for neuromuscular activation before a race or hard workout",
            target_hr_zones: vec![1],
            target_pace_zones: vec![1],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 10,
                    structure: "10-15 min @ Zone 1",
                    expected_tss_min: 5.0, expected_tss_max: 10.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 20,
                    structure: "15-20 min @ Zone 1",
                    expected_tss_min: 10.0, expected_tss_max: 15.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 30,
                    structure: "20-30 min @ Zone 1 with 2-4 strides",
                    expected_tss_min: 12.0, expected_tss_max: 20.0,
                }),
            ]),
        });

        // Time Trial: all-out effort for zone calibration
        templates.insert(WorkoutType::TimeTrial, WorkoutTemplate {
            workout_type: WorkoutType::TimeTrial,
            description: "All-out effort over a set distance to measure fitness and calibrate training zones",
            target_hr_zones: vec![4, 5, 6],
            target_pace_zones: vec![4, 5],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "15 min easy warmup + 4 strides + 3K all-out effort + 10 min easy cooldown",
                    expected_tss_min: 40.0, expected_tss_max: 55.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "15 min easy warmup + 4 strides + 5K all-out effort + 10 min easy cooldown",
                    expected_tss_min: 55.0, expected_tss_max: 75.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "15 min easy warmup + 4 strides + 30-min time trial (best effort) + 10 min easy cooldown",
                    expected_tss_min: 65.0, expected_tss_max: 85.0,
                }),
            ]),
        });

        // Form Drills: running-specific drills for economy and coordination
        templates.insert(WorkoutType::FormDrills, WorkoutTemplate {
            workout_type: WorkoutType::FormDrills,
            description: "Running-specific drills (A-skips, B-skips, high knees, bounding) for economy and coordination",
            target_hr_zones: vec![1, 2],
            target_pace_zones: vec![1, 2, 5, 6],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 30,
                    structure: "10 min easy jog + 15 min drill circuit (A-skip, B-skip, high knees, butt kicks, bounding 2x30m each) + 5 min easy jog",
                    expected_tss_min: 15.0, expected_tss_max: 25.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 40,
                    structure: "15 min easy jog + 20 min drill circuit (full drill set 3x30m each + 4 strides) + 5 min easy jog",
                    expected_tss_min: 25.0, expected_tss_max: 35.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 45,
                    structure: "15 min easy jog + 20 min drill circuit + 4x100m strides + 10 min easy jog",
                    expected_tss_min: 30.0, expected_tss_max: 45.0,
                }),
            ]),
        });

        // Plyometric Running Circuit: easy running with bodyweight plyometrics
        templates.insert(WorkoutType::PlyoRunning, WorkoutTemplate {
            workout_type: WorkoutType::PlyoRunning,
            description: "Hybrid session combining easy running with plyometric exercises for elastic energy development",
            target_hr_zones: vec![1, 2, 3],
            target_pace_zones: vec![1, 2],
            durations: HashMap::from([
                (DurationCategory::Short, DurationParams {
                    total_duration_min: 35,
                    structure: "10 min easy jog + 3 rounds (6 box jumps + 8 squat jumps + 4x30m bounding + 200m easy jog) + 10 min easy jog",
                    expected_tss_min: 25.0, expected_tss_max: 35.0,
                }),
                (DurationCategory::Medium, DurationParams {
                    total_duration_min: 45,
                    structure: "10 min easy jog + 4 rounds (8 box jumps + 10 squat jumps + 6x30m bounding + 2x30m single-leg hops + 200m easy jog) + 10 min easy jog",
                    expected_tss_min: 35.0, expected_tss_max: 50.0,
                }),
                (DurationCategory::Long, DurationParams {
                    total_duration_min: 55,
                    structure: "15 min easy jog + 5 rounds (full plyo circuit + 400m easy jog between) + 10 min easy jog",
                    expected_tss_min: 45.0, expected_tss_max: 60.0,
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
        _pace_zones: Option<&crate::domain::types::PaceZones>,
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
    fn new_mile_3k_types_exist_and_roundtrip() {
        let new_types = vec![
            ("track_1200m", WorkoutType::Track1200m),
            ("track_1600m", WorkoutType::Track1600m),
            ("track_mixed", WorkoutType::TrackMixed),
            ("track_mile_pace", WorkoutType::TrackMilePace),
            ("track_race_combo", WorkoutType::TrackRaceCombo),
        ];
        for (key, expected) in &new_types {
            let parsed = WorkoutType::from_str(key)
                .unwrap_or_else(|| panic!("Failed to parse: {}", key));
            assert_eq!(parsed, *expected);
            assert_eq!(parsed.as_str(), *key);
        }
    }

    #[test]
    fn new_mile_3k_types_have_display_names() {
        assert_eq!(WorkoutType::Track1200m.display_name(), "Track 1200m Repeats");
        assert_eq!(WorkoutType::Track1600m.display_name(), "Track 1600m Repeats");
        assert_eq!(WorkoutType::TrackMixed.display_name(), "Mixed Track Intervals");
        assert_eq!(WorkoutType::TrackMilePace.display_name(), "Track Mile Pace Repeats");
        assert_eq!(WorkoutType::TrackRaceCombo.display_name(), "Track Race Combo");
    }

    #[test]
    fn new_mile_3k_types_are_intensity() {
        assert!(WorkoutType::Track1200m.is_intensity());
        assert!(WorkoutType::Track1600m.is_intensity());
        assert!(WorkoutType::TrackMixed.is_intensity());
        assert!(WorkoutType::TrackMilePace.is_intensity());
        assert!(WorkoutType::TrackRaceCombo.is_intensity());
    }

    #[test]
    fn new_mile_3k_types_have_coaching_guides() {
        let guide = WorkoutType::Track1200m.coaching_guide();
        assert!(guide.contains("3K"), "track_1200m guide should reference 3K");

        let guide = WorkoutType::Track1600m.coaching_guide();
        assert!(guide.contains("aerobic power"), "track_1600m guide should reference aerobic power");

        let guide = WorkoutType::TrackMixed.coaching_guide();
        assert!(guide.contains("pyramid") || guide.contains("ladder") || guide.contains("cut-down"),
            "track_mixed guide should reference session formats");

        let guide = WorkoutType::TrackMilePace.coaching_guide();
        assert!(guide.contains("mile") || guide.contains("R pace"),
            "track_mile_pace guide should reference mile/R pace");

        let guide = WorkoutType::TrackRaceCombo.coaching_guide();
        assert!(guide.contains("combo") || guide.contains("mixed"),
            "track_race_combo guide should reference combo/mixed format");
    }

    #[test]
    fn new_mile_3k_types_in_all_running() {
        let all = WorkoutType::all_running();
        assert!(all.contains(&WorkoutType::Track1200m));
        assert!(all.contains(&WorkoutType::Track1600m));
        assert!(all.contains(&WorkoutType::TrackMixed));
        assert!(all.contains(&WorkoutType::TrackMilePace));
        assert!(all.contains(&WorkoutType::TrackRaceCombo));
    }

    #[test]
    fn is_intensity_session_correct() {
        assert!(WorkoutType::Vo2maxIntervals.is_intensity());
        assert!(WorkoutType::UnderOver.is_intensity());
        assert!(WorkoutType::Track200m.is_intensity());
        assert!(WorkoutType::Track400m.is_intensity());
        assert!(WorkoutType::Track800m.is_intensity());
        assert!(WorkoutType::Track1200m.is_intensity());
        assert!(WorkoutType::Track1600m.is_intensity());
        assert!(WorkoutType::TrackMixed.is_intensity());
        assert!(WorkoutType::TrackMilePace.is_intensity());
        assert!(WorkoutType::TrackRaceCombo.is_intensity());
        assert!(WorkoutType::AnaerobicHills.is_intensity());
        assert!(WorkoutType::AnaerobicFlat.is_intensity());
        assert!(WorkoutType::AnaerobicPower.is_intensity());
        assert!(WorkoutType::RaceSpecific.is_intensity());
        assert!(WorkoutType::TempoRun.is_intensity());
        assert!(WorkoutType::FartlekStructured.is_intensity());
        assert!(WorkoutType::CruiseIntervals.is_intensity());
        assert!(WorkoutType::LactateClearance.is_intensity());
        assert!(WorkoutType::TimeTrial.is_intensity());
        assert!(!WorkoutType::EasyRun.is_intensity());
        assert!(!WorkoutType::LongRun.is_intensity());
        assert!(!WorkoutType::LongRunModerate.is_intensity());
        assert!(!WorkoutType::ModerateRun.is_intensity());
        assert!(!WorkoutType::SteadyRun.is_intensity());
        assert!(!WorkoutType::RecoveryRun.is_intensity());
        assert!(!WorkoutType::Rest.is_intensity());
        assert!(!WorkoutType::HillSprints.is_intensity());
        assert!(!WorkoutType::ProgressionRun.is_intensity());
        assert!(!WorkoutType::MixedEnergy.is_intensity());
        assert!(!WorkoutType::ShakeoutRun.is_intensity());
        assert!(!WorkoutType::FormDrills.is_intensity());
        assert!(!WorkoutType::PlyoRunning.is_intensity());
    }
}
