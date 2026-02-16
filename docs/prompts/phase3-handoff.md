# Phase 3: Training Plan Viewing — Handoff Prompt

> Copy-paste this as your first message in a new Claude Code session.

---

## Prompt

I'm building CoachJan, an AI running coach. Phases 0-2 are complete (auth, profiles, plan generation). Start Phase 3: Training Plan Viewing.

### What Phase 3 builds
Rich calendar view of the training plan. Dashboard with today's workout and current training context. The athlete opens the app and knows exactly what to do today.

### Phase 3 spec (from docs/IMPLEMENTATION_PLAN.md lines 262-330)

**Frontend tasks:**
1. **Calendar component** (`frontend/src/components/plan/Calendar.tsx`) — CSS grid 7 cols Mon-Sun, rows = weeks. Each cell shows WorkoutCard. Navigation: prev/next week, "Today" button. Mesocycle boundaries as horizontal dividers. Mobile: stacked day view.
2. **WorkoutCard** (`frontend/src/components/plan/WorkoutCard.tsx`) — Compact mode: color-coded icon, duration, zone. Expanded mode (on click): full description, target HR/pace zones, expected TSS, coach notes, and for completed workouts: actual metrics alongside targets.
3. **WeekSummary** (`frontend/src/components/plan/WeekSummary.tsx`) — "Week 2 of Mesocycle 1 — Aerobic Capacity". Target volume vs completed, target TSS vs completed, sessions completed count.
4. **Plan Overview page** (`frontend/src/pages/Plan.tsx`) — Macrocycle timeline bar colored by phase at top, calendar below. Week/month view toggle.
5. **Dashboard updates** (`frontend/src/pages/Dashboard.tsx`) — Today's workout card (large, prominent). Phase banner. Weekly progress bar. CTL/ATL/TSB placeholder row.

**Backend tasks:**
6. **Additional plan endpoints** in `src/api/plans.rs`:
   - `GET /api/plan/week?date=YYYY-MM-DD` → workouts for the week containing that date
   - `GET /api/plan/workout/:id` → single workout with full details
   - Include computed fields: `is_today`, `is_past`, `is_completed`

**Tests:**
- Frontend component tests for calendar rendering
- Responsive tests for mobile layout
- Playwright E2E: navigate calendar, view workout details

### Key existing code to read first
- `docs/IMPLEMENTATION_PLAN.md` (lines 262-330 for Phase 3)
- `docs/FEATURES.md` (search for "F4: Training Plan Viewing")
- `docs/ARCHITECTURE.md` (frontend structure, API endpoints)
- `src/api/plans.rs` — existing plan endpoints (generate, confirm, get)
- `src/db/plans.rs` — existing plan CRUD (macrocycle, mesocycle, planned_workouts)
- `frontend/src/pages/Dashboard.tsx` — current dashboard (already has race countdown, workout list, coach message)
- `frontend/src/hooks/usePlan.ts` — existing plan hooks
- `frontend/src/api/types.ts` — existing types (Macrocycle, Mesocycle, PlannedWorkout)
- `frontend/src/api/plan.ts` — existing API client

### Tech stack
- **Backend**: Rust/Axum, SQLite/sqlx, edition 2024
- **Frontend**: React 19, TypeScript, Vite 7, Tailwind CSS v4, TanStack Query, React Router, Recharts, date-fns, Zustand
- **Testing**: cargo test (142 unit + 31 integration), Playwright for E2E
- **Design**: forest/terra/cream palette, font-serif headings, rounded-xl cards. Always invoke `/frontend-design` skill before building UI.

### Project structure
```
src/                          # Rust backend
  ai/                         # Claude API client, prompts, tools, context, handlers
  api/                        # Axum route handlers (auth, athletes, plans)
  auth/                       # Password hashing
  db/                         # SQLite CRUD (users, sessions, profiles, plans)
  domain/                     # Business logic (types, zones, bootstrap, workouts, validation)
  lib.rs                      # AppState + build_app() router
  main.rs                     # Entry point
frontend/src/                 # React frontend
  api/                        # API client functions + types
  hooks/                      # TanStack Query hooks
  pages/                      # Dashboard, Login, Onboarding, Profile
  components/layout/          # Shell, Nav
  components/shared/          # ZoneTable
migrations/                   # SQLite migrations (001-003)
tests/                        # integration_tests.rs, live_claude_test.rs
docs/                         # Architecture, Features, Implementation Plan
docs/phases/                  # Phase completion docs (0-2)
```

### Database tables (already exist)
- `macrocycles` — id, user_id, status, target_ctl, coach_message, created_at
- `mesocycles` — id, macrocycle_id, sequence_number, phase, focus, load_weeks, recovery_weeks, target_volume_km, start_date, end_date
- `planned_workouts` — id, mesocycle_id, user_id, scheduled_date, workout_type, duration_min, duration_category, target_hr_zones, target_pace_zones, expected_tss, description, coach_notes, is_completed, completed_workout_id

### Workflow
Use the brainstorming skill to design, then writing-plans skill to create the implementation plan, then subagent-driven-development to execute. Follow TDD. Invoke `/frontend-design` before any UI work. Commit after each task.

### Gotchas from previous phases
- Rust edition 2024: `env::remove_var` needs `unsafe {}` block
- Tailwind v4: `@import "tailwindcss"` not `@tailwind`, no tailwind.config.ts
- `coach_jan_system_prompt()` is a function not a const
- `build_macrocycle_context()` takes 4 args (profile, race_goal, ctl, weeks_until_race)
- Foreign keys: always `SqliteConnectOptions::pragma("foreign_keys", "ON")`
- Wiremock 0.6 for integration tests, ClaudeClient has `new_with_base_url()`

### What's already done in Dashboard.tsx
The dashboard already has:
- Race countdown (RaceCountdown component) with real data
- Training plan card showing current mesocycle workouts (TrainingPlanCard + WorkoutRow)
- Color-coded workout borders (WORKOUT_COLORS map with 32 types)
- Coach message (CoachNote component)
- Quick links section

Phase 3 enhances this with: today's workout (large, prominent), phase banner, weekly progress bar, CTL/ATL/TSB placeholder row.
