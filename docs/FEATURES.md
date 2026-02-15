# CoachJan — Feature Inventory & Technical Requirements

Each feature is mapped to an implementation phase and lists the Rust modules, API endpoints, frontend components, and key technical details an agent needs to build it.

---

## F1: Email + Password Authentication

**Phase**: 1 | **Priority**: Critical (blocks everything)

**What it does**: Athlete registers with email + password, then logs in with email + password. No email verification, no emails sent.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `api/auth.rs`, `db/users.rs`, `db/sessions.rs`, `auth/password.rs` |
| DB tables | `users` (email + password_hash), `sessions` |
| API endpoints | `POST /api/auth/register`, `POST /api/auth/login`, `POST /api/auth/logout`, `GET /api/auth/me` |
| Frontend pages | `Login.tsx` (login + register tabs/toggle) |
| Auth middleware | Axum extractor that reads session cookie, loads athlete from DB |
| Password hashing | Argon2id via `argon2` crate |
| Session format | UUID v4, stored in `sessions` table, sent as HTTP-only cookie |
| Session expiry | 30 days |
| Edge cases | Duplicate email on register → error. Wrong password → error. Expired session → 401. |

**Acceptance criteria**:
- Register with email + password → creates user row, session created, cookie set
- Login with correct email + password → session created, cookie set
- Login with wrong password → 401 error
- Register with existing email → 409 conflict error
- All authenticated routes return 401 without valid session cookie
- `GET /api/auth/me` returns athlete data if authenticated, 401 if not
- Passwords are never stored in plaintext (Argon2id hash only)

---

## F2: Athlete Profile Setup

**Phase**: 1 | **Priority**: Critical (blocks plan generation)

**What it does**: After first login, athlete fills out profile form. System calculates HR zones and pace zones from LTHR and FTPace.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `api/athletes.rs`, `domain/zones.rs`, `domain/bootstrap.rs`, `db/profiles.rs` |
| DB tables | `athlete_profiles`, `race_goals`, `ftpace_history`, `lthr_history` |
| API endpoints | `POST /api/athlete/profile`, `GET /api/athlete/profile`, `PUT /api/athlete/profile`, `GET /api/athlete/zones` |
| Frontend pages | `Onboarding.tsx` (multi-step form using react-hook-form + zod) |
| Frontend components | `ZoneTable.tsx` (shadcn/ui table) |

**Profile fields** (all required unless noted):
- name (string)
- age (integer)
- weight_kg (float)
- resting_hr (integer)
- max_hr (integer)
- lthr (integer)
- ftpace_m_per_s (float, optional — can be estimated from race result)
- current_weekly_volume_km (float)
- experience_level (enum: beginner/intermediate/advanced)
- sports_background (text, optional — free-form or structured JSON)
- goal race: distance_m (float), race_date (date string), race_name (string, optional)
- recent_race_results (optional): distance, time, date

**Zone calculation logic** (`domain/zones.rs`):

HR Zones (7-zone, from LTHR):
```
Zone 1: < 82% LTHR
Zone 2: 82-88% LTHR
Zone 3: 89-93% LTHR
Zone 4: 94-99% LTHR
Zone 5: 100-104% LTHR
Zone 6: 105-110% LTHR
Zone 7: > 110% LTHR
```

Pace Zones (6-zone, from FTPace as speed m/s):
```
Zone 1: < 75% FTPace
Zone 2: 75-85% FTPace
Zone 3: 86-95% FTPace
Zone 4: 96-105% FTPace
Zone 5: 106-120% FTPace
Zone 6: > 120% FTPace
```

**CTL Bootstrap** (`domain/bootstrap.rs`):
```
avg_pace_factor = { beginner: 0.65, intermediate: 0.75, advanced: 0.85 }
estimated_weekly_tss = weekly_km * avg_pace_factor * 5
estimated_daily_tss = estimated_weekly_tss / 7
initial_CTL = estimated_daily_tss
initial_ATL = estimated_daily_tss
```

**Acceptance criteria**:
- Profile form validates all required fields
- Zones calculate correctly from LTHR and FTPace
- Initial CTL/ATL are bootstrapped from reported volume
- Profile can be updated; zones recalculate on LTHR/FTPace change
- FTPace and LTHR changes are recorded in history tables
- Goal race is stored and linked to athlete

---

## F3: Training Plan Generation

**Phase**: 2 | **Priority**: Critical

**What it does**: Claude generates a macrocycle skeleton (mesocycle sequence), athlete reviews, then weekly plans are generated for the first mesocycle.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `ai/client.rs`, `ai/prompts.rs`, `ai/tools.rs`, `ai/context.rs`, `ai/handlers.rs`, `domain/validation.rs`, `db/plans.rs` |
| DB tables | `macrocycles`, `mesocycles`, `planned_workouts` |
| API endpoints | `POST /api/plan/generate`, `POST /api/plan/confirm` |
| Frontend pages | `Onboarding.tsx` (plan review step) |

**Claude API Integration** (`ai/client.rs`):
- HTTP client wrapping Anthropic's Messages API
- Supports `tool_use` (function calling)
- Handles streaming (optional) or batch responses
- Retry logic: on tool validation failure, retry with error context (max 2)
- Model selection: Sonnet for plan generation

**System prompt** (`ai/prompts.rs`):
- Base Coach Jan persona (cached via Anthropic prompt caching)
- Plan generation specific instructions
- Few-shot examples

**Tool schemas** (`ai/tools.rs`):
- `generate_macrocycle_skeleton`: mesocycle array with phase, focus, load/recovery weeks, target volume
- `generate_weekly_plan`: day-by-day workouts with type, duration, target zones, expected TSS

**Plan validation** (`domain/validation.rs`):
- Max 3 high-intensity sessions per week
- Volume increase ≤ 10% week-over-week (except recovery)
- Recovery weeks reduce volume 30-60%
- Weekly TSS within 0.5x-2.0x of CTL*7
- Workout type enum check
- No duplicate dates
- Min 1 rest/recovery day per week
- Max 1 long run per week
- Strength not on VO2max/speed days (warning only)

**Context assembly** (`ai/context.rs`):
- Full athlete profile
- Goal race details
- Current CTL
- Pre-computed: weeks until race, recommended mesocycle count/durations for level

**Two-phase generation flow**:
1. `POST /api/plan/generate` → calls Claude with `generate_macrocycle_skeleton` tool → returns skeleton for review
2. `POST /api/plan/confirm` → saves macrocycle + mesocycles → calls Claude with `generate_weekly_plan` for first mesocycle weeks → validates → saves planned workouts

**Acceptance criteria**:
- Macrocycle skeleton follows Olbrecht's capacity→utilization→taper progression
- Mesocycle durations match athlete level (beginner 1+1, intermediate 2+1, advanced 3+2)
- Weekly plans pass all validation rules
- Athlete can review skeleton before weekly plans are generated
- Plan generation completes within 30 seconds
- On Claude failure, athlete gets clear error message

---

## F4: Training Plan Viewing

**Phase**: 3 | **Priority**: High

**What it does**: Calendar view of the training plan. Athletes see upcoming workouts, completed workouts with compliance coloring, weekly summaries, and mesocycle context.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `api/plans.rs` (GET endpoints) |
| API endpoints | `GET /api/plan`, `GET /api/plan/week`, `GET /api/plan/workout/:id` |
| Frontend pages | `Plan.tsx`, `Dashboard.tsx` |
| Frontend components | `Calendar.tsx`, `WorkoutCard.tsx`, `WeekSummary.tsx` |

**Calendar component** (`Calendar.tsx`):
- CSS grid layout: 7 columns (Mon-Sun), rows = weeks
- Each cell shows: workout type icon/color, duration, primary target zone
- Completed workouts: colored border (green = on_target, yellow = off_target, red = missed/skipped)
- Click to expand workout detail
- Navigation: week/month view, prev/next buttons
- Current day highlighted
- Mesocycle boundaries shown as horizontal dividers with phase label

**WorkoutCard** (`WorkoutCard.tsx`):
- Compact: type icon, duration, target zones
- Expanded: full description, target paces/HR, expected TSS, coach notes
- Completed state: shows actual metrics alongside targets

**WeekSummary** (`WeekSummary.tsx`):
- Target vs actual: volume (km), TSS, number of sessions
- Phase context: "Week 2 of Mesocycle 1 — Aerobic Capacity Building"

**Dashboard** (`Dashboard.tsx`):
- Today's workout card (prominent)
- Current phase and mesocycle info
- CTL/ATL/TSB with trend arrows (↑↓→)
- Weekly progress bar (volume completed / target)
- Race countdown

**Acceptance criteria**:
- Calendar renders current and future weeks from plan data
- Past workouts show completion status and compliance
- Workout detail shows all prescribed targets
- Dashboard shows today's workout prominently
- Responsive layout works on mobile (stacked cards instead of calendar grid)

---

## F5: Manual FIT File Upload

**Phase**: 4 | **Priority**: Critical

**What it does**: Athlete uploads a FIT file from any GPS watch. The server parses it into structured data (time-series + summary metrics).

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `api/workouts.rs`, `fit/parser.rs`, `fit/records.rs`, `fit/types.rs`, `db/workouts.rs` |
| DB tables | `completed_workouts`, `workout_records` |
| API endpoints | `POST /api/workouts/upload`, `GET /api/workouts` (paginated), `GET /api/workouts/:id`, `GET /api/workouts/:id/records`, `DELETE /api/workouts/:id` |
| Frontend components | `UploadButton.tsx` |
| File handling | Axum multipart, FIT file stored on disk |
| FIT parsing | Prefer `fit_file` crate; only build custom parser with `nom` if existing crate is insufficient |

**FIT file format** (binary):
- Header: 14 bytes (file size, protocol version, profile version, data type)
- Records: sequence of definition messages + data messages
- Each data message maps to a FIT message type (file_id, session, lap, record, event, etc.)
- We primarily need: `record` messages (per-second data) and `session` messages (summary)
- CRC validation at end of file

**Parser design** (`fit/parser.rs`):
- Use `nom` for binary parsing combinators
- Parse header → iterate records → extract `record` and `session` messages
- Handle different FIT profiles (Garmin, Coros, Wahoo may use slightly different fields)
- Fields to extract per `record`: timestamp, heart_rate, speed, position_lat/long, altitude, cadence, power, distance
- Fields to extract from `session`: total_timer_time, total_distance, avg_heart_rate, max_heart_rate, avg_speed, max_speed, total_ascent, total_descent, avg_cadence

**Upload flow**:
1. Receive multipart upload
2. Save FIT file to disk (`data/fit_files/{athlete_id}/{date}_{id}.fit`)
3. Parse FIT binary → extract records + session summary
4. Compute derived metrics (avg pace, elevation, splits)
5. Store summary in `completed_workouts`, time-series in `workout_records`
6. Match to planned workout by date (if exists)
7. Return parsed summary + matched planned workout

**Acceptance criteria**:
- Accepts .fit files from Garmin, Coros, Wahoo, Suunto, Polar
- Parses to per-second records (HR, pace, GPS, elevation, cadence)
- Computes summary metrics (duration, distance, avg/max HR, avg/max pace, elevation gain)
- Stores original FIT file for re-processing
- Rejects non-FIT files with clear error
- Handles large files (2+ hour runs) without timeout
- Upload + parse completes in under 5 seconds

---

## F6: Workout Scoring & Classification

**Phase**: 5a | **Priority**: Critical

**What it does**: After parsing, compute training load metrics (rTSS, TRIMP, aerobic/anaerobic effect), classify the workout, and update daily load tracking.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `domain/scoring.rs`, `domain/effects.rs`, `domain/classification.rs`, `domain/load_tracking.rs` |
| DB tables | `completed_workouts` (score columns), `daily_metrics` |

**NGP calculation** (`domain/scoring.rs`):
```
For each second:
  grade = elevation_change / horizontal_distance
  cost_factor = 1 + (15.3 * grade) + (4.2 * grade^2)
  For downhill: cost_factor = max(0.6, cost_factor)
  adjusted_speed = actual_speed * cost_factor
NGP = rolling_average(adjusted_speed, 30s window) → weighted average
```

**rTSS calculation** (`domain/scoring.rs`):
```
IF = NGP / FTPace  (both in m/s)
rTSS = (duration_seconds * NGP * IF) / (FTPace * 3600) * 100
```

**TRIMP calculation** (`domain/scoring.rs`):
Per-second accumulation (not avg_HR — using avg_HR underestimates for intervals):
```
For each second t:
  delta_hr_t = (HR(t) - resting_HR) / (max_HR - resting_HR)
  TRIMP += (1/60) * delta_hr_t * 0.2445 * e^(3.411 * delta_hr_t)
```

**Aerobic effect** (`domain/effects.rs`):
- Estimate %VO2max from %HRR per second
- Accumulate EPOC: `rate = k1 * e^(k2 * intensity)`, decay = `k3 * EPOC`
- Only count samples ≤ 104% LTHR for aerobic
- Track peak EPOC
- Normalize by fitness (CTL-based factor)
- Scale to 0-5
- Constants: k1=0.1, k2=2.5, k3=0.002

**Anaerobic effect** (`domain/effects.rs`):
- Detect intervals where HR > 104% LTHR or pace > FTPace
- Filter intervals < 5s
- Score each: intensity_score * duration_weight * duration_min
- Aggregate, normalize by fitness, scale to 0-5

**Workout classification** (`domain/classification.rs`):
- Pure deterministic rules based on zone-time percentages and effect scores
- See §3.4 of PRODUCT_DESIGN.md for full decision tree

**ATL/CTL/TSB update** (`domain/load_tracking.rs`):
```
CTL_today = CTL_yesterday + (TSS_today - CTL_yesterday) / 42
ATL_today = ATL_yesterday + (TSS_today - ATL_yesterday) / 7
TSB = CTL_yesterday - ATL_yesterday
```
- Runs after every workout upload
- Also runs daily (backfill rest days with TSS=0)

**Zone time calculation**:
- Iterate time-series records
- For each second, determine HR zone and pace zone
- Accumulate seconds in each zone
- Store as individual columns in `completed_workouts` (hr_zone_1_seconds through hr_zone_7_seconds, pace_zone_1_seconds through pace_zone_6_seconds)

**HR data quality check**:
- After parsing, check what percentage of records have HR data
- If HR data < 50% present: set `hr_data_sufficient = false`, skip HR-based scoring (TRIMP, effects, HR zone time, classification)
- Still compute pace-based metrics (distance, pace, NGP, rTSS)
- Show warning to athlete: "Heart rate data was incomplete. Scoring is based on pace only."

**Acceptance criteria**:
- rTSS within 5% of TrainingPeaks for equivalent data
- TRIMP matches Coros formula (per-second accumulation)
- Aerobic/anaerobic effect scores calibrated against reference workouts in `tests/calibration/`
- Classification matches expected output for known workout patterns
- ATL/CTL/TSB update correctly daily
- Strength TSS estimates are added to daily totals
- Workouts with insufficient HR data are handled gracefully

---

## F7: Workout Analysis (AI Commentary)

**Phase**: 5b | **Priority**: High

**What it does**: After scoring (5a), Claude analyzes the workout as Coach Jan — comparing to prescription, commenting on training effect, providing load context. Runs as an async background task after upload.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `ai/handlers.rs` (workout analysis flow), `ai/context.rs`, `ai/tools.rs` |
| API endpoints | `GET /api/workouts/:id/analysis` (poll for async result) |
| Frontend pages | `WorkoutDetail.tsx`, `Chat.tsx` (analysis appears in chat) |
| Frontend components | `HRChart.tsx`, `PaceChart.tsx`, `ZoneBar.tsx`, `SplitsTable.tsx`, `MetricCard.tsx` |

**Claude interaction** (`analyze_workout` tool):
- Input: parsed metrics, computed scores, classification, prescribed workout, ATL/CTL/TSB, app flags
- Output: summary, compliance, training_effect_commentary, load_commentary, plan_adjustment (usually null), coach_message
- Model: Sonnet

**App-computed flags sent to Claude**:
- `consecutive_off_target_count`: number of recent workouts off-target in same direction
- `off_target_direction`: "harder" | "easier" | null
- `adjustment_eligible`: boolean (true if pattern warrants potential adjustment)
- `days_since_last_workout`: integer
- `current_mesocycle_week`: "Week 2 of 3, load phase"
- `days_until_race`: integer

**Workout detail view** (`WorkoutDetail.tsx`):
- HR over time chart (line chart, background colored by zone)
- Pace over time chart (line chart, background colored by zone)
- Time-in-zone bar charts (HR and pace)
- Splits table (per km: pace, HR, elevation)
- Metric cards: duration, distance, rTSS, TRIMP, aerobic/anaerobic effect, IF
- Coach commentary section
- Compliance indicator (vs. prescription)

**Acceptance criteria**:
- Upload → parse → score → analyze pipeline completes in under 30 seconds
- Commentary is specific to the workout data (not generic)
- Compliance comparison is accurate
- Charts render correctly with zone coloring
- Splits are calculated per km

---

## F8: Dashboard & Performance Chart

**Phase**: 5a (dashboard metrics) / 5b (PMC chart) | **Priority**: Medium

**What it does**: Dashboard shows today's workout, current metrics, and progress. PMC chart shows CTL/ATL/TSB over time.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| API endpoints | `GET /api/metrics`, `GET /api/metrics/current` |
| Frontend pages | `Dashboard.tsx`, `Performance.tsx` |
| Frontend components | `PMCChart.tsx` |

**PMC Chart** (`PMCChart.tsx`):
- Recharts line chart with 3 series: CTL (blue), ATL (red), TSB (yellow)
- X-axis: dates. Y-axis: load values
- Workout dots on timeline, colored by workout type
- Mesocycle boundary annotations
- Race day marker
- Tooltip showing exact values on hover
- Date range selector

**Dashboard metrics**:
- Current CTL, ATL, TSB with trend arrows
- TSB interpretation (negative = loading, positive = fresh)
- Weekly volume: completed/target km
- Race countdown: "52 days until [race name]"

**Acceptance criteria**:
- PMC chart renders with actual data from daily_metrics
- CTL/ATL/TSB values match manual calculation
- Chart is responsive and works on mobile
- Dashboard shows correct current-day information

---

## F9: Coach Chat

**Phase**: 5b | **Priority**: Medium

**What it does**: Free-form conversational interface with Coach Jan. Athlete can ask questions about training, physiology, their plan.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `ai/handlers.rs` (chat flow), `db/chat.rs` |
| DB tables | `chat_messages` |
| API endpoints | `POST /api/chat`, `GET /api/chat/history` |
| Frontend pages | `Chat.tsx` |
| Frontend components | `ChatWindow.tsx`, `Message.tsx`, `ChatInput.tsx` |

**Claude interaction**:
- Model: Haiku (fast, cheap)
- System prompt: full Coach Jan persona + guardrails
- Context: athlete profile, current plan, recent workouts, metrics, last 20 messages or 4k tokens
- No structured output — free-text response
- Guardrails: no plan modifications via chat, running topics only

**Chat UI**:
- Message bubbles (athlete right, coach left)
- Markdown rendering in coach messages
- Loading indicator while Claude responds
- Scroll-to-bottom on new message
- Workout analysis messages also appear in chat timeline

**Acceptance criteria**:
- Sub-3-second response time (Haiku)
- Coach stays in character
- Coach refuses to modify plan via chat
- Chat history persists across sessions
- Context includes recent workout data

---

## F10: Plan Adjustment

**Phase**: 6 | **Priority**: High

**What it does**: When sustained off-target patterns are detected, the system proposes plan adjustments. Athlete reviews and accepts/rejects.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `ai/handlers.rs` (adjustment flow), `domain/validation.rs` |
| DB tables | Need new `plan_adjustments` table |
| API endpoints | `GET /api/plan/adjustments`, `POST /api/plan/adjustments/:id/accept`, `POST /api/plan/adjustments/:id/reject` |
| Frontend components | Adjustment diff view in `Plan.tsx` or `Chat.tsx` |

**Additional DB table**:
```sql
CREATE TABLE plan_adjustments (
    id INTEGER PRIMARY KEY,
    athlete_id INTEGER NOT NULL REFERENCES athletes(id),
    trigger_reason TEXT NOT NULL,
    operations TEXT NOT NULL,          -- JSON array of operations
    explanation TEXT NOT NULL,
    proposed_by_workout_id INTEGER REFERENCES completed_workouts(id),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'rejected')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    resolved_at TEXT
);
```

**Trigger detection** (server-side, runs after each workout analysis):
- Track consecutive off-target count and direction
- Track ATL/CTL ratio
- Track days since last workout
- Set `adjustment_eligible = true` when triggers from §3.7 are met
- Pass flags to Claude during workout analysis

**Adjustment operations** (closed set):
- `swap_workout`: replace workout type on a specific date
- `reduce_intensity`: lower target zones for a workout
- `increase_intensity`: raise target zones
- `add_recovery_day`: insert rest/recovery day
- `extend_mesocycle`: add weeks to current mesocycle
- `skip_workout`: mark a planned workout as skipped

**Diff view**:
- Show before/after for each affected workout
- Coach's explanation of why
- Accept/Reject buttons
- On accept: apply operations to `planned_workouts` table

**Acceptance criteria**:
- No adjustment proposed for single off-target workout
- Adjustment proposed after 3+ consecutive off-target in same direction
- Adjustment proposed after 7+ day absence
- All adjustments are proposals (never auto-applied)
- Accepted adjustments correctly modify the plan
- Rejected adjustments are recorded but plan stays unchanged

---

## F11: Mesocycle Transition

**Phase**: 6 | **Priority**: High

**What it does**: At mesocycle boundaries, evaluate completed mesocycle and generate the next one.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| Backend modules | `ai/handlers.rs` (mesocycle evaluation flow) |
| AI tools | `evaluate_mesocycle`, then `generate_weekly_plan` for next mesocycle |

**Trigger**: When the last planned workout of a mesocycle is completed (or the mesocycle end date passes).

**Evaluation context sent to Claude**:
- Completed mesocycle summary: total volume, compliance rate, avg weekly TSS, CTL trend
- Pre-computed: CTL trajectory vs race-day target, weeks remaining, mesocycles remaining
- Current ATL/CTL/TSB
- Macrocycle skeleton (what was planned for next mesocycle)
- FTPace/LTHR re-validation data if available

**Output**: evaluation assessment, CTL vs target status, next phase recommendation, volume adjustment, strength progression, coach message.

**Then**: generate weekly plans for the next mesocycle (same as F3 phase 2).

**Acceptance criteria**:
- Mesocycle evaluation happens at boundary
- Evaluation considers macrocycle timeline
- Next mesocycle follows capacity→utilization→taper progression
- Generated weekly plans pass validation

---

## F12: Strength Programming (Running Rewired)

**Phase**: 6 (partial in 2) | **Priority**: Medium

**What it does**: Self-assessment during onboarding, strength/mobility exercises prescribed alongside running.

**Technical Requirements**:

| Component | Details |
|-----------|---------|
| DB tables | `movement_assessments`, `strength_progress` |
| Backend modules | Extensions to plan generation to include strength sessions |

**Onboarding addition**: Movement self-assessment (11 tests, pass/fail/limited). Results stored and passed to Claude during plan generation.

**Strength sessions in plan**: Already included in F3 plan generation tool schema (`strength_sessions` array). The `planned_workouts` table handles strength workout types (`strength_precision`, `strength_performance`, `strength_power`).

**Progression**: Level 1→4 tracked in `strength_progress`. Progression evaluated at mesocycle transitions.

**Acceptance criteria**:
- Assessment results stored and influence initial plan
- Strength sessions appear in weekly plans
- Progression advances at mesocycle boundaries when appropriate

---

## F13: Coros API Integration

**Phase**: Future (post-MVP) | **Priority**: Low (MVP has manual upload)

**What it does**: OAuth connection to Coros for automatic workout sync.

**Note**: Documented here for completeness. Not included in the implementation phases below. Manual FIT upload covers all watch brands for MVP.

---

## Feature-to-Phase Mapping

| Feature | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5a | Phase 5b | Phase 6 |
|---------|---------|---------|---------|---------|----------|----------|---------|
| F1: Auth | **Build** | | | | | | |
| F2: Profile | **Build** | | | | | | |
| F3: Plan Generation | | **Build** | | | | | |
| F4: Plan Viewing | | | **Build** | | | | |
| F5: FIT Upload | | | | **Build** | | | |
| F6: Scoring | | | | | **Build** | | |
| F7: Workout Analysis | | | | | | **Build** | |
| F8: Dashboard/PMC | | Partial | Partial | | **Build** | **Build** | |
| F9: Chat | | | | | | **Build** | |
| F10: Plan Adjustment | | | | | | | **Build** |
| F11: Mesocycle Transition | | | | | | | **Build** |
| F12: Strength | | Partial | | | | | **Build** |
