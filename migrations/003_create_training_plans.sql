-- Training plan structure: macrocycles -> mesocycles -> planned_workouts

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
        'easy_run', 'long_run', 'long_run_progression', 'long_run_moderate',
        'aerobic_development', 'moderate_run', 'steady_run',
        'tempo_run', 'vo2max_intervals', 'under_over',
        'track_200m', 'track_400m', 'track_800m',
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
