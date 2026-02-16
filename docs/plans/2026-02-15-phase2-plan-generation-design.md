# Phase 2 Design: Training Plan Generation

## Overview

After an athlete completes onboarding (Phase 1), they generate a periodized training plan. The plan is built through a layered approach:

1. **Macrocycle skeleton** (Claude) — mesocycle sequence with phases, focus, and durations
2. **Mesocycle day-by-day shape** (Claude) — workout types and duration categories for every day in a single call
3. **Detail fill** (Server + Claude hybrid) — server fills specific prescriptions from a workout registry; Claude adds coach notes and tweaks for personalization

## Key Design Decisions

### Workout Registry (Hardcoded in Rust)

Instead of having Claude invent workout structures each time, we maintain a **workout registry** in `src/domain/workouts.rs`. Each workout type has a template with adjustable parameters. Claude selects the workout type and duration category; the server fills in the specific prescription from the registry using the athlete's zones.

This approach:
- Reduces token usage (Claude makes high-level decisions, not low-level workout design)
- Ensures consistent, well-structured workout prescriptions
- Makes plans deterministic once Claude selects the shape
- Keeps workout design expertise in code, not prompts

### Single Call Per Mesocycle

Instead of calling Claude once per week, we call Claude once for the entire mesocycle with all days that need workouts. This gives Claude full context to balance workouts across the entire mesocycle — respecting load/recovery week patterns, progressive overload, and workout type distribution.

### Three-Layer Generation

```
Layer 1: Macrocycle Skeleton (Claude)
  "3 mesocycles: capacity 3+2w, capacity 2+1w, utilization 2+1w, taper 1w"

Layer 2: Mesocycle Plan Shape (Claude)
  "Mon: easy_run/short, Tue: vo2max_intervals/medium, Wed: rest, ..."
  (one call for ALL days in the mesocycle)

Layer 3: Detail Fill (Server + Claude hybrid)
  Server: Looks up vo2max_intervals/medium template → "5x4min @ Zone 5, 2.5min jog"
  Server: Resolves Zone 5 → 168-176 bpm, 3:45-4:05/km for this athlete
  Claude: Adds coach notes, tweaks race-specific sessions
```

---

## Workout Registry Design

### Structure

Each workout type in the registry has:
- **Name and description**
- **Olbrecht category** (aerobic capacity, aerobic utilization, etc.)
- **Template blocks** (warmup, main, cooldown) with parameter slots
- **Duration categories** (short, medium, long) with default parameters
- **Target HR and pace zones**
- **Expected TSS range** per duration category

### Workout Types

From the product design (PRODUCT_DESIGN.md §3.2):

| Type Key | Name | Category | Structure |
|----------|------|----------|-----------|
| `easy_run` | Easy Run | Aerobic Capacity | Continuous @ Zone 1-2 |
| `long_run` | Long Run | Aerobic Capacity | Continuous @ Zone 1-2, extended |
| `aerobic_development` | Aerobic Development | Aerobic Capacity | ~60% easy + ~40% short fast strides/surges |
| `tempo_run` | Tempo Run | Aerobic Utilization | Continuous or cruise intervals @ Zone 3-4 |
| `vo2max_intervals` | VO2max Intervals | Aerobic Utilization | Intervals @ Zone 5, jog recovery |
| `speed_sprint` | Speed/Sprint Work | Anaerobic Capacity | 10-30s all-out, full recovery |
| `race_specific` | Race-Specific | Anaerobic Utilization | Intervals at goal race pace |
| `recovery_run` | Recovery Run | Recovery | Zone 1, short duration |
| `rest` | Rest Day | Recovery | No running |
| `strength_precision` | Strength (Precision) | Injury Prevention | Neuromuscular, 15-20 min |
| `strength_performance` | Strength (Performance) | Strength | Loaded movements, 20-30 min |
| `strength_power` | Strength (Power) | Power | Explosive/plyometric, 20-30 min |

### Duration Categories

Each running workout type defines 3 duration categories:

```
easy_run:
  short:  25-35 min
  medium: 40-50 min
  long:   55-70 min

long_run:
  short:  60-75 min
  medium: 80-100 min
  long:   105-135 min

vo2max_intervals:
  short:  warmup(10min) + 4x3min @ Z5 / 2min jog + cooldown(10min)  ~40min
  medium: warmup(15min) + 5x4min @ Z5 / 2.5min jog + cooldown(10min) ~55min
  long:   warmup(15min) + 6x5min @ Z5 / 3min jog + cooldown(10min)  ~70min

tempo_run:
  short:  warmup(10min) + 15min @ Z3-4 + cooldown(10min)  ~35min
  medium: warmup(10min) + 2x12min @ Z3-4 / 3min easy + cooldown(10min) ~50min
  long:   warmup(10min) + 25min @ Z3-4 + cooldown(10min)  ~45min

speed_sprint:
  short:  warmup(15min) + 6x15s all-out / 2min walk + cooldown(10min)  ~35min
  medium: warmup(15min) + 8x20s all-out / 2.5min walk + cooldown(10min) ~45min
  long:   warmup(15min) + 10x25s all-out / 3min walk + cooldown(10min) ~55min

aerobic_development:
  short:  25min easy with 4x20s strides  ~30min
  medium: 35min easy with 6x20s strides  ~40min
  long:   45min easy with 8x20s strides  ~50min

race_specific:
  short:  warmup(10min) + 3x5min @ race pace / 2min jog + cooldown(10min) ~40min
  medium: warmup(10min) + 4x8min @ race pace / 3min jog + cooldown(10min) ~60min
  long:   warmup(10min) + 3x12min @ race pace / 3min jog + cooldown(10min) ~60min

recovery_run:
  short:  20 min @ Z1
  medium: 25 min @ Z1
  long:   30 min @ Z1
```

The server resolves zone references to actual HR/pace values for the specific athlete.

### Expected TSS by Workout

| Type | Short | Medium | Long |
|------|-------|--------|------|
| easy_run | 25-35 | 40-55 | 55-75 |
| long_run | 60-80 | 85-110 | 110-150 |
| vo2max_intervals | 55-70 | 70-90 | 90-115 |
| tempo_run | 45-55 | 60-75 | 55-70 |
| speed_sprint | 35-45 | 45-60 | 55-70 |
| aerobic_development | 30-40 | 40-50 | 50-65 |
| race_specific | 50-65 | 70-85 | 70-85 |
| recovery_run | 10-15 | 15-20 | 18-25 |

---

## Claude API Client Design

### Raw reqwest Client

`src/ai/client.rs` — Direct HTTP calls to the Anthropic Messages API.

```
POST https://api.anthropic.com/v1/messages
Headers:
  x-api-key: {ANTHROPIC_API_KEY}
  anthropic-version: 2023-06-01
  content-type: application/json
Body:
  { model, system, messages, tools, max_tokens }
```

Features:
- **Retry on 429** with exponential backoff (1s, 2s, 4s — max 3 attempts)
- **Timeout**: 60 seconds
- **Model enum**: Sonnet (plan generation), Haiku (chat in Phase 5b), Opus (future)
- **Parse tool_use content blocks** from response
- **Error types**: RateLimit, Timeout, InvalidResponse, ApiError

### Request/Response Types

```rust
// Request
struct ClaudeRequest {
    model: Model,
    system: String,
    messages: Vec<Message>,
    tools: Vec<Tool>,
    max_tokens: u32,
}

struct Message { role: Role, content: Vec<ContentBlock> }
enum ContentBlock { Text(String), ToolUse { id, name, input: Value }, ToolResult { tool_use_id, content } }

// Response parsing
struct ClaudeResponse { content: Vec<ContentBlock>, stop_reason: String, usage: Usage }
```

---

## Plan Generation Flow

### Step 1: Generate Macrocycle Skeleton

**Endpoint**: `POST /api/plan/generate`

**Flow**:
1. Build context: athlete profile, goal race, current CTL, weeks until race, mesocycle recommendations for athlete level
2. Call Claude with `generate_macrocycle_skeleton` tool
3. Parse response: target_ctl, mesocycles array, coach_message
4. Return skeleton for athlete review (NOT persisted)

**Tool schema** (`generate_macrocycle_skeleton`):
```json
{
  "target_ctl": 65.0,
  "coach_message": "Based on your current fitness and 16 weeks until race day...",
  "mesocycles": [
    {
      "sequence_number": 1,
      "phase": "capacity",
      "focus": "aerobic_capacity",
      "load_weeks": 3,
      "recovery_weeks": 2,
      "target_volume_km": 45.0
    },
    ...
  ]
}
```

### Step 2: Confirm and Generate Mesocycle Plan

**Endpoint**: `POST /api/plan/confirm`

**Flow**:
1. Persist macrocycle + mesocycles to DB
2. Calculate all dates for the first mesocycle (start date, end date, each day)
3. Call Claude with `generate_mesocycle_plan` tool — passing all days that need workouts
4. Parse response: array of day assignments (workout_type + duration_category + intensity_notes)
5. Server: fill details from workout registry + athlete zones
6. Call Claude with `add_coach_notes` tool — passing the filled-in plan for personalization
7. Validate full mesocycle plan (domain/validation.rs)
8. On validation failure: retry step 4 with error context (max 2 retries)
9. Persist planned workouts
10. Return complete plan

**Tool schema** (`generate_mesocycle_plan`):
```json
{
  "input": {
    "mesocycle": { "phase": "capacity", "focus": "aerobic_capacity", "load_weeks": 3, "recovery_weeks": 2 },
    "days": ["2026-02-16", "2026-02-17", ..., "2026-03-22"],
    "athlete_level": "intermediate",
    "current_ctl": 35.0,
    "target_volume_km": 45.0,
    "available_workout_types": ["easy_run", "long_run", "vo2max_intervals", ...]
  },
  "output": {
    "weeks": [
      {
        "week_number": 1,
        "week_type": "load",
        "target_volume_km": 42.0,
        "target_weekly_tss": 280,
        "days": [
          { "date": "2026-02-16", "workout_type": "easy_run", "duration_category": "medium" },
          { "date": "2026-02-17", "workout_type": "vo2max_intervals", "duration_category": "short" },
          { "date": "2026-02-18", "workout_type": "rest" },
          { "date": "2026-02-19", "workout_type": "easy_run", "duration_category": "short" },
          { "date": "2026-02-20", "workout_type": "tempo_run", "duration_category": "medium" },
          { "date": "2026-02-21", "workout_type": "rest" },
          { "date": "2026-02-22", "workout_type": "long_run", "duration_category": "medium" }
        ]
      },
      ...
    ]
  }
}
```

**Tool schema** (`add_coach_notes`):
```json
{
  "input": {
    "filled_plan": [...],
    "athlete_profile": {...},
    "race_goal": {...}
  },
  "output": {
    "workout_notes": [
      { "date": "2026-02-17", "coach_note": "First VO2max session of the block — focus on hitting Zone 5 for the work intervals. If the last rep feels too hard, it's okay to cut it short." },
      { "date": "2026-02-22", "coach_note": "Long run building aerobic base. Keep it conversational. If you feel good in the last 20 min, pick up to Zone 2 pace." }
    ],
    "mesocycle_overview": "We're building your aerobic engine in this first mesocycle. Weeks 1-3 progressively increase volume, then weeks 4-5 back off for recovery and adaptation."
  }
}
```

---

## Plan Validation Rules

All deterministic, in `src/domain/validation.rs`:

| Rule | Constraint | On Failure |
|------|-----------|------------|
| Max intensity sessions | <= 3 per week (vo2max, speed, race_specific, tempo) | Error |
| Volume progression | <= 10% increase week-over-week (load weeks) | Error |
| Recovery week reduction | 30-60% volume reduction from load weeks | Error |
| Weekly TSS range | Between 0.5x and 2.0x of CTL * 7 | Error |
| Valid workout types | All types in registry enum | Error |
| No duplicate dates | One workout per date | Error |
| Rest days | >= 1 rest or recovery day per week | Error |
| Long runs | <= 1 per week | Error |
| Strength + intensity | Strength not on VO2max/speed days | Warning |

On validation failure: retry Claude call with error context (max 2 retries).

---

## Database Changes

### New Migration: `003_create_training_plans.sql`

Tables `macrocycles`, `mesocycles`, and `planned_workouts` as defined in ARCHITECTURE.md.

No schema changes from the architecture doc — the existing schema supports the workout registry approach since `planned_workouts` stores the final resolved prescription (workout_type, duration_min, target_hr_zones, target_pace_zones, expected_tss, description).

---

## New Backend Files

| File | Purpose |
|------|---------|
| `src/ai/mod.rs` | AI module root |
| `src/ai/client.rs` | Anthropic Messages API client (reqwest) |
| `src/ai/prompts.rs` | System prompt: Coach Jan persona + plan generation instructions |
| `src/ai/tools.rs` | Tool schemas: generate_macrocycle_skeleton, generate_mesocycle_plan, add_coach_notes |
| `src/ai/context.rs` | Context assembly: athlete profile, race goal, CTL, weeks-to-race |
| `src/ai/handlers.rs` | Orchestration: skeleton generation, mesocycle plan generation, detail fill |
| `src/domain/workouts.rs` | Workout registry: templates, duration categories, TSS estimates |
| `src/domain/validation.rs` | Plan validation rules |
| `src/db/plans.rs` | CRUD for macrocycles, mesocycles, planned_workouts |
| `src/api/plans.rs` | POST /api/plan/generate, POST /api/plan/confirm, GET /api/plan |

---

## New API Endpoints

| Method | Path | Auth | Body | Response |
|--------|------|------|------|----------|
| POST | `/api/plan/generate` | Yes | `{ race_goal_id }` | `{ skeleton, coach_message }` |
| POST | `/api/plan/confirm` | Yes | `{ skeleton }` | `{ macrocycle, mesocycles, workouts }` |
| GET | `/api/plan` | Yes | — | `{ macrocycle, mesocycles }` |

---

## Frontend Changes

### Onboarding Extension (Steps 6-7)

Add to `Onboarding.tsx`:

**Step 6: Plan Generation Loading**
- "Coach Jan is building your training plan..."
- POST /api/plan/generate
- Show loading state with coach persona message
- On success, transition to step 7

**Step 7: Skeleton Review**
- Display macrocycle overview: mesocycle list with phase, focus, duration, date ranges
- Coach's overview message
- "Confirm Plan" button → POST /api/plan/confirm
- Loading state while weekly plans generate
- On success, redirect to dashboard

### Dashboard Update

- Show "Your plan has been generated. Here's your first week..." on Dashboard.tsx
- Simple list view of upcoming workouts (placeholder until Phase 3 calendar)
- Each workout shows: date, type, duration, target zones

---

## Testing Strategy

### Unit Tests
- Each validation rule tested independently with passing and failing cases
- Workout registry: verify all types have valid templates, TSS ranges are sensible
- Context assembly: verify output includes all required fields

### Integration Tests (wiremock)
- Mock Claude API with canned tool_use responses
- Full flow: generate skeleton → confirm → validate → persist
- Test validation rejection triggers retry with error context
- Test API error handling (429, 500, timeout)

### Live Integration Test
- One test behind `ANTHROPIC_API_KEY` env check
- Calls real Claude API with a test athlete profile
- Validates response parses correctly and passes validation
- Skipped in CI (no API key)

### Frontend Tests
- Onboarding step 6-7 components render correctly
- Loading states display
- Plan display on dashboard

### Playwright E2E
- Full flow: register → onboard → generate plan → review skeleton → confirm → see plan on dashboard

---

## Context Assembly

### For Macrocycle Skeleton (`ai/context.rs`)

```
You are Coach Jan, creating a periodized macrocycle for:
- Athlete: {name}, {age}yo, {experience_level}
- Current fitness: CTL={ctl}, weekly volume={weekly_km}km
- Goal: {race_name} ({race_distance}), {race_date} ({weeks_until_race} weeks away)
- LTHR: {lthr}, FTPace: {ftpace} (shown as min/km for readability)
- Mesocycle structure for {level}: {load_weeks}+{recovery_weeks} weeks

Create a macrocycle following Olbrecht's capacity → utilization → taper progression.
```

### For Mesocycle Plan (`ai/context.rs`)

```
Generate day-by-day workout assignments for this mesocycle:
- Phase: {phase}, Focus: {focus}
- Duration: {total_weeks} weeks ({load_weeks} load + {recovery_weeks} recovery)
- Dates: {start_date} to {end_date}
- Athlete level: {experience_level}
- Current CTL: {ctl}
- Target volume: {target_volume_km} km/week (load weeks)

Available workout types: {list from registry}
Duration categories: short, medium, long

Rules:
- Max 3 intensity sessions per week
- Volume increase <= 10% week-over-week in load weeks
- Recovery weeks: reduce volume 30-60%
- At least 1 rest day per week
- Max 1 long run per week
- Don't schedule strength on VO2max/speed days
```

---

## System Prompt (Coach Jan Persona)

Cached via Anthropic prompt caching. Includes:
- Coach Jan identity and philosophy (Olbrecht's framework)
- Plan stability rules (don't over-adjust)
- Capacity-first progression principle
- Workout type definitions and their Olbrecht categories
- Intensity distribution guidelines (85-90% easy in capacity, shift in competition)
- Guardrails (no medical advice, running topics only)
