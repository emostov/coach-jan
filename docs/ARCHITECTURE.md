# CoachJan — System Architecture

## End-State Overview

CoachJan is a single-server web application with a Rust backend and React/TypeScript frontend.

```
┌─────────────────────────────────────────────────────────┐
│                      Browser                            │
│  ┌───────────────────────────────────────────────────┐  │
│  │           React / TypeScript SPA                  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │Dashboard │ │Plan View │ │ Workout Detail   │  │  │
│  │  │          │ │(Calendar)│ │ (Charts/Metrics) │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │Chat/Coach│ │Profile   │ │ PMC Chart        │  │  │
│  │  │Interface │ │Settings  │ │ (CTL/ATL/TSB)    │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  └───────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────┘
                           │ HTTPS (JSON API + File Upload)
┌──────────────────────────┴──────────────────────────────┐
│                   Rust Backend (Axum)                    │
│                                                         │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │  API Layer  │  │  Domain Core │  │ AI Layer      │  │
│  │  (Routes,   │  │  (Training   │  │ (Claude API   │  │
│  │   Auth,     │  │   Algorithms,│  │  Client,      │  │
│  │   Handlers) │  │   FIT Parse, │  │  Prompt Mgmt, │  │
│  │             │  │   Zones,     │  │  Tool Schemas) │  │
│  │             │  │   Scoring)   │  │               │  │
│  └──────┬──────┘  └──────┬───────┘  └───────┬───────┘  │
│         │                │                   │          │
│  ┌──────┴────────────────┴───────────────────┴───────┐  │
│  │              Data Layer (SQLite via sqlx)          │  │
│  └───────────────────────────────────────────────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
                           │
                    ┌──────┴──────┐
                    │   SQLite    │
                    │   (File)    │
                    └─────────────┘

External Services:
  - Anthropic Claude API (plan generation, workout analysis, chat)
  - Coros API (future: automatic workout sync via OAuth)
```

---

## Tech Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Backend framework | Axum | Modern, tower-based, async, strong ecosystem |
| Database | SQLite via sqlx | Single-file, zero-config, sufficient for single-user. Async with sqlx compile-time checked queries |
| FIT parsing | `fit_file` crate (preferred) or custom via `nom` | ANT+ FIT binary format. Prefer existing crate; only build custom parser as last resort |
| AI client | reqwest + serde | Direct Anthropic API calls with tool_use support |
| Auth | Email + password | Argon2 password hashing, session cookies (SQLite session store) |
| Frontend framework | React 19 + TypeScript | Mature, well-tooled, strong charting ecosystem |
| Build tool | Vite | Fast dev server, optimized builds |
| Styling | Tailwind CSS 4 | Utility-first, responsive by default |
| Charts | Recharts | React-native charting, good for time-series |
| Calendar | Custom (CSS grid) | Training calendar has specific needs; simpler than adapting a library |
| Data fetching | TanStack Query | Caching, optimistic updates, loading states |
| Routing | React Router v7 | Standard, file-based routing option |
| State | Zustand | Minimal global state (auth, theme) |
| Forms | react-hook-form + zod | Schema validation, multi-step forms |
| UI primitives | shadcn/ui (Radix + Tailwind) | Accessible modals, tabs, toggles, dropdowns — unstyled, Tailwind-compatible |
| E2E testing | Playwright | Browser-based end-to-end tests for all user flows |

---

## Project Directory Structure

```
coachjan/
├── Cargo.toml
├── Cargo.lock
├── .env                          # ANTHROPIC_API_KEY, SMTP config, etc.
├── .env.example
├── sqlx-data.json                # Offline sqlx query data
│
├── migrations/                   # SQLite migrations (sqlx migrate)
│   ├── 001_create_users.sql
│   ├── 002_create_athlete_profiles.sql
│   ├── 003_create_training_plans.sql
│   ├── 004_create_workouts.sql
│   ├── 005_create_daily_metrics.sql
│   ├── 006_create_chat_messages.sql
│   └── 007_create_plan_adjustments.sql
│
├── src/
│   ├── main.rs                   # Axum app setup, routing, startup
│   ├── config.rs                 # Environment config loading
│   ├── error.rs                  # App-wide error types
│   │
│   ├── api/                      # HTTP layer — routes + handlers
│   │   ├── mod.rs
│   │   ├── auth.rs               # POST /auth/register, POST /auth/login, POST /auth/logout
│   │   ├── athletes.rs           # GET/PUT /athlete (profile)
│   │   ├── plans.rs              # POST /plan/generate, GET /plan, GET /plan/week/:id
│   │   ├── workouts.rs           # POST /workouts/upload, GET /workouts, GET /workouts/:id
│   │   ├── metrics.rs            # GET /metrics (ATL/CTL/TSB history)
│   │   ├── chat.rs               # POST /chat, GET /chat/history
│   │   └── middleware.rs         # Auth middleware, request logging
│   │
│   ├── domain/                   # Pure business logic — no I/O dependencies
│   │   ├── mod.rs
│   │   ├── zones.rs              # HR zone + pace zone calculation
│   │   ├── scoring.rs            # rTSS, TRIMP, NGP calculation
│   │   ├── effects.rs            # Aerobic/anaerobic effect algorithms
│   │   ├── classification.rs     # Workout classification (deterministic rules)
│   │   ├── load_tracking.rs      # ATL/CTL/TSB daily computation
│   │   ├── validation.rs         # Plan validation rules (§7.2)
│   │   ├── bootstrap.rs          # Initial CTL estimation from profile
│   │   └── types.rs              # Domain types, enums, value objects
│   │
│   ├── fit/                      # FIT file parser
│   │   ├── mod.rs
│   │   ├── parser.rs             # Binary FIT format parsing
│   │   ├── records.rs            # Record types (session, lap, record, etc.)
│   │   └── types.rs              # FIT-specific types
│   │
│   ├── ai/                       # Claude API integration
│   │   ├── mod.rs
│   │   ├── client.rs             # Anthropic API client (reqwest)
│   │   ├── prompts.rs            # System prompt assembly
│   │   ├── tools.rs              # Tool schemas for Claude (function calling)
│   │   ├── context.rs            # Context window assembly logic
│   │   └── handlers.rs           # AI interaction orchestration (plan gen, workout analysis, chat)
│   │
│   ├── db/                       # Database access layer
│   │   ├── mod.rs
│   │   ├── users.rs              # User auth CRUD
│   │   ├── profiles.rs           # Athlete profile CRUD
│   │   ├── plans.rs              # Training plan storage
│   │   ├── workouts.rs           # Workout storage
│   │   ├── metrics.rs            # Daily metrics storage
│   │   ├── sessions.rs           # Auth sessions
│   │   └── chat.rs               # Chat message storage
│   │
│   └── auth/                     # Auth utilities
│       ├── mod.rs
│       └── password.rs           # Argon2 password hashing + verification
│
├── frontend/                     # React SPA (separate Vite project)
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── tailwind.config.ts
│   ├── index.html
│   │
│   ├── src/
│   │   ├── main.tsx              # App entry
│   │   ├── App.tsx               # Router setup
│   │   │
│   │   ├── api/                  # API client layer
│   │   │   ├── client.ts         # Fetch wrapper, auth handling
│   │   │   ├── types.ts          # Shared API types (mirrors Rust types)
│   │   │   ├── auth.ts           # Auth API calls
│   │   │   ├── athlete.ts        # Athlete API calls
│   │   │   ├── plan.ts           # Plan API calls
│   │   │   ├── workouts.ts       # Workout API calls
│   │   │   ├── metrics.ts        # Metrics API calls
│   │   │   └── chat.ts           # Chat API calls
│   │   │
│   │   ├── pages/                # Page components (1:1 with routes)
│   │   │   ├── Login.tsx
│   │   │   ├── Onboarding.tsx    # Multi-step profile setup + plan generation
│   │   │   ├── Dashboard.tsx
│   │   │   ├── Plan.tsx          # Calendar view
│   │   │   ├── WorkoutDetail.tsx
│   │   │   ├── Chat.tsx          # Coach interaction
│   │   │   ├── Performance.tsx   # PMC chart
│   │   │   └── Profile.tsx       # Athlete profile + zones
│   │   │
│   │   ├── components/           # Reusable UI components
│   │   │   ├── layout/
│   │   │   │   ├── Shell.tsx     # App shell (nav, sidebar)
│   │   │   │   └── Nav.tsx
│   │   │   ├── charts/
│   │   │   │   ├── HRChart.tsx   # HR over time with zone coloring
│   │   │   │   ├── PaceChart.tsx
│   │   │   │   ├── PMCChart.tsx  # CTL/ATL/TSB performance chart
│   │   │   │   └── ZoneBar.tsx   # Time-in-zone bar chart
│   │   │   ├── plan/
│   │   │   │   ├── Calendar.tsx  # Training plan calendar grid
│   │   │   │   ├── WorkoutCard.tsx
│   │   │   │   └── WeekSummary.tsx
│   │   │   ├── chat/
│   │   │   │   ├── ChatWindow.tsx
│   │   │   │   ├── Message.tsx
│   │   │   │   └── ChatInput.tsx
│   │   │   ├── workout/
│   │   │   │   ├── UploadButton.tsx
│   │   │   │   ├── SplitsTable.tsx
│   │   │   │   └── MetricCard.tsx
│   │   │   └── shared/
│   │   │       ├── ZoneTable.tsx  # HR/pace zone display
│   │   │       ├── LoadingSpinner.tsx
│   │   │       └── ErrorBoundary.tsx
│   │   │
│   │   ├── hooks/                # Custom React hooks
│   │   │   ├── useAuth.ts
│   │   │   ├── useAthlete.ts
│   │   │   ├── usePlan.ts
│   │   │   ├── useWorkouts.ts
│   │   │   └── useMetrics.ts
│   │   │
│   │   └── utils/                # Frontend utilities
│   │       ├── formatting.ts     # Pace, duration, distance formatting
│   │       └── zones.ts          # Zone color mapping
│   │
│   └── public/
│       └── favicon.ico
│
├── tests/                        # Rust integration tests
│   ├── api_tests.rs
│   ├── domain_tests.rs
│   ├── fit_parser_tests.rs
│   ├── calibration/              # Scoring calibration tests
│   │   └── effect_scores_test.rs # Run FIT files through scoring, assert expected ranges
│   └── fixtures/                 # Test FIT files, sample data
│       ├── easy_run.fit
│       ├── tempo_run.fit
│       └── interval_session.fit
│
├── e2e/                          # Playwright end-to-end tests
│   ├── playwright.config.ts
│   ├── auth.spec.ts              # Register, login, logout flows
│   ├── onboarding.spec.ts        # Profile setup + plan generation
│   ├── plan.spec.ts              # Calendar view, workout cards
│   ├── upload.spec.ts            # FIT upload + workout detail
│   ├── chat.spec.ts              # Coach chat interaction
│   └── helpers/
│       ├── seed.ts               # Create test athlete with plan + workouts
│       └── auth.ts               # Login helper for authenticated tests
│
└── docs/                         # Project documentation
    ├── ARCHITECTURE.md           # This file
    ├── FEATURES.md               # Feature inventory
    ├── IMPLEMENTATION_PLAN.md    # Phased build plan
    └── phases/                   # Per-phase implementation guides
        ├── PHASE_1_AUTH_PROFILE.md
        ├── PHASE_2_PLAN_GENERATION.md
        ├── PHASE_3_PLAN_VIEWING.md
        ├── PHASE_4_WORKOUT_UPLOAD.md
        ├── PHASE_5A_SCORING.md
        ├── PHASE_5B_AI_ANALYSIS_CHAT.md
        └── PHASE_6_PLAN_ADJUSTMENT.md
```

---

## Database Schema

All tables use INTEGER PRIMARY KEY (SQLite rowid alias). Timestamps are stored as ISO 8601 UTC strings (TEXT, always with `Z` suffix).

**CRITICAL**: SQLite does not enforce foreign keys by default. Every connection must run `PRAGMA foreign_keys = ON` before any queries. In sqlx, use `SqliteConnectOptions::pragma("foreign_keys", "ON")`.

```sql
-- Auth & Identity (separate from athlete profile)
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,             -- Argon2id
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE athlete_profiles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    weight_kg REAL NOT NULL,
    resting_hr INTEGER NOT NULL,
    max_hr INTEGER NOT NULL,
    lthr INTEGER NOT NULL,
    ftpace_m_per_s REAL,                    -- FTPace in m/s (nullable until set)
    current_weekly_volume_km REAL NOT NULL,
    experience_level TEXT NOT NULL CHECK (experience_level IN ('beginner', 'intermediate', 'advanced')),
    sports_background TEXT,                  -- JSON: prior sports, injury history
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE sessions (
    id TEXT PRIMARY KEY,                     -- UUID session token
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_sessions_expires ON sessions(expires_at);

-- Physiological History
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

-- Race Goals
CREATE TABLE race_goals (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    race_name TEXT,
    distance_m REAL NOT NULL,               -- Race distance in meters
    race_date TEXT NOT NULL,
    target_time_seconds INTEGER,             -- Optional target finish time
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Training Plan Structure
CREATE TABLE macrocycles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    race_goal_id INTEGER NOT NULL REFERENCES race_goals(id) ON DELETE CASCADE,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    target_ctl REAL,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'abandoned')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE mesocycles (
    id INTEGER PRIMARY KEY,
    macrocycle_id INTEGER NOT NULL REFERENCES macrocycles(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,        -- Order within macrocycle
    phase TEXT NOT NULL CHECK (phase IN ('capacity', 'utilization', 'taper', 'recovery', 'transition')),
    focus TEXT NOT NULL CHECK (focus IN ('aerobic_capacity', 'aerobic_utilization', 'anaerobic_capacity', 'anaerobic_utilization', 'race_specific', 'recovery')),
    load_weeks INTEGER NOT NULL,
    recovery_weeks INTEGER NOT NULL,
    target_volume_km REAL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'active', 'completed')),
    evaluation_summary TEXT,                 -- JSON: Coach's mesocycle evaluation
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE planned_workouts (
    id INTEGER PRIMARY KEY,
    mesocycle_id INTEGER NOT NULL REFERENCES mesocycles(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scheduled_date TEXT NOT NULL,
    workout_type TEXT NOT NULL CHECK (workout_type IN (
        'easy_run', 'long_run', 'aerobic_development', 'tempo_run',
        'vo2max_intervals', 'speed_sprint', 'race_specific',
        'recovery_run', 'rest',
        'strength_precision', 'strength_performance', 'strength_power'
    )),
    duration_min INTEGER,
    target_hr_zones TEXT,                    -- JSON array: [1, 2]
    target_pace_zones TEXT,                  -- JSON array: [2, 3]
    expected_tss REAL,
    description TEXT,                        -- Coach notes for the workout
    is_completed INTEGER NOT NULL DEFAULT 0,
    completed_workout_id INTEGER REFERENCES completed_workouts(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_planned_workouts_user_date ON planned_workouts(user_id, scheduled_date);

-- Completed Workouts
CREATE TABLE completed_workouts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    planned_workout_id INTEGER REFERENCES planned_workouts(id),  -- NULL if unplanned
    fit_file_path TEXT,                      -- Path to stored FIT file
    source TEXT NOT NULL CHECK (source IN ('fit_upload', 'coros_api', 'manual')),
    hr_data_sufficient INTEGER NOT NULL DEFAULT 1,  -- 0 if HR data < 50% present

    -- Parsed summary
    started_at TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL,
    distance_m REAL NOT NULL,
    avg_hr INTEGER,
    max_hr INTEGER,
    avg_pace_m_per_s REAL,
    max_pace_m_per_s REAL,
    avg_cadence REAL,
    elevation_gain_m REAL,
    elevation_loss_m REAL,
    ngp_m_per_s REAL,                        -- Normalized Graded Pace

    -- Computed scores (NULL if hr_data_sufficient = 0 for HR-based scores)
    rtss REAL,
    trimp REAL,
    aerobic_effect REAL,
    anaerobic_effect REAL,
    intensity_factor REAL,

    -- Zone time distribution (individual columns for queryability)
    hr_zone_1_seconds INTEGER DEFAULT 0,
    hr_zone_2_seconds INTEGER DEFAULT 0,
    hr_zone_3_seconds INTEGER DEFAULT 0,
    hr_zone_4_seconds INTEGER DEFAULT 0,
    hr_zone_5_seconds INTEGER DEFAULT 0,
    hr_zone_6_seconds INTEGER DEFAULT 0,
    hr_zone_7_seconds INTEGER DEFAULT 0,
    pace_zone_1_seconds INTEGER DEFAULT 0,
    pace_zone_2_seconds INTEGER DEFAULT 0,
    pace_zone_3_seconds INTEGER DEFAULT 0,
    pace_zone_4_seconds INTEGER DEFAULT 0,
    pace_zone_5_seconds INTEGER DEFAULT 0,
    pace_zone_6_seconds INTEGER DEFAULT 0,

    -- Classification
    classification TEXT CHECK (classification IN (
        'aerobic_capacity', 'aerobic_utilization',
        'anaerobic_capacity', 'anaerobic_utilization', 'mixed'
    )),
    compliance TEXT CHECK (compliance IN (
        'on_target', 'harder_than_prescribed', 'easier_than_prescribed',
        'different_type', 'unplanned'
    )),

    -- Coach commentary
    coach_summary TEXT,
    coach_commentary TEXT,                   -- Full coach response

    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_completed_workouts_user_date ON completed_workouts(user_id, started_at);

-- Time-series data (stored separately due to volume)
CREATE TABLE workout_records (
    id INTEGER PRIMARY KEY,
    workout_id INTEGER NOT NULL REFERENCES completed_workouts(id) ON DELETE CASCADE,
    timestamp_ms INTEGER NOT NULL,           -- Milliseconds since workout start
    heart_rate INTEGER,
    speed_m_per_s REAL,
    latitude REAL,
    longitude REAL,
    altitude_m REAL,
    cadence INTEGER,
    power_watts INTEGER,
    distance_m REAL                          -- Cumulative distance
);

CREATE INDEX idx_workout_records_workout ON workout_records(workout_id);

-- Daily Metrics (PMC data)
CREATE TABLE daily_metrics (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    total_tss REAL NOT NULL DEFAULT 0,       -- Sum of all workout TSS for the day
    atl REAL NOT NULL,
    ctl REAL NOT NULL,
    tsb REAL NOT NULL,
    UNIQUE(user_id, date)
);

-- Chat History
CREATE TABLE chat_messages (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('athlete', 'coach', 'system')),
    content TEXT NOT NULL,
    context_type TEXT,                        -- 'workout_analysis', 'plan_generation', 'freeform', etc.
    related_workout_id INTEGER REFERENCES completed_workouts(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_chat_messages_user_date ON chat_messages(user_id, created_at);

-- Plan Adjustments (Phase 6)
CREATE TABLE plan_adjustments (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trigger_reason TEXT NOT NULL,
    operations TEXT NOT NULL,                -- JSON array of operations
    explanation TEXT NOT NULL,
    proposed_by_workout_id INTEGER REFERENCES completed_workouts(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'rejected')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    resolved_at TEXT
);

-- Strength Assessment
CREATE TABLE movement_assessments (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assessment_type TEXT NOT NULL,            -- 'hip_flexor_length', 'ankle_dorsiflexion', etc.
    result TEXT NOT NULL CHECK (result IN ('pass', 'fail', 'limited')),
    notes TEXT,
    assessed_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE strength_progress (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    current_level INTEGER NOT NULL DEFAULT 1 CHECK (current_level BETWEEN 1 AND 4),
    current_workout_number INTEGER NOT NULL DEFAULT 1,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

---

## API Surface

All endpoints return JSON. Auth-protected routes require a valid session cookie.

### Auth
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| POST | `/api/auth/register` | `{ email, password }` | `{ athlete }` + sets session cookie | Create account |
| POST | `/api/auth/login` | `{ email, password }` | `{ athlete }` + sets session cookie | Log in |
| POST | `/api/auth/logout` | — | `{ message }` | Clears session |
| GET | `/api/auth/me` | — | `{ athlete }` or 401 | Check auth status |

### Athlete Profile
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| POST | `/api/athlete/profile` | `{ ...profile fields }` | `{ profile, zones }` | Complete profile during onboarding |
| GET | `/api/athlete/profile` | — | `{ profile, zones }` | Get current profile + zones |
| PUT | `/api/athlete/profile` | `{ ...fields }` | `{ profile, zones }` | Update profile (zones recalculate) |
| GET | `/api/athlete/zones` | — | `{ hr_zones, pace_zones }` | Current zone tables |

### Training Plan
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| POST | `/api/plan/generate` | `{ race_goal_id }` | `{ macrocycle_skeleton }` | Phase 1: skeleton via Claude |
| POST | `/api/plan/confirm` | `{ macrocycle_id, modifications? }` | `{ macrocycle, first_week }` | Confirm skeleton, generate week 1 |
| GET | `/api/plan` | — | `{ macrocycle, mesocycles }` | Current plan overview |
| GET | `/api/plan/week?date=...` | — | `{ week, workouts }` | Workouts for a specific week |
| GET | `/api/plan/workout/:id` | — | `{ planned_workout }` | Single workout detail |

### Workouts
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| POST | `/api/workouts/upload` | `multipart/form-data (FIT file)` | `{ workout }` | Upload + parse + score (deterministic only, fast) |
| GET | `/api/workouts/:id/analysis` | — | `{ analysis }` or 202 if pending | AI coach analysis (triggered async after upload) |
| GET | `/api/workouts` | `?from=&to=&limit=20&offset=0` | `[{ workout_summary }]` | List completed workouts (paginated) |
| GET | `/api/workouts/:id` | — | `{ workout, analysis? }` | Full workout detail |
| GET | `/api/workouts/:id/records` | — | `[{ record }]` | Time-series data for charts |
| DELETE | `/api/workouts/:id` | — | `{ message }` | Delete workout, recalculate daily metrics |

### Metrics
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| GET | `/api/metrics` | `?from=&to=` | `[{ date, atl, ctl, tsb }]` | PMC chart data |
| GET | `/api/metrics/current` | — | `{ atl, ctl, tsb, date }` | Latest ATL/CTL/TSB |

### Chat
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| POST | `/api/chat` | `{ message }` | `{ response }` | Send message, get coach reply |
| GET | `/api/chat/history` | `?limit=&offset=` | `[{ message }]` | Chat history |

### Plan Adjustment
| Method | Path | Body | Response | Notes |
|--------|------|------|----------|-------|
| GET | `/api/plan/adjustments` | — | `[{ adjustment }]` | Pending adjustment proposals |
| POST | `/api/plan/adjustments/:id/accept` | — | `{ updated_plan }` | Accept a proposed adjustment |
| POST | `/api/plan/adjustments/:id/reject` | — | `{ message }` | Reject a proposed adjustment |

---

## Key Design Decisions

### 1. Deterministic vs. AI Boundary

The single most important architectural boundary: **Claude reasons, the server computes.**

| Always Server-Side (Deterministic) | Always Claude (AI) |
|------------------------------------|--------------------|
| rTSS / TRIMP calculation | Plan structure decisions |
| Aerobic/anaerobic effect scoring | Workout analysis commentary |
| NGP calculation | Mesocycle evaluation |
| ATL/CTL/TSB daily update | Free-form coaching chat |
| Workout classification | Plan adjustment proposals |
| Zone calculation from LTHR/FTPace | Coach persona and communication |
| Date arithmetic | Strength exercise selection |
| Plan validation rules | Explaining "why" behind prescriptions |

### 2. AI Interaction Pattern

All Claude calls follow the same flow:

```
1. Handler assembles context (profile, metrics, temporal facts, flags)
2. Handler selects system prompt variant + tool schemas
3. Client sends request to Anthropic API with tool_use
4. Response is parsed and validated
5. On validation failure: retry with error context (max 2 retries)
6. On success: persist results, return to frontend
7. On total failure: return computed metrics only (graceful degradation)
```

### 3. Static Frontend Serving

In production, the Rust server serves the built React SPA from a `static/` directory. During development, Vite dev server proxies API calls to the Rust backend.

```
Production:
  Browser → Axum → /api/* (JSON API)
                  → /* (static files, SPA fallback to index.html)

Development:
  Browser → Vite (port 5173) → /api/* proxy → Axum (port 3000)
```

### 4. FIT File Storage

FIT files are stored on disk in a structured directory:
```
data/
  fit_files/
    {athlete_id}/
      {YYYY-MM-DD}_{workout_id}.fit
```

The file path is recorded in `completed_workouts.fit_file_path`. Original FIT files are retained for re-processing if algorithms are updated.

### 5. Session Management

- Sessions expire in 30 days (configurable)
- Session token is a UUID stored in an HTTP-only, Secure, SameSite=Strict cookie
- Session lookup on each authenticated request via Axum middleware (filter `WHERE expires_at > datetime('now')`)
- Passwords hashed with Argon2id before storage
- SameSite=Strict provides CSRF protection — cross-origin requests never include the cookie
- Background task cleans up expired sessions hourly

### 6. Background Tasks

Several features require periodic or async processing. Use `tokio::spawn` with `tokio::time::interval` — no external job queue needed.

| Task | Trigger | What it does |
|------|---------|-------------|
| Session cleanup | Hourly interval | Delete sessions where `expires_at < now` |
| Rest-day backfill | Before any ATL/CTL/TSB read or workout upload | Fill `daily_metrics` with TSS=0 for days between last entry and today |
| AI workout analysis | After upload (async) | Call Claude for coach commentary, store result, create chat message |
| AI retry on failure | After failed Claude call | Retry up to 2 times with backoff |

### 7. Workout Upload Pipeline

Upload is split into two stages to ensure the athlete always gets fast feedback:

```
Stage 1 (synchronous, < 5 seconds):
  Receive FIT file → save to disk → parse → compute derived metrics →
  calculate NGP, rTSS, TRIMP, effects → classify → update ATL/CTL/TSB →
  match to planned workout → return workout with all scores

Stage 2 (async background task):
  Build AI context → call Claude for analysis → store commentary →
  create chat message → check adjustment triggers →
  if adjustment warranted → store pending adjustment proposal
```

The frontend polls `GET /api/workouts/:id/analysis` (or uses SSE) to pick up the AI commentary when ready. If Claude fails, the athlete still has all their deterministic scores.

---

## Deployment

### Local Development
```bash
# Backend
cargo run                  # Starts Axum on :3000

# Frontend (separate terminal)
cd frontend && npm run dev # Starts Vite on :5173 with proxy to :3000
```

### Production (Railway or similar)
```bash
# Build frontend
cd frontend && npm run build  # Outputs to frontend/dist/

# Copy to static dir
cp -r frontend/dist/ static/

# Build backend (serves static files)
cargo build --release

# Run
DATABASE_URL=sqlite:data/coachjan.db \
ANTHROPIC_API_KEY=sk-... \
./target/release/coachjan
```

Single binary + SQLite file + FIT file directory. No Redis, no Postgres, no container orchestration.

---

## Testing Strategy

### Rust Unit Tests
- All `src/domain/` modules have unit tests (pure functions, no mocking needed)
- Zone calculation, scoring, effects, classification, load tracking, validation, bootstrap
- Run: `cargo test`

### Rust Integration Tests
- `tests/` directory — test API endpoints end-to-end
- Each test gets a fresh in-memory SQLite database (`sqlite::memory:`), migrations run before each test
- Claude API mocked via `wiremock` crate (mock HTTP server returning canned tool_use responses)
- Test auth flows, workout upload pipeline, plan generation with mocked Claude

### Scoring Calibration Tests
- `tests/calibration/` — run real FIT files through scoring pipeline
- Assert aerobic/anaerobic effect scores within tolerance of expected values
- Reference data: FIT files with known Coros/Garmin scores stored in `tests/fixtures/`
- These tests are the ground truth for tuning k1, k2, k3, scale factors

### Frontend Component Tests
- Vitest + Testing Library for React component tests
- MSW (Mock Service Worker) for mocking API responses in component tests
- Test form validation, zone display, chart rendering with sample data
- Run: `cd frontend && npm test`

### End-to-End Tests (Playwright)
- `e2e/` directory — browser-based tests against a running app
- Test complete user flows: register → onboard → view plan → upload workout → see analysis → chat
- Each test uses a seeded database via `e2e/helpers/seed.ts`
- Run: `npx playwright test`
- Added at the end of each phase to validate the phase's usable state
- Playwright tests are the primary validation for frontend features — they test real browser behavior, responsive layout, and user interactions

### What Gets Tested When
| Phase | Rust Tests | Frontend Tests | Playwright E2E |
|-------|-----------|----------------|----------------|
| 1 | Auth, zones, bootstrap | Login/register form, zone table | Register → onboard → see zones |
| 2 | Plan validation, Claude mock | Plan review UI | Onboard → generate plan → confirm |
| 3 | Plan query endpoints | Calendar, workout cards | Navigate calendar, view workout details |
| 4 | FIT parser, upload handler | Upload button, charts | Upload FIT → see workout data |
| 5a | Scoring, effects, ATL/CTL/TSB | Enhanced workout view, PMC chart | Upload → see scores + PMC chart |
| 5b | AI analysis (mocked), chat | Chat UI, analysis display | Upload → see coach commentary → chat |
| 6 | Adjustments, mesocycle transition | Adjustment diff UI | Trigger adjustment → accept/reject |

---

## Rust Crate Dependencies

```toml
[dependencies]
# Web framework
axum = { version = "0.8", features = ["multipart"] }
tokio = { version = "1", features = ["full"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "fs", "trace"] }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# HTTP client (Claude API)
reqwest = { version = "0.12", features = ["json"] }

# Auth
uuid = { version = "1", features = ["v4"] }
argon2 = "0.5"                  # Password hashing (Argon2id)

# Time
chrono = { version = "0.4", features = ["serde"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Config
dotenvy = "0.15"

# FIT parsing
nom = "7"                   # Binary parser combinators for FIT format

[dev-dependencies]
axum-test = "16"             # HTTP testing
wiremock = "0.6"             # Mock HTTP server for Claude API tests
```

---

## Frontend Dependencies

```json
{
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "react-router": "^7.0.0",
    "@tanstack/react-query": "^5.0.0",
    "react-hook-form": "^7.54.0",
    "@hookform/resolvers": "^3.9.0",
    "zod": "^3.24.0",
    "recharts": "^2.15.0",
    "zustand": "^5.0.0",
    "date-fns": "^4.0.0"
  },
  "devDependencies": {
    "typescript": "^5.7.0",
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "vite": "^6.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    "tailwindcss": "^4.0.0",
    "vitest": "^3.0.0",
    "@testing-library/react": "^16.0.0"
  }
}
```

**Note**: `shadcn/ui` components are installed individually via `npx shadcn@latest add <component>` (dialog, tabs, dropdown-menu, etc.). They are copied into `frontend/src/components/ui/` as source files, not imported as a package dependency.

**Playwright** (E2E tests, installed at project root):
```json
{
  "devDependencies": {
    "@playwright/test": "^1.50.0"
  }
}
```
