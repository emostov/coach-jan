# Phase 3: Training Plan Viewing — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Calendar view of the training plan + enhanced dashboard with today's workout.

**Architecture:** Extend `GET /api/plan` to return all workouts grouped by mesocycle. Add `GET /api/plan/workout/:id` for deep-linking. Build Plan.tsx page with CSS grid calendar, workout cards, weekly summaries, and macrocycle timeline. Enhance Dashboard with today's workout hero card.

**Tech Stack:** Rust/Axum, SQLite/sqlx, React 19, TypeScript, Tailwind CSS v4, TanStack Query, date-fns, Zustand

**Design doc:** `docs/plans/2026-02-16-phase3-plan-viewing-design.md`

---

## Task 1: DB Migration — Add `target_distance_km` Column

**Files:**
- Create: `migrations/004_add_target_distance_km.sql`

**Step 1: Write the migration**

```sql
-- Add target_distance_km to planned_workouts for weekly mileage summaries
ALTER TABLE planned_workouts ADD COLUMN target_distance_km REAL;
```

**Step 2: Run migration to verify it applies**

Run: `cargo test test_router_has_correct_routes -- --nocapture` (any test that runs migrations)
Expected: PASS (migrations run without error)

**Step 3: Commit**

```bash
git add migrations/004_add_target_distance_km.sql
git commit -m "feat: add target_distance_km column to planned_workouts"
```

---

## Task 2: Update Rust DB Structs for `target_distance_km`

**Files:**
- Modify: `src/db/plans.rs` — `PlannedWorkout` struct (~line 217), `CreatePlannedWorkout` struct (~line 236), `create_planned_workout` fn (~line 251), `get_planned_workouts` fn (~line 301)

**Step 1: Write failing test**

Add to `src/db/plans.rs` tests:

```rust
#[tokio::test]
async fn test_create_planned_workout_with_distance() {
    let pool = setup_pool().await;
    let user_id = create_test_user(&pool).await;
    let race_goal_id = create_test_race_goal(&pool, user_id).await;
    let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
    let meso = create_test_mesocycle(&pool, mc.id).await;

    let input = CreatePlannedWorkout {
        mesocycle_id: meso.id,
        user_id,
        scheduled_date: "2026-03-03".to_string(),
        workout_type: "easy_run".to_string(),
        duration_min: Some(45),
        duration_category: Some("medium".to_string()),
        target_hr_zones: None,
        target_pace_zones: None,
        expected_tss: Some(35.0),
        description: None,
        coach_notes: None,
        target_distance_km: Some(8.5),
    };

    let workout = create_planned_workout(&pool, &input)
        .await
        .expect("should succeed");
    assert_eq!(workout.target_distance_km, Some(8.5));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_create_planned_workout_with_distance -- --nocapture`
Expected: FAIL (no field `target_distance_km` on struct)

**Step 3: Add `target_distance_km` to structs and queries**

In `PlannedWorkout` struct, add after `coach_notes`:
```rust
pub target_distance_km: Option<f64>,
```

In `CreatePlannedWorkout` struct, add after `coach_notes`:
```rust
pub target_distance_km: Option<f64>,
```

Update `create_planned_workout` INSERT query to include `target_distance_km` in both the column list and VALUES, bind it, and add to RETURNING clause. Add to the `Ok(PlannedWorkout { ... })` construction.

Update `get_planned_workouts` SELECT query to include `target_distance_km` in the column list. Add to the struct construction in the `.map()`.

**Step 4: Fix all existing `CreatePlannedWorkout` usage**

Search for all places constructing `CreatePlannedWorkout` and add `target_distance_km: None`:
- `src/db/plans.rs` tests (multiple test functions)
- `src/ai/handlers.rs` `persist_workouts` function (~line 806)
- `tests/integration_tests.rs` (if any)

**Step 5: Run tests to verify they pass**

Run: `cargo test`
Expected: All tests PASS

**Step 6: Commit**

```bash
git add src/db/plans.rs src/ai/handlers.rs
git commit -m "feat: add target_distance_km to PlannedWorkout structs and queries"
```

---

## Task 3: New DB Query — Get All Plan Workouts Grouped by Mesocycle

**Files:**
- Modify: `src/db/plans.rs` — add `get_plan_with_all_workouts` function

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_get_plan_with_all_workouts() {
    let pool = setup_pool().await;
    let user_id = create_test_user(&pool).await;
    let race_goal_id = create_test_race_goal(&pool, user_id).await;
    let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
    let meso1 = create_test_mesocycle(&pool, mc.id).await;

    // Create a second mesocycle
    let input2 = CreateMesocycle {
        macrocycle_id: mc.id,
        sequence_number: 2,
        phase: "utilization".to_string(),
        focus: "aerobic_utilization".to_string(),
        load_weeks: 2,
        recovery_weeks: 1,
        target_volume_km: Some(140.0),
        start_date: "2026-03-29".to_string(),
        end_date: "2026-04-18".to_string(),
    };
    let meso2 = create_mesocycle(&pool, &input2).await.expect("create meso2");

    // Add workouts to both mesocycles
    let w1 = CreatePlannedWorkout {
        mesocycle_id: meso1.id, user_id,
        scheduled_date: "2026-03-03".to_string(),
        workout_type: "easy_run".to_string(),
        duration_min: Some(45), duration_category: None,
        target_hr_zones: None, target_pace_zones: None,
        expected_tss: Some(35.0), description: None, coach_notes: None,
        target_distance_km: Some(8.0),
    };
    create_planned_workout(&pool, &w1).await.expect("w1");

    let w2 = CreatePlannedWorkout {
        mesocycle_id: meso2.id, user_id,
        scheduled_date: "2026-04-01".to_string(),
        workout_type: "tempo_run".to_string(),
        duration_min: Some(50), duration_category: None,
        target_hr_zones: None, target_pace_zones: None,
        expected_tss: Some(70.0), description: None, coach_notes: None,
        target_distance_km: Some(10.0),
    };
    create_planned_workout(&pool, &w2).await.expect("w2");

    let result = get_plan_with_all_workouts(&pool, user_id)
        .await
        .expect("should not error")
        .expect("should find plan");

    let (found_mc, meso_with_workouts) = result;
    assert_eq!(found_mc.id, mc.id);
    assert_eq!(meso_with_workouts.len(), 2);
    assert_eq!(meso_with_workouts[0].workouts.len(), 1);
    assert_eq!(meso_with_workouts[0].workouts[0].workout_type, "easy_run");
    assert_eq!(meso_with_workouts[1].workouts.len(), 1);
    assert_eq!(meso_with_workouts[1].workouts[0].workout_type, "tempo_run");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_get_plan_with_all_workouts -- --nocapture`
Expected: FAIL (function not found)

**Step 3: Implement `MesocycleWithWorkouts` and `get_plan_with_all_workouts`**

Add to `src/db/plans.rs`:

```rust
#[derive(Debug, Clone, Serialize)]
pub struct MesocycleWithWorkouts {
    #[serde(flatten)]
    pub mesocycle: Mesocycle,
    pub workouts: Vec<PlannedWorkout>,
}

/// Get the full plan: active macrocycle with all mesocycles and their workouts.
pub async fn get_plan_with_all_workouts(
    pool: &SqlitePool,
    user_id: i64,
) -> AppResult<Option<(Macrocycle, Vec<MesocycleWithWorkouts>)>> {
    let macrocycle = match get_current_macrocycle(pool, user_id).await? {
        Some(mc) => mc,
        None => return Ok(None),
    };

    let mesocycles = get_mesocycles(pool, macrocycle.id).await?;

    let mut result = Vec::new();
    for meso in mesocycles {
        let workouts = get_planned_workouts(pool, meso.id).await?;
        result.push(MesocycleWithWorkouts {
            mesocycle: meso,
            workouts,
        });
    }

    Ok(Some((macrocycle, result)))
}
```

**Step 4: Run tests**

Run: `cargo test test_get_plan_with_all_workouts -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add src/db/plans.rs
git commit -m "feat: add get_plan_with_all_workouts query"
```

---

## Task 4: New DB Query — Get Single Workout With Mesocycle Context

**Files:**
- Modify: `src/db/plans.rs`

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_get_workout_with_context() {
    let pool = setup_pool().await;
    let user_id = create_test_user(&pool).await;
    let race_goal_id = create_test_race_goal(&pool, user_id).await;
    let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
    let meso = create_test_mesocycle(&pool, mc.id).await;

    let w = CreatePlannedWorkout {
        mesocycle_id: meso.id, user_id,
        scheduled_date: "2026-03-03".to_string(),
        workout_type: "easy_run".to_string(),
        duration_min: Some(45), duration_category: None,
        target_hr_zones: None, target_pace_zones: None,
        expected_tss: Some(35.0), description: None, coach_notes: None,
        target_distance_km: None,
    };
    let workout = create_planned_workout(&pool, &w).await.expect("create workout");

    let (found_workout, meso_context) = get_workout_with_context(&pool, workout.id, user_id)
        .await
        .expect("should not error")
        .expect("should find workout");

    assert_eq!(found_workout.id, workout.id);
    assert_eq!(meso_context.phase, "capacity");
    assert_eq!(meso_context.focus, "aerobic_capacity");
    assert_eq!(meso_context.sequence_number, 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_get_workout_with_context -- --nocapture`
Expected: FAIL

**Step 3: Implement**

```rust
/// Minimal mesocycle context for workout detail view.
#[derive(Debug, Clone, Serialize)]
pub struct MesocycleContext {
    pub id: i64,
    pub phase: String,
    pub focus: String,
    pub sequence_number: i64,
}

/// Get a single workout with its mesocycle context. Verifies user ownership.
pub async fn get_workout_with_context(
    pool: &SqlitePool,
    workout_id: i64,
    user_id: i64,
) -> AppResult<Option<(PlannedWorkout, MesocycleContext)>> {
    let row = sqlx::query(
        r#"SELECT pw.id, pw.mesocycle_id, pw.user_id, pw.scheduled_date, pw.workout_type,
                  pw.duration_min, pw.duration_category, pw.target_hr_zones, pw.target_pace_zones,
                  pw.expected_tss, pw.description, pw.coach_notes, pw.target_distance_km,
                  pw.is_completed, pw.completed_workout_id, pw.created_at,
                  m.id as meso_id, m.phase, m.focus, m.sequence_number
           FROM planned_workouts pw
           JOIN mesocycles m ON pw.mesocycle_id = m.id
           WHERE pw.id = ? AND pw.user_id = ?"#,
    )
    .bind(workout_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let workout = PlannedWorkout {
            id: r.get("id"),
            mesocycle_id: r.get("mesocycle_id"),
            user_id: r.get("user_id"),
            scheduled_date: r.get("scheduled_date"),
            workout_type: r.get("workout_type"),
            duration_min: r.get("duration_min"),
            duration_category: r.get("duration_category"),
            target_hr_zones: r.get("target_hr_zones"),
            target_pace_zones: r.get("target_pace_zones"),
            expected_tss: r.get("expected_tss"),
            description: r.get("description"),
            coach_notes: r.get("coach_notes"),
            target_distance_km: r.get("target_distance_km"),
            is_completed: r.get("is_completed"),
            completed_workout_id: r.get("completed_workout_id"),
            created_at: r.get("created_at"),
        };
        let context = MesocycleContext {
            id: r.get("meso_id"),
            phase: r.get("phase"),
            focus: r.get("focus"),
            sequence_number: r.get("sequence_number"),
        };
        (workout, context)
    }))
}
```

**Step 4: Run tests**

Run: `cargo test test_get_workout_with_context -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add src/db/plans.rs
git commit -m "feat: add get_workout_with_context query"
```

---

## Task 5: Update `GET /api/plan` Handler + Add `GET /api/plan/workout/:id`

**Files:**
- Modify: `src/api/plans.rs` — update `get_plan` handler (~line 122), add `get_workout` handler, update `router()` (~line 153)

**Step 1: Write integration test for updated GET /api/plan**

Add to `tests/integration_tests.rs` (or `src/api/plans.rs` tests):

```rust
#[tokio::test]
async fn test_get_plan_returns_all_workouts() {
    // Setup: create user, profile, macrocycle, 2 mesocycles, workouts in each
    // Call GET /api/plan
    // Assert response has mesocycles[0].workouts and mesocycles[1].workouts
}
```

**Step 2: Update `get_plan` handler**

Replace the current `get_plan` handler body to use `get_plan_with_all_workouts`:

```rust
async fn get_plan(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
) -> AppResult<impl IntoResponse> {
    let plan = plans_db::get_plan_with_all_workouts(&state.db, auth.user_id).await?;

    match plan {
        Some((macrocycle, mesocycles)) => {
            Ok(Json(serde_json::json!({
                "macrocycle": macrocycle,
                "mesocycles": mesocycles
            })))
        }
        None => Err(AppError::NotFound(
            "No active training plan found".to_string(),
        )),
    }
}
```

**Step 3: Add `get_workout` handler**

```rust
async fn get_workout(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    axum::extract::Path(workout_id): axum::extract::Path<i64>,
) -> AppResult<impl IntoResponse> {
    let result = plans_db::get_workout_with_context(&state.db, workout_id, auth.user_id).await?;

    match result {
        Some((workout, mesocycle)) => {
            Ok(Json(serde_json::json!({
                "workout": workout,
                "mesocycle": mesocycle
            })))
        }
        None => Err(AppError::NotFound("Workout not found".to_string())),
    }
}
```

**Step 4: Update router**

```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", axum::routing::post(generate_plan))
        .route("/confirm", axum::routing::post(confirm_plan))
        .route("/", axum::routing::get(get_plan))
        .route("/workout/{id}", axum::routing::get(get_workout))
}
```

Note: Axum 0.8 uses `{id}` syntax for path params, not `:id`.

**Step 5: Run tests**

Run: `cargo test`
Expected: All PASS

**Step 6: Commit**

```bash
git add src/api/plans.rs
git commit -m "feat: update GET /api/plan to return all workouts, add GET /api/plan/workout/:id"
```

---

## Task 6: Update AI Tool Schema for `target_distance_km`

**Files:**
- Modify: `src/ai/tools.rs` — `generate_mesocycle_plan_tool()` day properties (~line 91)
- Modify: `src/ai/handlers.rs` — `ClaudeDay` struct (~line 87), `persist_workouts` (~line 806)

**Step 1: Add `target_distance_km` to tool schema**

In `generate_mesocycle_plan_tool()`, inside the `days` item properties (after `duration_category`), add:

```json
"target_distance_km": {
    "type": "number",
    "description": "Target distance in kilometers for this workout. Required for running workouts, omit for rest/strength."
}
```

**Step 2: Add field to `ClaudeDay` struct**

```rust
#[derive(Debug, Clone, Deserialize)]
struct ClaudeDay {
    date: String,
    workout_type: String,
    duration_category: Option<String>,
    target_distance_km: Option<f64>,
}
```

**Step 3: Pass through in `persist_workouts`**

In `persist_workouts`, when constructing `CreatePlannedWorkout`, use `f.target_distance_km` instead of `None`. To do this, also add `target_distance_km: Option<f64>` to `FilledWorkout` struct in `handlers.rs` and populate it from `ClaudeDay.target_distance_km` in `fill_workouts_from_registry`.

Actually simpler: pass `target_distance_km` through directly from `ClaudeDay` since the registry doesn't compute distance:

In `persist_workouts`, change `target_distance_km: None` to reference the day's value. This requires threading the `ClaudeDay` data through. The simplest approach: add `target_distance_km: Option<f64>` to `FilledWorkout` and populate it from `ClaudeDay` during `fill_workouts_from_registry`.

**Step 4: Run tests**

Run: `cargo test`
Expected: All PASS

**Step 5: Commit**

```bash
git add src/ai/tools.rs src/ai/handlers.rs
git commit -m "feat: add target_distance_km to tool schema and plan generation pipeline"
```

---

## Task 7: Frontend Types + API Client Updates

**Files:**
- Modify: `frontend/src/api/types.ts` — update `PlanResponse`, `PlannedWorkout`, add `MesocycleWithWorkouts`, `MesocycleContext`, `WorkoutDetailResponse`
- Modify: `frontend/src/api/plan.ts` — add `getWorkout` function
- Modify: `frontend/src/hooks/usePlan.ts` — add `useWorkout` hook

**Step 1: Update types**

In `frontend/src/api/types.ts`:

```typescript
// Update PlannedWorkout - add target_distance_km
export interface PlannedWorkout {
  // ... existing fields ...
  target_distance_km: number | null;
}

// Replace PlanResponse
export interface PlanResponse {
  macrocycle: Macrocycle;
  mesocycles: MesocycleWithWorkouts[];
}

export interface MesocycleWithWorkouts extends Mesocycle {
  workouts: PlannedWorkout[];
}

export interface MesocycleContext {
  id: number;
  phase: string;
  focus: string;
  sequence_number: number;
}

export interface WorkoutDetailResponse {
  workout: PlannedWorkout;
  mesocycle: MesocycleContext;
}
```

Remove the old `PlanResponse` that had a flat `workouts` array.

**Step 2: Update API client**

In `frontend/src/api/plan.ts`, add:

```typescript
export function getWorkout(workoutId: number): Promise<WorkoutDetailResponse> {
  return apiFetch(`/plan/workout/${workoutId}`);
}
```

**Step 3: Update hooks**

In `frontend/src/hooks/usePlan.ts`, add:

```typescript
export function useWorkout(workoutId: number | null) {
  return useQuery({
    queryKey: ['workout', workoutId],
    queryFn: () => planApi.getWorkout(workoutId!),
    enabled: workoutId !== null,
  });
}
```

**Step 4: Run type check**

Run: `cd frontend && npx tsc --noEmit`
Expected: Type errors in Dashboard.tsx (it references `planData.workouts` which no longer exists on `PlanResponse`). That's expected — we'll fix it in Task 12 (Dashboard enhancements).

**Step 5: Commit**

```bash
git add frontend/src/api/types.ts frontend/src/api/plan.ts frontend/src/hooks/usePlan.ts
git commit -m "feat: update frontend types and API client for Phase 3 plan response"
```

---

## Task 8: Extract Workout Colors + Add Plan Utilities

**Files:**
- Create: `frontend/src/utils/workoutColors.ts`
- Create: `frontend/src/utils/planHelpers.ts`
- Modify: `frontend/src/pages/Dashboard.tsx` — import colors from new location

**Step 1: Extract WORKOUT_COLORS**

Move the `WORKOUT_COLORS` map and `getWorkoutBorderColor` from `Dashboard.tsx` to `frontend/src/utils/workoutColors.ts`:

```typescript
export const WORKOUT_COLORS: Record<string, string> = {
  // ... same map as in Dashboard.tsx ...
};

export function getWorkoutBorderColor(workoutType: string): string {
  return WORKOUT_COLORS[workoutType] ?? '#94a3b8';
}
```

**Step 2: Create plan helpers**

Create `frontend/src/utils/planHelpers.ts`:

```typescript
import { startOfISOWeek, endOfISOWeek, isWithinInterval, parseISO, format, isSameDay } from 'date-fns';
import type { PlanResponse, MesocycleWithWorkouts, PlannedWorkout } from '../api/types';

/** Get all workouts from all mesocycles as a flat array. */
export function getAllWorkouts(plan: PlanResponse): PlannedWorkout[] {
  return plan.mesocycles.flatMap(m => m.workouts);
}

/** Get today's workout, if any. */
export function getTodaysWorkout(plan: PlanResponse): PlannedWorkout | null {
  const today = new Date();
  return getAllWorkouts(plan).find(w => isSameDay(parseISO(w.scheduled_date), today)) ?? null;
}

/** Get workouts for the ISO week containing the given date. */
export function getWeekWorkouts(plan: PlanResponse, date: Date): PlannedWorkout[] {
  const weekStart = startOfISOWeek(date);
  const weekEnd = endOfISOWeek(date);
  return getAllWorkouts(plan).filter(w =>
    isWithinInterval(parseISO(w.scheduled_date), { start: weekStart, end: weekEnd })
  );
}

/** Calculate weekly summary from a list of workouts. */
export function getWeekSummary(workouts: PlannedWorkout[]) {
  const targetKm = workouts.reduce((sum, w) => sum + (w.target_distance_km ?? 0), 0);
  const targetTss = workouts.reduce((sum, w) => sum + (w.expected_tss ?? 0), 0);
  const targetMinutes = workouts.reduce((sum, w) => sum + (w.duration_min ?? 0), 0);
  const totalSessions = workouts.filter(w => w.workout_type !== 'rest').length;
  const completedSessions = workouts.filter(w => w.is_completed === 1).length;

  return { targetKm, targetTss, targetMinutes, totalSessions, completedSessions };
}

/** Find the mesocycle that contains the given date. */
export function getMesocycleForDate(plan: PlanResponse, date: Date): MesocycleWithWorkouts | null {
  return plan.mesocycles.find(m => {
    const start = parseISO(m.start_date);
    const end = parseISO(m.end_date);
    return date >= start && date <= end;
  }) ?? null;
}

/** Get week number within a mesocycle for a given date. */
export function getWeekOfMesocycle(mesocycle: MesocycleWithWorkouts, date: Date): number {
  const start = startOfISOWeek(parseISO(mesocycle.start_date));
  const current = startOfISOWeek(date);
  return Math.floor((current.getTime() - start.getTime()) / (7 * 24 * 60 * 60 * 1000)) + 1;
}

/** Format workout type from snake_case to Title Case. */
export function formatWorkoutType(snakeCase: string): string {
  return snakeCase.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}

/** Format phase name. */
export function formatPhase(phase: string): string {
  return formatWorkoutType(phase);
}

/** Format minutes to hours:minutes display. */
export function formatMinutes(minutes: number): string {
  const hrs = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hrs > 0) return `${hrs}h ${mins > 0 ? `${mins}m` : ''}`.trim();
  return `${mins}m`;
}
```

**Step 3: Update Dashboard imports**

In `Dashboard.tsx`, replace the local `WORKOUT_COLORS` and `getWorkoutBorderColor` with imports from `../utils/workoutColors`. Also replace local `formatWorkoutType` and `formatPhase` with imports from `../utils/planHelpers`.

**Step 4: Run type check**

Run: `cd frontend && npx tsc --noEmit`
Expected: May still have errors from the PlanResponse change — that's OK, will be fixed in Task 12.

**Step 5: Commit**

```bash
git add frontend/src/utils/workoutColors.ts frontend/src/utils/planHelpers.ts frontend/src/pages/Dashboard.tsx
git commit -m "feat: extract workout colors and add plan helper utilities"
```

---

## Task 9: MacrocycleTimeline Component

**Files:**
- Create: `frontend/src/components/plan/MacrocycleTimeline.tsx`

**Step 1: Build component**

> IMPORTANT: Invoke `/frontend-design` skill before writing this component.

```typescript
import { parseISO, differenceInDays } from 'date-fns';
import type { MesocycleWithWorkouts } from '../../api/types';
import { formatPhase, formatWorkoutType } from '../../utils/planHelpers';

const PHASE_COLORS: Record<string, string> = {
  capacity: 'bg-forest',
  utilization: 'bg-terra',
  taper: 'bg-amber-500',
  recovery: 'bg-slate-400',
  transition: 'bg-slate-300',
};

interface Props {
  mesocycles: MesocycleWithWorkouts[];
  macrocycleStart: string;
  macrocycleEnd: string;
}

export default function MacrocycleTimeline({ mesocycles, macrocycleStart, macrocycleEnd }: Props) {
  const totalDays = differenceInDays(parseISO(macrocycleEnd), parseISO(macrocycleStart));
  if (totalDays <= 0) return null;

  const today = new Date();
  const todayOffset = differenceInDays(today, parseISO(macrocycleStart));
  const todayPct = Math.min(Math.max((todayOffset / totalDays) * 100, 0), 100);

  return (
    <div className="rounded-xl bg-white border border-cream-dark p-4">
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-xs font-medium text-slate uppercase tracking-wider">Training Timeline</h3>
      </div>
      <div className="relative h-8 rounded-lg overflow-hidden flex">
        {mesocycles.map((m) => {
          const days = differenceInDays(parseISO(m.end_date), parseISO(m.start_date));
          const widthPct = (days / totalDays) * 100;
          const bgColor = PHASE_COLORS[m.phase] ?? 'bg-slate-300';
          return (
            <div
              key={m.id}
              className={`${bgColor} flex items-center justify-center`}
              style={{ width: `${widthPct}%` }}
              title={`${formatPhase(m.phase)} — ${formatWorkoutType(m.focus)}`}
            >
              {widthPct > 10 && (
                <span className="text-cream text-[10px] font-medium truncate px-1">
                  {formatPhase(m.phase)}
                </span>
              )}
            </div>
          );
        })}
        {/* Today marker */}
        <div
          className="absolute top-0 bottom-0 w-0.5 bg-charcoal"
          style={{ left: `${todayPct}%` }}
        />
      </div>
    </div>
  );
}
```

**Step 2: Run type check**

Run: `cd frontend && npx tsc --noEmit`

**Step 3: Commit**

```bash
git add frontend/src/components/plan/MacrocycleTimeline.tsx
git commit -m "feat: add MacrocycleTimeline component"
```

---

## Task 10: WorkoutCard Component

**Files:**
- Create: `frontend/src/components/plan/WorkoutCard.tsx`

> IMPORTANT: Invoke `/frontend-design` skill before writing this component.

**Step 1: Build compact workout card for calendar cells**

```typescript
import type { PlannedWorkout } from '../../api/types';
import { getWorkoutBorderColor } from '../../utils/workoutColors';
import { formatWorkoutType } from '../../utils/planHelpers';
import { isSameDay, parseISO, isPast, startOfDay } from 'date-fns';

interface Props {
  workout: PlannedWorkout;
  onClick: () => void;
}

export default function WorkoutCard({ workout, onClick }: Props) {
  const borderColor = getWorkoutBorderColor(workout.workout_type);
  const isToday = isSameDay(parseISO(workout.scheduled_date), new Date());
  const isPastDate = isPast(startOfDay(parseISO(workout.scheduled_date))) && !isToday;
  const isCompleted = workout.is_completed === 1;
  const isRest = workout.workout_type === 'rest';

  return (
    <button
      onClick={onClick}
      className={`w-full text-left p-1.5 rounded-lg transition-colors text-xs
        ${isToday ? 'ring-2 ring-forest bg-forest/5' : ''}
        ${isPastDate && !isCompleted && !isRest ? 'opacity-50' : ''}
        ${isRest ? 'bg-cream-dark/30' : 'hover:bg-cream/80'}
      `}
      style={{ borderLeft: isRest ? 'none' : `3px solid ${borderColor}` }}
    >
      <p className={`font-medium truncate ${isRest ? 'text-slate-light' : 'text-charcoal'}`}>
        {formatWorkoutType(workout.workout_type)}
      </p>
      {!isRest && (
        <div className="flex items-center gap-1 mt-0.5 text-slate-light">
          {workout.duration_min && <span>{workout.duration_min}m</span>}
          {workout.target_hr_zones && <span className="truncate">{workout.target_hr_zones}</span>}
        </div>
      )}
      {isCompleted && (
        <span className="text-forest font-medium">✓</span>
      )}
    </button>
  );
}
```

**Step 2: Run type check**

Run: `cd frontend && npx tsc --noEmit`

**Step 3: Commit**

```bash
git add frontend/src/components/plan/WorkoutCard.tsx
git commit -m "feat: add WorkoutCard component"
```

---

## Task 11: WeekSummary Component

**Files:**
- Create: `frontend/src/components/plan/WeekSummary.tsx`

> IMPORTANT: Invoke `/frontend-design` skill before writing this component.

**Step 1: Build week summary for the right column**

```typescript
import type { PlannedWorkout } from '../../api/types';
import { getWeekSummary, formatMinutes } from '../../utils/planHelpers';

interface Props {
  workouts: PlannedWorkout[];
  weekLabel: string;
}

export default function WeekSummary({ workouts, weekLabel }: Props) {
  const { targetKm, targetTss, targetMinutes, totalSessions, completedSessions } =
    getWeekSummary(workouts);

  return (
    <div className="text-xs space-y-1 p-2">
      <p className="font-medium text-charcoal truncate">{weekLabel}</p>
      {targetKm > 0 && (
        <p className="text-slate">{targetKm.toFixed(1)} km</p>
      )}
      {targetTss > 0 && (
        <p className="text-slate">{Math.round(targetTss)} TSS</p>
      )}
      {targetMinutes > 0 && (
        <p className="text-slate">{formatMinutes(targetMinutes)}</p>
      )}
      <p className="text-slate">{completedSessions}/{totalSessions}</p>
    </div>
  );
}
```

**Step 2: Run type check + commit**

```bash
git add frontend/src/components/plan/WeekSummary.tsx
git commit -m "feat: add WeekSummary component"
```

---

## Task 12: WorkoutDrawer Component

**Files:**
- Create: `frontend/src/components/plan/WorkoutDrawer.tsx`

> IMPORTANT: Invoke `/frontend-design` skill before writing this component.

**Step 1: Build slide-out drawer for workout details**

```typescript
import { useEffect, useRef } from 'react';
import type { PlannedWorkout, MesocycleWithWorkouts } from '../../api/types';
import { getWorkoutBorderColor } from '../../utils/workoutColors';
import { formatWorkoutType, formatPhase } from '../../utils/planHelpers';
import { format, parseISO } from 'date-fns';

interface Props {
  workout: PlannedWorkout | null;
  mesocycle: MesocycleWithWorkouts | null;
  onClose: () => void;
}

export default function WorkoutDrawer({ workout, mesocycle, onClose }: Props) {
  const drawerRef = useRef<HTMLDivElement>(null);

  // Close on Escape
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    document.addEventListener('keydown', handler);
    return () => document.removeEventListener('keydown', handler);
  }, [onClose]);

  if (!workout) return null;

  const borderColor = getWorkoutBorderColor(workout.workout_type);
  const date = parseISO(workout.scheduled_date);

  return (
    <>
      {/* Backdrop */}
      <div className="fixed inset-0 bg-charcoal/20 z-40" onClick={onClose} />

      {/* Drawer */}
      <div
        ref={drawerRef}
        className="fixed right-0 top-0 bottom-0 w-full max-w-md bg-white shadow-xl z-50 overflow-y-auto"
      >
        <div className="p-6">
          {/* Header */}
          <div className="flex items-start justify-between mb-6">
            <div>
              <p className="text-xs text-slate uppercase tracking-wider">
                {format(date, 'EEEE, MMMM d, yyyy')}
              </p>
              <h2
                className="font-serif text-2xl text-charcoal font-bold mt-1"
                style={{ borderLeft: `4px solid ${borderColor}`, paddingLeft: '12px' }}
              >
                {formatWorkoutType(workout.workout_type)}
              </h2>
              {mesocycle && (
                <p className="text-xs text-slate mt-1 pl-4">
                  {formatPhase(mesocycle.phase)} — {formatWorkoutType(mesocycle.focus)}
                </p>
              )}
            </div>
            <button
              onClick={onClose}
              className="text-slate hover:text-charcoal transition-colors p-1"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          {/* Metrics row */}
          <div className="grid grid-cols-3 gap-3 mb-6">
            {workout.duration_min && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{workout.duration_min}</p>
                <p className="text-[10px] text-slate uppercase">Minutes</p>
              </div>
            )}
            {workout.expected_tss != null && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{Math.round(workout.expected_tss)}</p>
                <p className="text-[10px] text-slate uppercase">TSS</p>
              </div>
            )}
            {workout.target_distance_km != null && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{workout.target_distance_km.toFixed(1)}</p>
                <p className="text-[10px] text-slate uppercase">km</p>
              </div>
            )}
          </div>

          {/* Target zones */}
          {(workout.target_hr_zones || workout.target_pace_zones) && (
            <div className="mb-6">
              <h3 className="text-xs font-medium text-slate uppercase tracking-wider mb-2">Target Zones</h3>
              <div className="space-y-1">
                {workout.target_hr_zones && (
                  <p className="text-sm text-charcoal">HR: {workout.target_hr_zones}</p>
                )}
                {workout.target_pace_zones && (
                  <p className="text-sm text-charcoal">Pace: {workout.target_pace_zones}</p>
                )}
              </div>
            </div>
          )}

          {/* Description */}
          {workout.description && (
            <div className="mb-6">
              <h3 className="text-xs font-medium text-slate uppercase tracking-wider mb-2">Description</h3>
              <p className="text-sm text-charcoal leading-relaxed">{workout.description}</p>
            </div>
          )}

          {/* Coach notes */}
          {workout.coach_notes && (
            <div className="rounded-xl bg-forest/5 border border-forest/10 p-4">
              <div className="flex gap-2">
                <div className="w-6 h-6 rounded-full bg-forest flex items-center justify-center flex-shrink-0">
                  <span className="text-cream text-[10px] font-bold font-serif">J</span>
                </div>
                <p className="text-sm text-charcoal leading-relaxed">
                  {workout.coach_notes}
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
```

**Step 2: Run type check + commit**

```bash
git add frontend/src/components/plan/WorkoutDrawer.tsx
git commit -m "feat: add WorkoutDrawer slide-out component"
```

---

## Task 13: Calendar Component

**Files:**
- Create: `frontend/src/components/plan/Calendar.tsx`

> IMPORTANT: Invoke `/frontend-design` skill before writing this component.

This is the largest frontend component. It renders:
- 8-column CSS grid (Mon-Sun + weekly summary)
- Week rows for all weeks in the macrocycle
- Mesocycle boundary dividers
- Navigation (Today button, month label)
- Responsive: stacked daily list on mobile

**Step 1: Build component**

```typescript
import { useRef, useCallback, useMemo, useState } from 'react';
import {
  startOfISOWeek, endOfISOWeek, addWeeks, isSameDay, parseISO,
  format, isWithinInterval, eachWeekOfInterval, isSameWeek,
} from 'date-fns';
import type { PlanResponse, PlannedWorkout, MesocycleWithWorkouts } from '../../api/types';
import WorkoutCard from './WorkoutCard';
import WeekSummary from './WeekSummary';
import WorkoutDrawer from './WorkoutDrawer';
import { getAllWorkouts, getMesocycleForDate, getWeekOfMesocycle, formatPhase, formatWorkoutType } from '../../utils/planHelpers';

interface Props {
  plan: PlanResponse;
}

export default function Calendar({ plan }: Props) {
  const todayRef = useRef<HTMLDivElement>(null);
  const [selectedWorkout, setSelectedWorkout] = useState<PlannedWorkout | null>(null);
  const [selectedMesocycle, setSelectedMesocycle] = useState<MesocycleWithWorkouts | null>(null);

  const allWorkouts = useMemo(() => getAllWorkouts(plan), [plan]);
  const macroStart = parseISO(plan.macrocycle.start_date);
  const macroEnd = parseISO(plan.macrocycle.end_date);

  // Generate all ISO weeks in the macrocycle
  const weeks = useMemo(() => {
    return eachWeekOfInterval(
      { start: macroStart, end: macroEnd },
      { weekStartsOn: 1 }
    );
  }, [macroStart, macroEnd]);

  // Group workouts by week start date
  const workoutsByWeek = useMemo(() => {
    const map = new Map<string, PlannedWorkout[]>();
    for (const w of allWorkouts) {
      const weekStart = startOfISOWeek(parseISO(w.scheduled_date));
      const key = format(weekStart, 'yyyy-MM-dd');
      if (!map.has(key)) map.set(key, []);
      map.get(key)!.push(w);
    }
    return map;
  }, [allWorkouts]);

  const scrollToToday = useCallback(() => {
    todayRef.current?.scrollIntoView({ behavior: 'smooth', block: 'center' });
  }, []);

  const handleWorkoutClick = (workout: PlannedWorkout) => {
    const meso = plan.mesocycles.find(m => m.id === workout.mesocycle_id) ?? null;
    setSelectedWorkout(workout);
    setSelectedMesocycle(meso);
  };

  const today = new Date();
  const dayHeaders = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

  // Track mesocycle boundaries
  let lastMesocycleId: number | null = null;

  return (
    <div>
      {/* Navigation */}
      <div className="flex items-center justify-between mb-4">
        <button
          onClick={scrollToToday}
          className="text-xs font-medium text-forest border border-forest/30 rounded-lg px-3 py-1.5 hover:bg-forest/5 transition-colors"
        >
          Today
        </button>
      </div>

      {/* Desktop calendar grid */}
      <div className="hidden md:block">
        {/* Day headers */}
        <div className="grid grid-cols-[repeat(7,1fr)_120px] gap-px mb-1">
          {dayHeaders.map(d => (
            <div key={d} className="text-center text-[10px] font-medium text-slate uppercase tracking-wider py-1">
              {d}
            </div>
          ))}
          <div className="text-center text-[10px] font-medium text-slate uppercase tracking-wider py-1">
            Summary
          </div>
        </div>

        {/* Week rows */}
        {weeks.map((weekStart) => {
          const weekKey = format(weekStart, 'yyyy-MM-dd');
          const weekWorkouts = workoutsByWeek.get(weekKey) ?? [];
          const isThisWeek = isSameWeek(weekStart, today, { weekStartsOn: 1 });
          const currentMeso = getMesocycleForDate(plan, weekStart);
          const mesoId = currentMeso?.id ?? null;

          // Check for mesocycle boundary
          const showBoundary = lastMesocycleId !== null && mesoId !== lastMesocycleId && currentMeso;
          lastMesocycleId = mesoId;

          const weekLabel = currentMeso
            ? `Wk ${getWeekOfMesocycle(currentMeso, weekStart)}`
            : format(weekStart, 'MMM d');

          return (
            <div key={weekKey}>
              {/* Mesocycle boundary divider */}
              {showBoundary && currentMeso && (
                <div className="flex items-center gap-2 py-2">
                  <div className="flex-1 h-px bg-terra/30" />
                  <span className="text-[10px] font-medium text-terra">
                    {formatPhase(currentMeso.phase)} — {formatWorkoutType(currentMeso.focus)}
                  </span>
                  <div className="flex-1 h-px bg-terra/30" />
                </div>
              )}

              <div
                ref={isThisWeek ? todayRef : undefined}
                className={`grid grid-cols-[repeat(7,1fr)_120px] gap-px ${
                  isThisWeek ? 'bg-forest/5 rounded-lg' : ''
                }`}
              >
                {/* 7 day cells */}
                {Array.from({ length: 7 }, (_, i) => {
                  const day = addWeeks(weekStart, 0);
                  const cellDate = new Date(day);
                  cellDate.setDate(cellDate.getDate() + i);
                  const cellDateStr = format(cellDate, 'yyyy-MM-dd');
                  const workout = weekWorkouts.find(w => w.scheduled_date === cellDateStr);
                  const isToday = isSameDay(cellDate, today);

                  return (
                    <div
                      key={i}
                      className={`min-h-[80px] p-1 border-b border-cream-dark/50 ${
                        isToday ? 'bg-forest/5' : ''
                      }`}
                    >
                      <p className={`text-[10px] mb-0.5 ${isToday ? 'font-bold text-forest' : 'text-slate-light'}`}>
                        {format(cellDate, 'd')}
                      </p>
                      {workout && (
                        <WorkoutCard
                          workout={workout}
                          onClick={() => handleWorkoutClick(workout)}
                        />
                      )}
                    </div>
                  );
                })}

                {/* Weekly summary column */}
                <div className="border-b border-cream-dark/50 border-l border-cream-dark/30">
                  <WeekSummary workouts={weekWorkouts} weekLabel={weekLabel} />
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Mobile stacked list */}
      <div className="md:hidden space-y-6">
        {weeks.map((weekStart) => {
          const weekKey = format(weekStart, 'yyyy-MM-dd');
          const weekWorkouts = workoutsByWeek.get(weekKey) ?? [];
          if (weekWorkouts.length === 0) return null;
          const currentMeso = getMesocycleForDate(plan, weekStart);
          const weekLabel = currentMeso
            ? `Week ${getWeekOfMesocycle(currentMeso, weekStart)}`
            : format(weekStart, 'MMM d');

          return (
            <div key={weekKey}>
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-sm font-medium text-charcoal">
                  {weekLabel} · {format(weekStart, 'MMM d')} – {format(endOfISOWeek(weekStart), 'MMM d')}
                </h3>
              </div>
              <WeekSummary workouts={weekWorkouts} weekLabel={weekLabel} />
              <div className="space-y-1 mt-2">
                {weekWorkouts
                  .sort((a, b) => a.scheduled_date.localeCompare(b.scheduled_date))
                  .map(w => (
                    <div key={w.id} className="flex items-center gap-2">
                      <span className="text-[10px] text-slate-light w-8">
                        {format(parseISO(w.scheduled_date), 'EEE')}
                      </span>
                      <div className="flex-1">
                        <WorkoutCard workout={w} onClick={() => handleWorkoutClick(w)} />
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Workout detail drawer */}
      <WorkoutDrawer
        workout={selectedWorkout}
        mesocycle={selectedMesocycle}
        onClose={() => setSelectedWorkout(null)}
      />
    </div>
  );
}
```

**Step 2: Run type check**

Run: `cd frontend && npx tsc --noEmit`

**Step 3: Commit**

```bash
git add frontend/src/components/plan/Calendar.tsx
git commit -m "feat: add Calendar component with responsive layout"
```

---

## Task 14: Plan.tsx Page

**Files:**
- Create: `frontend/src/pages/Plan.tsx`
- Modify: `frontend/src/App.tsx` — add route (~line 90)
- Modify: `frontend/src/components/layout/Nav.tsx` — enable Plan link (~line 13)

**Step 1: Build Plan page**

> IMPORTANT: Invoke `/frontend-design` skill before writing this page.

```typescript
import { useCurrentPlan } from '../hooks/usePlan';
import MacrocycleTimeline from '../components/plan/MacrocycleTimeline';
import Calendar from '../components/plan/Calendar';

export default function Plan() {
  const { data: plan, isLoading, isError } = useCurrentPlan();

  if (isLoading) {
    return (
      <div className="space-y-6">
        <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>
        <div className="rounded-xl bg-white border border-cream-dark p-8 text-center">
          <div className="w-8 h-8 border-2 border-forest border-t-transparent rounded-full animate-spin mx-auto" />
          <p className="text-slate text-sm mt-3">Loading your plan...</p>
        </div>
      </div>
    );
  }

  if (isError || !plan) {
    return (
      <div className="space-y-6">
        <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>
        <div className="rounded-xl bg-white border border-cream-dark p-8 text-center">
          <p className="text-slate">No training plan found. Complete onboarding to generate your plan.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>

      <MacrocycleTimeline
        mesocycles={plan.mesocycles}
        macrocycleStart={plan.macrocycle.start_date}
        macrocycleEnd={plan.macrocycle.end_date}
      />

      <Calendar plan={plan} />
    </div>
  );
}
```

**Step 2: Add route in App.tsx**

In `App.tsx`, add import and route:
```typescript
import Plan from './pages/Plan';
```
And inside the `<Route element={<AuthGuard requireProfile><Shell /></AuthGuard>}>`:
```tsx
<Route path="/plan" element={<Plan />} />
```

**Step 3: Enable Plan link in Nav.tsx**

In `Nav.tsx`, change the Plan nav item from `disabled: true` to remove the `disabled` property:
```typescript
{ label: 'Plan', path: '/plan', icon: '\u25B6' },
```

**Step 4: Run type check**

Run: `cd frontend && npx tsc --noEmit`

**Step 5: Commit**

```bash
git add frontend/src/pages/Plan.tsx frontend/src/App.tsx frontend/src/components/layout/Nav.tsx
git commit -m "feat: add Plan page with calendar view, enable Plan nav link"
```

---

## Task 15: Dashboard Enhancements

**Files:**
- Modify: `frontend/src/pages/Dashboard.tsx` — replace TrainingPlanCard, add today's workout hero, phase banner, weekly progress, CTL/ATL/TSB placeholder

> IMPORTANT: Invoke `/frontend-design` skill before modifying the dashboard.

**Step 1: Rewrite Dashboard to use new PlanResponse shape**

Key changes:
1. Replace `planData.workouts` references (no longer a flat array) with helper functions
2. Replace `TrainingPlanCard` with `TodaysWorkoutCard` (hero element)
3. Add `PhaseBanner` component
4. Add `WeeklyProgressBar` component
5. Add `MetricsPlaceholder` component (CTL/ATL/TSB)
6. Remove `WorkoutRow` component (no longer needed)
7. Import helpers from `../utils/planHelpers`

The Dashboard should show:
- Welcome header (keep)
- Race countdown (keep)
- Phase banner (new)
- Today's workout card — hero (new, replaces TrainingPlanCard)
- Weekly progress bar (new)
- CTL/ATL/TSB placeholder row (new)
- Quick links (keep, update Plan link to be enabled)
- Coach note (keep)

**Step 2: Implement each sub-component inline in Dashboard.tsx**

`TodaysWorkoutCard`: Large card showing today's workout with type, duration, zones, description, coach notes. If rest day: shows rest message. If no workout today: shows next upcoming.

`PhaseBanner`: "Mesocycle 1 — Aerobic Capacity Building, Week 2 of 3"

`WeeklyProgressBar`: Horizontal bar + mini metrics for current week (km, TSS, sessions).

`MetricsPlaceholder`: Three cards with 0.0 values and "Phase 5" labels for CTL/ATL/TSB.

**Step 3: Run type check**

Run: `cd frontend && npx tsc --noEmit`
Expected: PASS

**Step 4: Commit**

```bash
git add frontend/src/pages/Dashboard.tsx
git commit -m "feat: enhance Dashboard with today's workout hero, phase banner, weekly progress"
```

---

## Task 16: Backend Tests for New Endpoints

**Files:**
- Modify: `tests/integration_tests.rs` — add tests for updated `GET /api/plan` and new `GET /api/plan/workout/:id`

**Step 1: Write integration tests**

Test `GET /api/plan` returns mesocycles with nested workouts array.
Test `GET /api/plan/workout/:id` returns workout with mesocycle context.
Test `GET /api/plan/workout/:id` returns 404 for non-existent workout.
Test `GET /api/plan/workout/:id` returns 404 for another user's workout (ownership check).

**Step 2: Run tests**

Run: `cargo test`
Expected: All PASS

**Step 3: Commit**

```bash
git add tests/integration_tests.rs
git commit -m "test: add integration tests for Phase 3 plan endpoints"
```

---

## Task 17: Final Verification

**Step 1: Run all backend tests**

Run: `cargo test`
Expected: All PASS (142+ unit + 31+ integration)

**Step 2: Run frontend type check**

Run: `cd frontend && npx tsc --noEmit`
Expected: No errors

**Step 3: Manual smoke test**

Run: `cargo run` in one terminal, `cd frontend && npm run dev` in another.
- Navigate to Dashboard — verify today's workout card, phase banner, weekly progress
- Click "View full plan" → Plan page loads with macrocycle timeline and calendar
- Click a workout in the calendar → drawer slides out with details
- Verify responsive: resize to mobile width, see stacked daily list
- Verify Plan nav link is enabled and active

**Step 4: Commit any final fixes**

---

## Task Summary

| # | Task | Type | Estimated Steps |
|---|------|------|----------------|
| 1 | DB Migration (target_distance_km) | Backend | 3 |
| 2 | Update Rust structs | Backend | 6 |
| 3 | get_plan_with_all_workouts query | Backend | 5 |
| 4 | get_workout_with_context query | Backend | 5 |
| 5 | Update API handlers + router | Backend | 6 |
| 6 | Update AI tool schema | Backend | 5 |
| 7 | Frontend types + API + hooks | Frontend | 5 |
| 8 | Extract colors + plan helpers | Frontend | 5 |
| 9 | MacrocycleTimeline component | Frontend | 3 |
| 10 | WorkoutCard component | Frontend | 3 |
| 11 | WeekSummary component | Frontend | 3 |
| 12 | WorkoutDrawer component | Frontend | 3 |
| 13 | Calendar component | Frontend | 3 |
| 14 | Plan.tsx page + routing | Frontend | 5 |
| 15 | Dashboard enhancements | Frontend | 4 |
| 16 | Integration tests | Backend | 3 |
| 17 | Final verification | QA | 4 |
