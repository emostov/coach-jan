# Phase 2: Training Plan Generation

## Goal
Athlete can generate a periodized macrocycle training plan via Claude AI, review the skeleton, confirm it, and see their daily workouts on the dashboard.

## Usable State After This Phase
An athlete with a profile and race goal can generate a full training plan. The plan includes mesocycles (training blocks) with weekly workouts, each with specific workout types, durations, HR/pace zones, and coach notes. The dashboard shows a race countdown, current week's workouts, and a coach message.

---

## Completion Requirements

### Backend
- [x] Database migration for macrocycles, mesocycles, planned_workouts tables
- [x] Workout registry with 33 workout types and 3 duration categories each
- [x] Plan validation rules (rest day, max intensity, volume increase, no duplicate dates, max one long run)
- [x] Claude API client with retry logic and rate limit handling
- [x] Tool schemas for macrocycle skeleton and mesocycle detail generation
- [x] System prompt encoding Olbrecht's training philosophy
- [x] Context assembly from athlete profile, race goal, CTL, and weeks until race
- [x] Plan CRUD operations (macrocycle, mesocycle, planned_workouts)
- [x] Plan generation orchestration (skeleton → detail → fill → validate → save)
- [x] API endpoints: POST /api/plan/generate, POST /api/plan/confirm, GET /api/plan
- [x] ProfileResponse includes race_goal for frontend plan generation flow

### Frontend
- [x] Plan API client functions (generatePlan, confirmPlan, getCurrentPlan)
- [x] TanStack Query hooks (useCurrentPlan, useGeneratePlan, useConfirmPlan)
- [x] Onboarding step 6: plan generation with coach avatar and rotating messages
- [x] Onboarding step 7: skeleton review with mesocycle cards and confirm button
- [x] Dashboard: race countdown with real data
- [x] Dashboard: training plan card with current mesocycle workouts
- [x] Dashboard: workout rows with type, duration, HR zones, color-coded borders
- [x] Dashboard: coach message from macrocycle

### Tests
- [x] Unit tests: workout registry, validation rules, context assembly, zone calculation
- [x] Unit tests: skeleton/mesocycle parsing, duration filling, week plan building
- [x] Integration tests: plan generation with wiremock (5 tests)
- [x] Integration tests: auth + profile + plan flows
- [x] Live Claude integration test (ignored, runs with ANTHROPIC_API_KEY)

---

## Files Created/Modified

### Migrations
- `migrations/003_create_training_plans.sql` — macrocycles, mesocycles, planned_workouts tables

### Backend Files
| File | Purpose |
|------|---------|
| `src/ai/mod.rs` | AI module root |
| `src/ai/client.rs` | Claude API client with retry, rate limiting, configurable base URL |
| `src/ai/prompts.rs` | Coach Jan system prompt with Olbrecht philosophy |
| `src/ai/tools.rs` | Tool schemas for macrocycle skeleton and mesocycle detail |
| `src/ai/context.rs` | Context assembly from athlete data |
| `src/ai/handlers.rs` | Plan generation orchestration (parse, fill, validate) |
| `src/db/plans.rs` | Plan CRUD (macrocycle, mesocycle, planned_workouts) |
| `src/domain/workouts.rs` | 33 workout types with templates and duration categories |
| `src/domain/validation.rs` | Week plan validation rules |
| `src/api/plans.rs` | Plan API endpoints (generate, confirm, get) |

### Frontend Files
| File | Purpose |
|------|---------|
| `frontend/src/api/plan.ts` | Plan API client functions |
| `frontend/src/api/types.ts` | Plan-related types (Macrocycle, Mesocycle, PlannedWorkout) |
| `frontend/src/hooks/usePlan.ts` | TanStack Query hooks for plan operations |
| `frontend/src/pages/Onboarding.tsx` | Steps 6-7 (plan generation + skeleton review) |
| `frontend/src/pages/Dashboard.tsx` | Real plan display, race countdown, coach message |

### Test Files
| File | Purpose |
|------|---------|
| `tests/integration_tests.rs` | Wiremock-based plan generation integration tests |
| `tests/live_claude_test.rs` | Live Claude API test (ignored by default) |

---

## API Endpoints

| Method | Path | Auth | Body | Response |
|--------|------|------|------|----------|
| POST | `/api/plan/generate` | Yes | `{ race_goal_id }` | `{ macrocycle, mesocycles }` (skeleton) |
| POST | `/api/plan/confirm` | Yes | `{ macrocycle_id }` | `{ macrocycle, mesocycles, workouts }` |
| GET | `/api/plan` | Yes | — | `{ macrocycle, mesocycles, workouts }` |

---

## Workout Types (33 total)

### Easy/Aerobic (green)
easy_run, recovery_run, long_run, long_run_progression, long_run_moderate, aerobic_development, moderate_run, shakeout_run, mixed_energy

### Tempo/Threshold (terra)
tempo_run, cruise_intervals, steady_run, progression_run, race_specific, lactate_clearance, under_over, time_trial

### VO2max/Anaerobic (red)
vo2max_intervals, track_200m, track_400m, track_800m, anaerobic_hills, anaerobic_flat, anaerobic_power, hill_sprints, fartlek_structured

### Strength/Drills (slate)
strength_precision, strength_performance, strength_power, form_drills, plyo_running

### Rest
rest

---

## Plan Generation Flow

```
1. POST /api/plan/generate { race_goal_id }
2. Build context from profile + race_goal + CTL + weeks_until_race
3. Claude generates macrocycle skeleton (tool_use)
4. Parse skeleton → return to frontend for review
5. POST /api/plan/confirm { macrocycle_id }
6. For each mesocycle: Claude generates weekly workout detail (tool_use)
7. Fill workout templates (duration, TSS, zones) from registry
8. Validate each week (rest day, intensity limits, volume)
9. Save all workouts to DB
10. GET /api/plan returns full plan with workouts
```

---

## How to Test

```bash
# Unit + integration tests
cargo test

# Frontend type checking
cd frontend && npx tsc --noEmit

# Live Claude test (requires API key)
ANTHROPIC_API_KEY=sk-... cargo test --test live_claude_test -- --ignored --nocapture
```
