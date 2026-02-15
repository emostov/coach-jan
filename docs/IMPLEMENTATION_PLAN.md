# CoachJan — Implementation Plan

## Guiding Principles

1. **Each phase produces a usable app.** No phase is purely "infrastructure." Every phase ends with something the athlete can interact with.
2. **Vertical slices.** Each phase builds a complete path from frontend → API → domain → database.
3. **TDD.** Write tests first for domain logic. Integration tests for API endpoints. Component tests for frontend.
4. **Agent-friendly.** Each phase has a dedicated doc in `docs/phases/` with everything an agent needs: exact files to create/modify, data structures, test cases, and acceptance criteria. An agent should be able to complete a phase without reading the full product design.

## Prerequisites (Phase 0)

Before any feature work, set up the project skeleton.

**Tasks**:
1. Initialize Rust project: `cargo init --name coachjan`
2. Add all dependencies to `Cargo.toml` (see ARCHITECTURE.md)
3. Set up Axum app skeleton (`main.rs`, `config.rs`, `error.rs`)
4. Set up SQLite with sqlx (connection pool, migration runner)
5. Create `migrations/` directory with initial schema
6. Initialize frontend: `npm create vite@latest frontend -- --template react-ts`
7. Install frontend dependencies (see ARCHITECTURE.md)
8. Configure Tailwind CSS
9. Configure Vite proxy to backend (`/api` → `localhost:3000`)
10. Set up `.env.example` with required environment variables
11. Create `data/` directory structure for FIT files
12. Set up basic logging with `tracing`
13. Create shared error type that maps to HTTP responses
14. Set up CI: `cargo test` + `cargo clippy` + `npm run build`

**Deliverable**: Empty app that starts, serves a "Hello from CoachJan" page, connects to SQLite, and runs migrations.

---

## Phase 1: Authentication & Athlete Profile

**Goal**: Athlete can register with email + password, log in, and fill out their profile. The app shows their calculated HR and pace zones.

**Usable state after this phase**: An athlete can create an account, set up their profile, see their training zones, and update their information. This is the foundation that everything else builds on.

### Backend Tasks

#### 1.1 Auth System
- **Files**: `src/api/auth.rs`, `src/api/middleware.rs`, `src/db/sessions.rs`, `src/auth/password.rs`
- **Migrations**: `001_create_users.sql`, `002_create_athlete_profiles.sql` (sessions table included in 001)
- Implement `POST /api/auth/register`:
  - Accept `{ email, password }` body
  - Validate email format and password length (min 8 chars)
  - If email already exists → return 409 Conflict
  - Hash password with Argon2id
  - Create user row (email + password_hash)
  - Create session (UUID v4), store in `sessions` table
  - Set HTTP-only, Secure, SameSite=Strict cookie with session ID
  - Return `{ user }` (id, email, has_profile: false)
- Implement `POST /api/auth/login`:
  - Accept `{ email, password }` body
  - Look up user by email → 401 if not found
  - Verify password against stored Argon2id hash → 401 if wrong
  - Create session (UUID v4), store in `sessions` table
  - Set HTTP-only, Secure, SameSite=Strict cookie with session ID
  - Return `{ athlete }`
- Implement `POST /api/auth/logout`:
  - Delete session from DB
  - Clear cookie
- Implement `GET /api/auth/me`:
  - Read session cookie → look up session → return athlete (or 401)
- Implement auth middleware (Axum extractor):
  - Extract session ID from cookie
  - Load athlete from DB
  - Make athlete available to handlers via request extensions
  - Return 401 if no valid session
- Implement password utilities (`src/auth/password.rs`):
  - `hash_password(password: &str) -> Result<String>` (Argon2id)
  - `verify_password(password: &str, hash: &str) -> Result<bool>`

#### 1.2 Athlete Profile
- **Files**: `src/api/athletes.rs`, `src/domain/zones.rs`, `src/domain/bootstrap.rs`, `src/domain/types.rs`, `src/db/profiles.rs`
- **Migrations**: `002_create_athlete_profiles.sql`, `003_create_training_plans.sql` (race_goals part)
- Implement `POST /api/athlete/profile`:
  - Accept full profile body (linked to authenticated user)
  - Validate all required fields
  - Calculate zones from LTHR and FTPace
  - Bootstrap initial CTL/ATL from weekly volume
  - Store athlete profile in `athlete_profiles`, create initial daily_metrics entry
  - Store FTPace in history, LTHR in history
  - Store race goal
  - Return profile + zones
- Implement `GET /api/athlete/profile`:
  - Return full profile + calculated zones
- Implement `PUT /api/athlete/profile`:
  - Update profile fields
  - If LTHR changed → recalculate HR zones, record in history
  - If FTPace changed → recalculate pace zones, record in history
  - Return updated profile + zones
- Implement zone calculation (`domain/zones.rs`):
  - `calculate_hr_zones(lthr: u16) -> HrZones` (7-zone)
  - `calculate_pace_zones(ftpace_m_per_s: f64) -> PaceZones` (6-zone)
  - Each zone is a struct with `min` and `max` values
- Implement CTL bootstrap (`domain/bootstrap.rs`):
  - `bootstrap_ctl(weekly_km: f64, level: ExperienceLevel) -> (f64, f64)` → (CTL, ATL)

#### 1.3 Domain Types
- **File**: `src/domain/types.rs`
- Define core enums: `ExperienceLevel`, `WorkoutType`, `Phase`, `Focus`, `Compliance`, `Classification`
- Define value objects: `HrZones`, `PaceZones`, `HrZone`, `PaceZone`
- All enums derive `Serialize`, `Deserialize`, and map to/from SQL TEXT

### Frontend Tasks

#### 1.4 Login / Register Page
- **File**: `frontend/src/pages/Login.tsx`
- Toggle between Login and Register modes
- Login: email + password fields + "Log In" button
- Register: email + password + confirm password fields + "Create Account" button
- Error state: display error message (wrong password, email taken, etc.)
- On success: redirect to `/onboarding` (if profile incomplete) or `/` (if complete)
- Clean, centered layout. Mobile-friendly.

#### 1.5 Onboarding Flow
- **File**: `frontend/src/pages/Onboarding.tsx`
- Multi-step form:
  - Step 1: Basic info (name, age, weight)
  - Step 2: Physiology (resting HR, max HR, LTHR)
  - Step 3: Running (FTPace or recent race result, weekly volume, experience level)
  - Step 4: Goal (race distance, date, name)
  - Step 5: Review (show calculated zones, confirm)
- Progress indicator (step 1 of 5)
- Back/Next navigation
- Form validation on each step
- On final submit → POST /api/athlete → redirect to dashboard (empty for now)

#### 1.6 Profile Page
- **File**: `frontend/src/pages/Profile.tsx`
- Display current profile fields (editable)
- Display HR zone table and pace zone table
- Save button → PUT /api/athlete
- Show zone recalculation in real-time as LTHR/FTPace change

#### 1.7 App Shell & Routing
- **Files**: `frontend/src/App.tsx`, `frontend/src/components/layout/Shell.tsx`, `frontend/src/components/layout/Nav.tsx`
- React Router setup:
  - `/login` → Login page (unauthenticated)
  - `/onboarding` → Onboarding (authenticated, no profile)
  - `/` → Dashboard (authenticated, has profile)
  - `/plan` → Plan view
  - `/chat` → Coach chat
  - `/performance` → PMC chart
  - `/profile` → Profile settings
- Auth guard: redirect to `/login` if unauthenticated, to `/onboarding` if no profile
- Navigation: bottom bar on mobile, sidebar on desktop

#### 1.8 API Client & Hooks
- **Files**: `frontend/src/api/client.ts`, `frontend/src/api/auth.ts`, `frontend/src/api/athlete.ts`, `frontend/src/hooks/useAuth.ts`, `frontend/src/hooks/useAthlete.ts`
- Fetch wrapper with auth error handling (401 → redirect to login)
- TanStack Query hooks for all API calls

### Tests
- Unit tests: zone calculation, CTL bootstrap, password hashing/verification
- Integration tests: auth flow (register → me → logout → login → me), profile CRUD
- Test: register duplicate email returns 409
- Test: login wrong password returns 401
- Frontend: login/register form validation (react-hook-form + zod), onboarding form validation, zone display
- **Playwright E2E**: register → complete onboarding → see zones on profile page → logout → login → see dashboard

---

## Phase 2: Training Plan Generation

**Goal**: After profile setup, Claude generates a periodized macrocycle. Athlete reviews the skeleton and confirms. Weekly plans are generated for the first mesocycle.

**Usable state after this phase**: Athlete has a complete training plan for their first mesocycle. They can see the macrocycle overview (which mesocycles, what phases), and the detailed weekly schedule. They can't yet view it in a nice calendar (Phase 3), but they can see their plan data.

### Backend Tasks

#### 2.1 Claude API Client
- **Files**: `src/ai/client.rs`, `src/config.rs`
- HTTP client wrapping Anthropic Messages API:
  - `send_message(system: &str, messages: Vec<Message>, tools: Vec<Tool>, model: Model) -> Result<Response>`
  - Handle tool_use responses: parse tool name + input JSON
  - Handle content responses (text blocks)
  - Retry on 429 (rate limit) with exponential backoff
  - Timeout: 60 seconds for plan generation
  - API key from environment
- Model enum: `Sonnet`, `Haiku`, `Opus`

#### 2.2 System Prompt & Tool Schemas
- **Files**: `src/ai/prompts.rs`, `src/ai/tools.rs`
- Base system prompt: Coach Jan persona, philosophy, plan stability rules, workout type enum, guardrails
- Tool schema for `generate_macrocycle_skeleton`:
  - Properties: `target_ctl`, `mesocycles` (array of phase/focus/weeks/volume), `coach_message`
- Tool schema for `generate_weekly_plan`:
  - Properties: `week_number`, `target_volume_km`, `target_tss`, `workouts` (array of day/type/duration/zones/tss/notes), `strength_sessions`
- Use `serde_json::Value` for tool input parsing, then validate against expected structure

#### 2.3 Context Assembly
- **File**: `src/ai/context.rs`
- `build_plan_generation_context(athlete, race_goal, ctl) -> String`
  - Includes: full profile, goal race, current CTL, weeks until race, recommended mesocycle structure for level
- `build_weekly_plan_context(athlete, mesocycle, week_number) -> String`
  - Includes: mesocycle focus, volume target, week position

#### 2.4 Plan Generation Handler
- **File**: `src/ai/handlers.rs`
- `generate_macrocycle(athlete, race_goal) -> MacroclyleSkeleton`:
  1. Build context
  2. Call Claude with `generate_macrocycle_skeleton` tool
  3. Parse tool response
  4. Return skeleton for review (not persisted yet)
- `confirm_and_generate_weeks(athlete, skeleton) -> Macrocycle`:
  1. Persist macrocycle and mesocycles to DB
  2. For each week in first mesocycle: call Claude with `generate_weekly_plan`
  3. Validate each week (§7.2 rules)
  4. On validation failure: retry with error context (max 2)
  5. Persist planned workouts
  6. Return complete plan

#### 2.5 Plan Validation
- **File**: `src/domain/validation.rs`
- `validate_weekly_plan(plan, athlete_ctl, previous_week_volume) -> Result<(), Vec<ValidationError>>`
- Rules (each returns specific error):
  - `max_intensity_sessions(workouts) <= 3`
  - `volume_increase_pct(this_week, last_week) <= 10` (unless recovery)
  - `recovery_week_reduction(this_week, load_week) in 30-60%`
  - `weekly_tss in (ctl * 7 * 0.5) .. (ctl * 7 * 2.0)`
  - All workout types in enum
  - No duplicate dates
  - `rest_days >= 1`
  - `long_runs <= 1`
  - Strength not on VO2max/speed days (warning)

#### 2.6 Plan Storage
- **Files**: `src/db/plans.rs`
- **Migrations**: `002_create_training_plans.sql`
- CRUD for macrocycles, mesocycles, planned_workouts
- `get_current_plan(athlete_id) -> Option<Macrocycle>` with nested mesocycles
- `get_week_workouts(athlete_id, date) -> Vec<PlannedWorkout>`

### API Endpoints
- `POST /api/plan/generate` → returns macrocycle skeleton (JSON, not persisted)
- `POST /api/plan/confirm` → persists macrocycle, generates weekly plans, returns full plan
- `GET /api/plan` → returns current macrocycle with all mesocycles

### Frontend Tasks

#### 2.7 Plan Generation UI (in Onboarding)
- Add steps 6-7 to `Onboarding.tsx`:
  - Step 6: "Generating your training plan..." → POST /api/plan/generate → show skeleton
  - Step 7: Review skeleton (mesocycle list with phase, focus, duration). Confirm or request modifications. → POST /api/plan/confirm → redirect to dashboard
- Show loading state with coach persona message ("Let me build your plan...")
- Display macrocycle overview: list of mesocycles with phase labels and date ranges

#### 2.8 Basic Plan Display
- Simple list view of upcoming workouts (placeholder until Phase 3 calendar)
- Show on dashboard: "Your plan has been generated. Here's your first week..."

### Tests
- Unit tests: plan validation rules (each rule tested independently)
- Integration tests: full plan generation flow (mock Claude API)
- Test: validation rejection triggers retry with error context

---

## Phase 3: Training Plan Viewing

**Goal**: Rich calendar view of the training plan. Dashboard with today's workout and current training context.

**Usable state after this phase**: Athlete can view their entire training plan in a calendar, see today's workout with full details, and track their weekly progress against targets. This is the daily touchpoint — the athlete opens the app and knows exactly what to do today.

### Frontend Tasks

#### 3.1 Calendar Component
- **File**: `frontend/src/components/plan/Calendar.tsx`
- CSS grid: 7 columns (Mon-Sun), rows = weeks
- Data source: `GET /api/plan/week?date=...` (fetch visible weeks)
- Each cell renders a `WorkoutCard`:
  - Future/planned: workout type icon, duration, target zone
  - Completed: green/yellow/red border based on compliance
  - Rest day: dimmed cell
  - Today: highlighted border
- Navigation: prev/next week buttons, "Today" button
- Mesocycle boundaries: horizontal divider with phase label
- Mobile: switch to stacked day view (vertical list)

#### 3.2 Workout Card
- **File**: `frontend/src/components/plan/WorkoutCard.tsx`
- Compact mode (in calendar cell):
  - Color-coded icon by workout type
  - Duration (e.g., "45 min")
  - Primary zone (e.g., "Z1-2")
- Expanded mode (on click/tap):
  - Full workout description
  - Target HR zones and pace zones
  - Expected TSS
  - Coach notes
  - For completed: actual metrics alongside targets

#### 3.3 Week Summary
- **File**: `frontend/src/components/plan/WeekSummary.tsx`
- Sidebar or bottom bar showing:
  - "Week 2 of Mesocycle 1 — Aerobic Capacity"
  - Target volume: 40 km | Completed: 28 km
  - Target TSS: 250 | Completed: 180
  - Sessions: 4/6 completed

#### 3.4 Dashboard
- **File**: `frontend/src/pages/Dashboard.tsx`
- Today's workout card (large, prominent)
- If no workout today: "Rest day" or next upcoming workout
- Phase banner: "Mesocycle 1 — Aerobic Capacity Building, Week 2 of 3"
- Quick metrics row: CTL / ATL / TSB (placeholder values until Phase 5)
- Race countdown: "52 days until [race name]"
- Weekly progress bar

#### 3.5 Plan Overview Page
- **File**: `frontend/src/pages/Plan.tsx`
- Top section: macrocycle timeline bar (colored by phase)
- Calendar below
- Tab/toggle: "Week" view vs "Month" view

### Backend Tasks

#### 3.6 Additional Plan Endpoints
- `GET /api/plan/week?date=YYYY-MM-DD` → returns planned workouts for the week containing that date
- `GET /api/plan/workout/:id` → returns single planned workout with full details
- Include computed fields: `is_today`, `is_past`, `is_completed`

### Tests
- Frontend component tests: Calendar renders correct number of days, workout cards display correctly
- Responsive tests: verify mobile layout works

---

## Phase 4: Manual FIT File Upload & Parsing

**Goal**: Athlete can upload a FIT file. The server parses it and displays the workout data — time-series charts, summary metrics, splits.

**Usable state after this phase**: Athlete goes for a run, uploads their FIT file, and sees a rich workout view — HR chart, pace chart, splits table, summary metrics. The workout is linked to the planned workout for that day. No scoring or AI analysis yet (Phase 5), but the raw data is all there.

### Backend Tasks

#### 4.1 FIT File Parser
- **Files**: `src/fit/parser.rs`, `src/fit/records.rs`, `src/fit/types.rs`
- Parse FIT binary format using `nom`:
  - Read 14-byte header (size, protocol, profile, data type indicator)
  - Parse definition messages (field definitions for upcoming data messages)
  - Parse data messages using definitions
  - Handle compressed timestamps
  - CRC validation
- Extract `record` messages → `Vec<FitRecord>`:
  ```rust
  struct FitRecord {
      timestamp_ms: u64,  // ms since workout start
      heart_rate: Option<u16>,
      speed_m_per_s: Option<f64>,
      latitude: Option<f64>,
      longitude: Option<f64>,
      altitude_m: Option<f64>,
      cadence: Option<u16>,
      power_watts: Option<u16>,
      distance_m: Option<f64>,
  }
  ```
- Extract `session` message → `FitSession`:
  ```rust
  struct FitSession {
      start_time: DateTime<Utc>,
      total_timer_time_s: f64,
      total_distance_m: f64,
      avg_heart_rate: Option<u16>,
      max_heart_rate: Option<u16>,
      avg_speed_m_per_s: Option<f64>,
      max_speed_m_per_s: Option<f64>,
      total_ascent_m: Option<u16>,
      total_descent_m: Option<u16>,
      avg_cadence: Option<u16>,
  }
  ```
- Handle device-specific quirks:
  - Garmin uses semicircles for GPS (convert: degrees = semicircles * (180 / 2^31))
  - Coros may have different field IDs for some metrics
  - Missing fields → None (not all watches have power, for example)

#### 4.2 Workout Upload Handler
- **File**: `src/api/workouts.rs`
- `POST /api/workouts/upload`:
  1. Receive multipart form data
  2. Validate file extension (.fit) and size (< 50MB)
  3. Save file to `data/fit_files/{athlete_id}/{date}_{id}.fit`
  4. Parse FIT file → records + session
  5. Compute derived metrics:
     - Per-km splits (pace, avg HR, elevation change)
     - Time in each HR zone (from records + athlete's zones)
     - Time in each pace zone (from records + athlete's pace zones)
  6. Match to planned workout by date
  7. Store in `completed_workouts` + `workout_records`
  8. Return parsed summary + records count + matched planned workout

#### 4.3 Workout Storage
- **Files**: `src/db/workouts.rs`
- **Migrations**: `003_create_workouts.sql`
- Bulk insert for `workout_records` (can be thousands of rows)
- Queries:
  - `get_workouts(athlete_id, from, to) -> Vec<WorkoutSummary>` (no time-series)
  - `get_workout(id) -> CompletedWorkout` (with summary metrics)
  - `get_workout_records(workout_id) -> Vec<WorkoutRecord>` (time-series for charts)

### Frontend Tasks

#### 4.4 Upload Button
- **File**: `frontend/src/components/workout/UploadButton.tsx`
- File input (accept=".fit")
- Drag-and-drop zone
- Upload progress indicator
- On success: navigate to workout detail page

#### 4.5 Workout Detail Page
- **File**: `frontend/src/pages/WorkoutDetail.tsx`
- Summary metrics row: duration, distance, avg pace, avg HR, elevation gain
- HR over time chart (`HRChart.tsx`):
  - Line chart of HR values over time
  - Background bands colored by HR zone
  - X-axis: elapsed time. Y-axis: BPM
- Pace over time chart (`PaceChart.tsx`):
  - Line chart of pace over time (inverted Y-axis: faster = higher)
  - Background bands colored by pace zone
  - X-axis: elapsed time. Y-axis: min/km
- Time-in-zone bars (`ZoneBar.tsx`):
  - Horizontal bar chart for HR zones (7 bars)
  - Horizontal bar chart for pace zones (6 bars)
  - Shows time in each zone (mm:ss)
- Splits table (`SplitsTable.tsx`):
  - Per-km rows: km number, pace, avg HR, elevation change
  - Color-coded pace column
- Matched prescription card (if linked to planned workout):
  - "Planned: 45min Easy Run, Zone 1-2"
  - (Scoring/compliance comes in Phase 5)

#### 4.6 Workout List
- Add to `Dashboard.tsx`: recent completed workouts list
- `GET /api/workouts?from=&to=` for workout history

### Tests
- FIT parser: test against real FIT files from different watches
- Keep test fixture files in `tests/fixtures/`
- Unit tests: per-km split calculation, zone-time calculation
- Integration tests: upload → parse → store → retrieve flow
- Edge cases: file with missing HR data, very short run, very long run

---

## Phase 5a: Workout Scoring & Classification

**Goal**: After upload, compute all training load metrics (rTSS, TRIMP, NGP, aerobic/anaerobic effects), classify the workout, and update ATL/CTL/TSB. No AI in this phase — pure deterministic scoring.

**Usable state after this phase**: Athlete uploads a workout and immediately sees all scores — rTSS, TRIMP, aerobic/anaerobic effect, classification badge, time-in-zone breakdown, and updated ATL/CTL/TSB. The dashboard shows real load metrics. The PMC chart shows CTL/ATL/TSB over time. This is valuable standalone — athletes see exactly what their training is doing physiologically.

### Backend Tasks

#### 5a.1 NGP & Scoring
- **File**: `src/domain/scoring.rs`
- `calculate_ngp(records: &[WorkoutRecord]) -> f64`:
  - For each pair of consecutive records, compute grade from altitude change / horizontal distance
  - Apply cost factor with downhill floor at 0.6
  - 30-second rolling average on adjusted speed
  - Return weighted average NGP
- `calculate_rtss(ngp: f64, ftpace: f64, duration_s: f64) -> f64`:
  - IF = NGP / FTPace
  - rTSS = (duration * NGP * IF) / (FTPace * 3600) * 100
- `calculate_trimp(records: &[WorkoutRecord], resting_hr: u16, max_hr: u16) -> f64`:
  - Per-second accumulation (not avg_HR):
  - `For each second: delta_hr = (HR(t) - resting) / (max - resting); TRIMP += (1/60) * delta_hr * 0.2445 * e^(3.411 * delta_hr)`

#### 5a.2 Aerobic/Anaerobic Effects
- **File**: `src/domain/effects.rs`
- `calculate_aerobic_effect(records, lthr, resting_hr, max_hr, ctl) -> f64`:
  - Per-second EPOC accumulation for samples ≤ 104% LTHR
  - Track peak EPOC
  - Normalize by fitness factor
  - Scale to 0-5
- `calculate_anaerobic_effect(records, lthr, ftpace, resting_hr, max_hr, ctl) -> f64`:
  - Detect high-intensity intervals
  - Score each by intensity, duration weight, duration
  - Aggregate, normalize, scale to 0-5
- Constants file or config for k1, k2, k3, scale factors (tunable)

#### 5a.3 Workout Classification
- **File**: `src/domain/classification.rs`
- `classify_workout(hr_zone_seconds, aerobic_effect, anaerobic_effect) -> Classification`:
  - Pure deterministic rules from §3.4
  - Returns enum: AerobicCapacity, AerobicUtilization, AnaerobicCapacity, AnaerobicUtilization, Mixed

#### 5a.4 ATL/CTL/TSB Update
- **File**: `src/domain/load_tracking.rs`
- `update_daily_metrics(user_id, date, tss, db) -> DailyMetrics`:
  - Get previous day's ATL/CTL (or bootstrap values)
  - Calculate today's ATL, CTL, TSB
  - Upsert into `daily_metrics`
- `backfill_rest_days(user_id, from_date, to_date, db)`:
  - Fill in days with TSS=0 between workouts
  - Called before any metrics read or workout upload
  - Important for correct CTL decay

#### 5a.5 Upload Pipeline Update
- Update `POST /api/workouts/upload` to include scoring (synchronous, fast):
  1. Parse FIT (existing)
  2. Check HR data quality (skip HR-based scoring if < 50% HR data present)
  3. Calculate NGP
  4. Calculate rTSS, TRIMP
  5. Calculate aerobic/anaerobic effect
  6. Classify workout
  7. Calculate zone time (store as individual columns)
  8. Determine compliance (if linked to planned workout)
  9. Backfill rest days, then update daily metrics (ATL/CTL/TSB)
  10. Store all scores in `completed_workouts`
  11. Return workout with all scores (no AI commentary yet)

#### 5a.6 Metrics API
- **Files**: `src/api/metrics.rs`, `src/db/metrics.rs`
- **Migration**: `005_create_daily_metrics.sql`
- `GET /api/metrics?from=&to=` → PMC chart data
- `GET /api/metrics/current` → latest ATL/CTL/TSB

### Frontend Tasks

#### 5a.7 Enhanced Workout Detail
- Update `WorkoutDetail.tsx` to show:
  - Scoring section: rTSS, TRIMP, intensity factor
  - Effect scores: aerobic (0-5 gauge/bar), anaerobic (0-5 gauge/bar)
  - Classification badge
  - Compliance indicator (vs. prescription)
  - Before/after ATL/CTL/TSB change
  - Warning banner if HR data was insufficient

#### 5a.8 Dashboard Metrics
- Update `Dashboard.tsx`:
  - Real CTL/ATL/TSB values (not placeholders)
  - Trend arrows based on 7-day direction
  - TSB interpretation text

#### 5a.9 PMC Chart
- **Files**: `frontend/src/pages/Performance.tsx`, `frontend/src/components/charts/PMCChart.tsx`
- Recharts line chart:
  - 3 lines: CTL (blue), ATL (red/orange), TSB (green/yellow)
  - Workout dots on timeline (colored by type)
  - Date range selector (30d, 60d, 90d, all)
  - Tooltip with exact values
  - Mesocycle boundary annotations
  - Race day marker (if in view)
  - Responsive sizing

### Tests
- **Calibration tests** (critical): `tests/calibration/` with real FIT files and expected score ranges
- Unit tests: NGP, rTSS, TRIMP (per-second), aerobic/anaerobic effects, classification, ATL/CTL/TSB
- Test HR data quality check (skip scoring when HR insufficient)
- Integration tests: upload → score → metrics update pipeline
- Frontend: chart renders with sample data, score display
- **Playwright E2E**: upload FIT file → see scores on workout detail → check PMC chart updates

---

## Phase 5b: AI Analysis & Coach Chat

**Goal**: Add AI-powered workout analysis (async after upload) and free-form coach chat.

**Usable state after this phase**: The full daily training loop is complete. Upload triggers async AI analysis that appears as coach commentary on the workout and in the chat timeline. Athletes can ask Coach Jan questions anytime. This is the core coaching experience.

### Backend Tasks

#### 5b.1 AI Workout Analysis (Async)
- **File**: `src/ai/handlers.rs`
- `analyze_workout_async(user_id, workout_id)`:
  - Spawned via `tokio::spawn` after upload returns
  - Build context: parsed metrics, scores, classification, prescription, ATL/CTL/TSB, app flags
  - Call Claude (Sonnet) with `analyze_workout` tool
  - Parse response: summary, compliance, commentaries, plan_adjustment (usually null)
  - Store coach commentary in `completed_workouts`
  - Store as chat message in `chat_messages`
  - On failure: retry up to 2 times; on total failure, log and leave commentary NULL
- `GET /api/workouts/:id/analysis`:
  - Return analysis if ready, 202 Accepted if still processing

#### 5b.2 App-Computed Flags
- Computed before calling Claude:
  - `consecutive_off_target_count`
  - `off_target_direction`: "harder" | "easier" | null
  - `adjustment_eligible`: boolean
  - `days_since_last_workout`
  - `current_mesocycle_week`
  - `days_until_race`

#### 5b.3 Coach Chat
- **Files**: `src/api/chat.rs`, `src/db/chat.rs`, `src/ai/handlers.rs`
- **Migration**: `006_create_chat_messages.sql`
- `POST /api/chat`:
  - Accept `{ message }` body
  - Store athlete message
  - Build context: profile, plan, recent workouts, metrics, last 20 messages or 4k tokens
  - Call Claude (Haiku) with Coach Jan persona
  - Store coach response
  - Return response
- `GET /api/chat/history?limit=50&before=<message_id>`:
  - Return chat messages (cursor-based pagination)
  - Include workout analysis messages in timeline

### Frontend Tasks

#### 5b.4 Coach Commentary on Workout Detail
- Update `WorkoutDetail.tsx`:
  - Poll `GET /api/workouts/:id/analysis` after upload
  - Show loading spinner while analysis pending
  - Render coach commentary (markdown) when ready

#### 5b.5 Chat Page
- **File**: `frontend/src/pages/Chat.tsx`
- Chat interface:
  - Message list with bubbles (athlete right-aligned, coach left-aligned)
  - Markdown rendering in coach messages
  - Input bar with send button (shadcn/ui components)
  - Loading indicator during response
  - Auto-scroll to bottom on new message
  - Workout analysis messages in timeline with link to workout detail
  - Load more (cursor pagination) for history

### Tests
- Integration tests: full upload → score → async analyze pipeline (mock Claude via wiremock)
- Test: Claude failure → workout still has scores, commentary is null
- Test: chat stores and retrieves messages correctly
- Frontend: chat message flow, analysis loading state
- **Playwright E2E**: upload FIT → see scores immediately → coach commentary appears → navigate to chat → send message → get response

---

## Phase 6: Plan Adjustment & Mesocycle Transition

**Goal**: The system detects patterns, proposes plan adjustments, and handles mesocycle transitions. The full training cycle is complete.

**Usable state after this phase**: The complete CoachJan experience. The training loop is closed: plan → train → analyze → adjust → transition → new plan. The coach actively monitors training patterns, proposes changes when warranted (and stays quiet when not), and transitions between mesocycles with full evaluation.

### Backend Tasks

#### 6.1 Pattern Detection
- **File**: `src/domain/load_tracking.rs` (extend)
- `detect_adjustment_triggers(athlete_id, db) -> AdjustmentFlags`:
  - Query last 5 completed workouts
  - Count consecutive off-target in same direction
  - Calculate ATL/CTL ratio
  - Check days since last workout
  - Return structured flags

#### 6.2 Plan Adjustment System
- **Migration**: new `plan_adjustments` table
- **Files**: `src/api/plans.rs` (extend), `src/db/plans.rs` (extend)
- When workout analysis returns `plan_adjustment != null`:
  - Validate proposed operations (target dates exist, workout types valid)
  - Store in `plan_adjustments` table as pending
  - Return to frontend for athlete review
- `POST /api/plan/adjustments/:id/accept`:
  - Apply operations to `planned_workouts`:
    - `swap_workout`: update type, duration, zones
    - `reduce_intensity`: update target zones
    - `increase_intensity`: update target zones
    - `add_recovery_day`: insert new recovery workout
    - `extend_mesocycle`: add weeks, update mesocycle end date
    - `skip_workout`: mark as skipped
  - Mark adjustment as accepted
- `POST /api/plan/adjustments/:id/reject`:
  - Mark adjustment as rejected, no plan changes

#### 6.3 Mesocycle Transition
- **File**: `src/ai/handlers.rs` (extend)
- Trigger: detect when current mesocycle's last workout is completed or end date passed
- `evaluate_mesocycle(athlete, mesocycle, metrics) -> MesocycleEvaluation`:
  - Build context: completed mesocycle summary, CTL trajectory vs target, weeks remaining
  - Call Claude with `evaluate_mesocycle` tool
  - Parse: assessment, ctl_vs_target, next phase recommendation, volume adjustment, strength progression
- After evaluation:
  - Mark current mesocycle as completed
  - Update next mesocycle parameters if needed
  - Generate weekly plans for next mesocycle (reuse Phase 2 generation)
  - If final mesocycle → transition to taper workflow

#### 6.4 Strength Programming
- **Migration**: movement_assessments, strength_progress tables
- Add movement assessment to onboarding (optional step)
- Strength sessions included in plan generation (already in tool schema)
- Strength level progression evaluated at mesocycle transition

#### 6.5 FTPace Re-validation
- Detect at mesocycle boundary
- If race result available → recalculate FTPace
- If time trial data → estimate FTPace
- Update zones if FTPace changes

### Frontend Tasks

#### 6.6 Adjustment Review UI
- When adjustment is pending, show notification banner
- Diff view: "Coach Jan proposes changes:"
  - For each operation: before/after comparison
  - Coach's explanation
  - Accept / Reject buttons
- Show in chat timeline as well

#### 6.7 Mesocycle Transition UI
- Transition view when mesocycle completes:
  - Summary of completed mesocycle (volume, compliance, CTL change)
  - Coach's evaluation message
  - Next mesocycle preview (phase, focus, volume target)
  - "Let's go" confirmation

#### 6.8 Movement Assessment (Onboarding Extension)
- Optional step in onboarding
- 11 self-assessment tests with pass/fail/limited options
- Instructions for each test (text + optional image/video link)

### Tests
- Unit tests: pattern detection with various workout sequences
- Integration tests: adjustment lifecycle (propose → accept/reject)
- Integration tests: mesocycle transition flow
- Test: no adjustment for single off-target (critical invariant)
- Test: adjustment after 3 consecutive off-target
- **Playwright E2E**: simulate adjustment trigger → see diff view → accept → verify plan changed

---

## Phase Summary

| Phase | Name | Backend | Frontend | Usable State |
|-------|------|---------|----------|-------------|
| 0 | Project Setup | Skeleton, DB, config | Vite + React + shadcn/ui scaffold | App starts and serves empty page |
| 1 | Auth & Profile | Auth, profile, zones | Login, onboarding, profile | Athlete can sign up and see their zones |
| 2 | Plan Generation | Claude API, plan gen, validation | Plan generation in onboarding | Athlete has a generated training plan |
| 3 | Plan Viewing | Plan query endpoints | Calendar, dashboard, workout cards | Athlete sees daily plan in calendar view |
| 4 | FIT Upload | FIT parser, upload handler | Upload button, workout charts, splits | Athlete uploads runs and sees data |
| 5a | Scoring | Scoring, effects, ATL/CTL/TSB | Enhanced workout view, PMC chart, dashboard metrics | Athlete sees workout scores and load tracking |
| 5b | AI Analysis & Chat | Async AI analysis, chat system | Coach commentary, chat page | Full daily training loop with coaching |
| 6 | Plan Adjustment | Pattern detection, adjustments, mesocycle transitions | Adjustment review, transition UI | Complete closed-loop coaching system |

---

## Development Notes for Agents

### Working on a Phase
1. Read the phase-specific doc in `docs/phases/PHASE_N_*.md` (to be generated at phase start)
2. Read `docs/ARCHITECTURE.md` for directory structure, DB schema, and API surface
3. Read `docs/FEATURES.md` for the relevant features' technical requirements
4. Write tests first (TDD)
5. Implement backend changes
6. Implement frontend changes — **invoke `/frontend-design` skill** before making any frontend design choices (layout, component structure, styling, responsive behavior, UX patterns)
7. Write Playwright E2E tests — **invoke `/playwright` skill** when writing or modifying Playwright tests
8. Run full test suite before marking complete
9. Add documentation for testing and using all new code you added so next agent knows how to use

### Required Skills
- **`/frontend-design`**: MUST be invoked before making frontend design decisions — component layout, page structure, responsive breakpoints, color choices, spacing, UX flows, accessibility patterns. This ensures consistent, high-quality UI across all phases.
- **`/playwright`**: MUST be invoked when writing, updating, or debugging Playwright E2E tests. This ensures tests follow best practices for selectors, assertions, and test structure.

### Cross-Phase Dependencies
- Phase 2 depends on Phase 1 (auth + profile must exist)
- Phase 3 depends on Phase 2 (plan data must exist to view)
- Phase 4 has no dependency on Phase 3 (upload is independent of viewing)
- Phase 5a depends on Phase 4 (scoring needs parsed workout data)
- Phase 5b depends on Phase 5a (AI analysis needs computed scores)
- Phase 6 depends on Phase 5b (adjustments need scoring + analysis)

### Testing Strategy
- **Rust unit tests**: all `src/domain/` modules (pure functions, no mocking)
- **Rust integration tests**: API endpoints with in-memory SQLite (`sqlite::memory:`), migrations run per test
- **Claude API mocking**: use `wiremock` crate to return canned tool_use responses in tests
- **Scoring calibration**: `tests/calibration/` with real FIT files + expected score ranges
- **Frontend component tests**: Vitest + Testing Library + MSW (Mock Service Worker)
- **Playwright E2E tests**: `e2e/` directory, test complete user flows against running app with seeded data
- Every phase adds Playwright tests validating its usable state

### Shared Conventions
- All dates: ISO 8601 strings (`2024-03-15`)
- All timestamps: ISO 8601 UTC with Z suffix (`2024-03-15T10:30:00Z`)
- All paces: stored as speed in m/s internally, displayed as min/km in UI
- All distances: stored in meters internally, displayed as km in UI
- API errors: `{ "error": "message", "code": "ERROR_CODE" }`
- All domain logic in `src/domain/` is pure (no I/O, no DB, no HTTP)
- All AI interaction goes through `src/ai/handlers.rs`
- Frontend forms use react-hook-form + zod for validation
- Frontend UI primitives from shadcn/ui (Radix + Tailwind)
- Frontend formatting utilities in `frontend/src/utils/formatting.ts`
- SQLite: always set `PRAGMA foreign_keys = ON` on connection init
