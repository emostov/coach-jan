-- Athlete profiles
CREATE TABLE athlete_profiles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    weight_kg REAL NOT NULL,
    resting_hr INTEGER NOT NULL,
    max_hr INTEGER NOT NULL,
    lthr INTEGER NOT NULL,
    ftpace_m_per_s REAL,
    current_weekly_volume_km REAL NOT NULL,
    experience_level TEXT NOT NULL CHECK (experience_level IN ('beginner', 'intermediate', 'advanced')),
    sports_background TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Physiological history
CREATE TABLE ftpace_history (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    pace_m_per_s REAL NOT NULL,
    source TEXT NOT NULL CHECK (source IN ('race', 'time_trial', 'estimate', 'workout_derived')),
    recorded_at TEXT NOT NULL,
    notes TEXT
);

CREATE TABLE lthr_history (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lthr INTEGER NOT NULL,
    source TEXT NOT NULL CHECK (source IN ('race', 'time_trial', 'estimate', 'manual')),
    recorded_at TEXT NOT NULL,
    notes TEXT
);

-- Race goals
CREATE TABLE race_goals (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    race_name TEXT,
    distance_m REAL NOT NULL,
    race_date TEXT NOT NULL,
    target_time_seconds INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Initial daily metrics entry (for CTL/ATL bootstrap)
CREATE TABLE daily_metrics (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    total_tss REAL NOT NULL DEFAULT 0,
    atl REAL NOT NULL,
    ctl REAL NOT NULL,
    tsb REAL NOT NULL,
    UNIQUE(user_id, date)
);
