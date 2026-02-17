# Phase 3: Training Plan Viewing — Design

**Date**: 2026-02-16
**Status**: Approved

## Goal

Rich calendar view of the training plan. Dashboard with today's workout and current training context. The athlete opens the app and knows exactly what to do today.

---

## Backend

### Modified: `GET /api/plan`

Returns the active macrocycle with ALL mesocycles, each including their workouts array.

```json
{
  "macrocycle": { "id": 1, "start_date": "...", "end_date": "...", ... },
  "mesocycles": [
    {
      "id": 1, "sequence_number": 1, "phase": "capacity", "focus": "aerobic_capacity",
      "start_date": "...", "end_date": "...", "load_weeks": 3, "recovery_weeks": 1,
      "target_volume_km": 160.0, "status": "active",
      "workouts": [
        { "id": 1, "scheduled_date": "2026-03-03", "workout_type": "easy_run", "duration_min": 45, "target_distance_km": 8.0, ... }
      ]
    },
    {
      "id": 2, "sequence_number": 2, "phase": "utilization", ...
      "workouts": []
    }
  ]
}
```

Key decisions:
- Workouts grouped by mesocycle (not flat list) — calendar needs mesocycle boundaries
- No `is_today`/`is_past` computed fields — client derives from `date-fns`
- No query parameters — entire plan is ~200 rows, trivially small
- Future mesocycles with no generated workouts return `"workouts": []`

### New: `GET /api/plan/workout/:id`

Single workout detail with minimal mesocycle context. For deep-linking.

```json
{
  "workout": { "id": 1, "scheduled_date": "...", ... },
  "mesocycle": { "id": 1, "phase": "capacity", "focus": "aerobic_capacity", "sequence_number": 1 }
}
```

### Removed: `GET /api/plan/week?date=`

Dropped per API review. Redundant when all workouts are fetched via `GET /api/plan`. Client-side filtering is a one-liner with `date-fns`.

### New DB migration: `target_distance_km`

Add `target_distance_km REAL` column to `planned_workouts` table. Enables weekly mileage summaries in the calendar. Update plan generation tool schema to include distance targets.

### New DB query: `get_all_plan_workouts`

Fetch all planned_workouts across all mesocycles for the active macrocycle via JOIN on mesocycles → macrocycles.

---

## Frontend — Plan Page

### Layout (desktop, ≥ 768px)

```
┌──────────────────────────────────────────────────────────┐
│  Macrocycle Timeline Bar (colored by phase)               │
├──────────────────────────────────────────────────────────┤
│  [Today] button            "March 2026"   [◀ Prev][Next ▶]│
├───────┬───────┬───────┬───────┬───────┬───────┬─────────┤
│  Mon  │  Tue  │  Wed  │  Thu  │  Fri  │  Sat  │  Sun    │ Weekly
├───────┼───────┼───────┼───────┼───────┼───────┼─────────┤ Summary
│ Easy  │ Tempo │ Rest  │ VO2   │ Easy  │ Long  │ Rest    │ 52km
│ 45min │ 50min │       │ 55min │ 40min │ 90min │         │ 280 TSS
│  Z1-2 │  Z3-4 │       │  Z5   │  Z1-2 │  Z1-3 │         │ 4:35 hrs
├───────┴───────┴───────┴───────┴───────┴───────┴─────────┤
│ ── Mesocycle boundary divider with phase label ──────────│
├───────┬───────┬───────┬───────┬───────┬───────┬─────────┤
│  ...next week...                                          │
└──────────────────────────────────────────────────────────┘
```

- CSS grid: 8 columns (7 days + weekly summary)
- Default: monthly view, continuous scroll showing all weeks
- Today's date: highlighted border
- Mesocycle boundaries: labeled dividers between week rows
- Clicking a workout card: opens slide-out drawer with full details
- "Today" button: scrolls to current week

### Layout (mobile, < 768px)

Stacked daily list grouped by week. Week summary as header card above each week group.

### Components

| Component | File | Purpose |
|-----------|------|---------|
| Calendar | `components/plan/Calendar.tsx` | Grid layout, week rows, navigation, responsive switch |
| WorkoutCard | `components/plan/WorkoutCard.tsx` | Compact card in cell + expanded content in drawer |
| WeekSummary | `components/plan/WeekSummary.tsx` | Right column (desktop) / header card (mobile) |
| MacrocycleTimeline | `components/plan/MacrocycleTimeline.tsx` | Colored phase bar at top |
| WorkoutDrawer | `components/plan/WorkoutDrawer.tsx` | Slide-out detail panel |

### Plan.tsx page

- Top: MacrocycleTimeline bar
- Navigation: Today button, month label, prev/next
- Body: Calendar grid with WeekSummary column
- Drawer: WorkoutDrawer (opened on workout click)

---

## Frontend — Dashboard Enhancements

### Today's Workout Card (hero element)

Large, prominent card replacing the current flat workout list:
- Workout today → type, duration, target zones, description, coach notes
- Rest day → "Rest Day — Recovery is training too"
- No workout → next upcoming workout with "Next up: Tomorrow" label

### Phase Banner

Below race countdown: "Mesocycle 1 — Aerobic Capacity Building, Week 2 of 3"

### Weekly Progress Bar

Horizontal bar: completed vs target for current week.
Three mini metrics: km, TSS, sessions (e.g., "28/42 km · 180/250 TSS · 4/6 sessions")

### CTL/ATL/TSB Placeholder Row

Three metric cards with 0.0 values and "Coming in Phase 5" label.

### Removed from Dashboard

The TrainingPlanCard with full workout list is replaced by the today's workout hero + "View full plan →" link.

---

## Data Flow

### Single data source

`useCurrentPlan()` hook fetches `GET /api/plan` once, cached by TanStack Query. Both Dashboard and Plan page use this hook.

### Updated types

```typescript
interface PlanResponse {
  macrocycle: Macrocycle;
  mesocycles: MesocycleWithWorkouts[];
}

interface MesocycleWithWorkouts extends Mesocycle {
  workouts: PlannedWorkout[];
}
```

`PlannedWorkout` gains `target_distance_km: number | null`.

### Client-side utilities (no new hooks)

- `getTodaysWorkout(plan)` — workout where scheduled_date === today
- `getWeekWorkouts(plan, date)` — filter workouts within ISO week
- `getWeekSummary(workouts)` — sum distance, TSS, duration
- `getCurrentMesocycle(plan)` — mesocycle where today is between start/end dates
- `getWeekOfMesocycle(mesocycle, date)` — week number within mesocycle

### Drawer data

Populated from cached plan data. No extra API call. `GET /api/plan/workout/:id` exists for deep-linking only.

### Nav update

Enable "Plan" link in Nav.tsx (remove `disabled: true`).

---

## Workout color coding

Reuse existing `WORKOUT_COLORS` map from Dashboard.tsx. Extract to shared utility `frontend/src/utils/workoutColors.ts`.

---

## Testing

- **Backend**: Unit tests for new DB queries. Integration test for modified `GET /api/plan` response shape. Integration test for `GET /api/plan/workout/:id`.
- **Frontend**: Component tests for Calendar rendering correct days/weeks. WorkoutCard compact and expanded states. WeekSummary calculations.
- **E2E (Playwright)**: Navigate to Plan page, see calendar with workouts. Click workout, see drawer. Navigate between weeks. Verify mobile stacked layout. Dashboard shows today's workout.
