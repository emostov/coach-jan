-- Add workout completion feedback fields and fix CHECK constraint
-- to include all 37 workout types.
-- SQLite cannot ALTER CHECK constraints, so we recreate the table.

-- Step 1: Create new table with updated schema
CREATE TABLE planned_workouts_new (
    id INTEGER PRIMARY KEY,
    mesocycle_id INTEGER NOT NULL REFERENCES mesocycles(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scheduled_date TEXT NOT NULL,
    workout_type TEXT NOT NULL CHECK (workout_type IN (
        'easy_run', 'long_run', 'long_run_progression', 'long_run_moderate',
        'aerobic_development', 'moderate_run', 'steady_run',
        'tempo_run', 'vo2max_intervals', 'under_over',
        'track_200m', 'track_400m', 'track_800m',
        'track_1200m', 'track_1600m', 'track_mixed',
        'track_mile_pace', 'track_race_combo',
        'anaerobic_hills', 'anaerobic_flat', 'anaerobic_power',
        'hill_sprints', 'race_specific',
        'fartlek_structured', 'cruise_intervals', 'progression_run',
        'lactate_clearance', 'mixed_energy',
        'shakeout_run', 'time_trial', 'form_drills', 'plyo_running',
        'recovery_run', 'rest',
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
    -- New completion feedback fields
    rpe INTEGER CHECK (rpe IS NULL OR (rpe >= 1 AND rpe <= 10)),
    athlete_notes TEXT,
    actual_duration_min INTEGER,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Step 2: Copy existing data
INSERT INTO planned_workouts_new (
    id, mesocycle_id, user_id, scheduled_date, workout_type,
    duration_min, duration_category, target_hr_zones, target_pace_zones,
    expected_tss, description, coach_notes, is_completed,
    completed_workout_id, created_at
)
SELECT
    id, mesocycle_id, user_id, scheduled_date, workout_type,
    duration_min, duration_category, target_hr_zones, target_pace_zones,
    expected_tss, description, coach_notes, is_completed,
    completed_workout_id, created_at
FROM planned_workouts;

-- Step 3: Drop old table and rename
DROP TABLE planned_workouts;
ALTER TABLE planned_workouts_new RENAME TO planned_workouts;

-- Step 4: Recreate index
CREATE INDEX idx_planned_workouts_user_date ON planned_workouts(user_id, scheduled_date);
