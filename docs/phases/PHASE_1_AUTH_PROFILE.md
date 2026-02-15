# Phase 1: Authentication & Athlete Profile

## Goal
Athlete can register with email + password, log in, fill out their profile, and see calculated HR and pace zones.

## Usable State After This Phase
An athlete can create an account, set up their profile, see their training zones, and update their information. This is the foundation everything else builds on.

---

## Completion Requirements

### Backend
- [x] Auth system: register, login, logout, me endpoints working
- [x] Password hashing with Argon2id
- [x] Session-based auth with HTTP-only cookies (30-day expiry)
- [x] Auth middleware (Axum extractor) protecting routes
- [x] Athlete profile CRUD (create, read, update)
- [x] Zone calculation from LTHR (7-zone HR) and FTPace (6-zone pace)
- [x] CTL/ATL bootstrap from weekly volume + experience level
- [x] FTPace and LTHR history recording
- [x] Race goal storage
- [x] Initial daily_metrics entry on profile creation

### Frontend
- [x] Login/Register page with form validation
- [x] Multi-step onboarding flow (5 steps)
- [x] Profile page with editable fields and zone display
- [x] App shell with routing and auth guard
- [x] API client with auth error handling (401 → redirect)
- [x] TanStack Query hooks for all API calls

### Tests
- [x] Unit tests: zone calculation, CTL bootstrap, password hashing
- [x] Integration tests: auth flow (register → me → logout → login → me)
- [x] Integration tests: profile CRUD
- [x] Test: duplicate email returns 409
- [x] Test: wrong password returns 401
- [x] Playwright E2E: register → onboarding → see zones → profile → logout → login

---

## Files to Create/Modify

### Migrations
- `migrations/002_create_athlete_profiles.sql` — athlete_profiles, race_goals, ftpace_history, lthr_history, daily_metrics

### Backend Files
| File | Purpose |
|------|---------|
| `src/auth/mod.rs` | Auth module root |
| `src/auth/password.rs` | Argon2id hash + verify |
| `src/db/mod.rs` | DB module root |
| `src/db/users.rs` | User CRUD |
| `src/db/sessions.rs` | Session CRUD |
| `src/db/profiles.rs` | Profile + race goal + history CRUD |
| `src/domain/mod.rs` | Domain module root |
| `src/domain/types.rs` | ExperienceLevel, HrZones, PaceZones |
| `src/domain/zones.rs` | calculate_hr_zones, calculate_pace_zones |
| `src/domain/bootstrap.rs` | bootstrap_ctl |
| `src/api/mod.rs` | API module root |
| `src/api/auth.rs` | POST register/login/logout, GET me |
| `src/api/middleware.rs` | Auth extractor |
| `src/api/athletes.rs` | POST/GET/PUT profile |
| `src/main.rs` | Wire up routing |

### Frontend Files
| File | Purpose |
|------|---------|
| `frontend/src/api/client.ts` | Fetch wrapper with 401 handling |
| `frontend/src/api/types.ts` | Shared API types |
| `frontend/src/api/auth.ts` | Auth API calls |
| `frontend/src/api/athlete.ts` | Athlete API calls |
| `frontend/src/hooks/useAuth.ts` | Auth hook (TanStack Query) |
| `frontend/src/hooks/useAthlete.ts` | Athlete hook |
| `frontend/src/pages/Login.tsx` | Login/Register |
| `frontend/src/pages/Onboarding.tsx` | Multi-step profile setup |
| `frontend/src/pages/Profile.tsx` | Profile + zones view |
| `frontend/src/pages/Dashboard.tsx` | Placeholder dashboard |
| `frontend/src/components/layout/Shell.tsx` | App shell |
| `frontend/src/components/layout/Nav.tsx` | Navigation |
| `frontend/src/components/shared/ZoneTable.tsx` | Zone display table |
| `frontend/src/App.tsx` | Router setup + auth guard |

### E2E Tests
| File | Purpose |
|------|---------|
| `e2e/auth.spec.ts` | Register, login, logout flows |
| `e2e/onboarding.spec.ts` | Profile setup + zone verification |

---

## API Endpoints

| Method | Path | Auth | Body | Response |
|--------|------|------|------|----------|
| POST | `/api/auth/register` | No | `{ email, password }` | `{ user }` + session cookie |
| POST | `/api/auth/login` | No | `{ email, password }` | `{ user }` + session cookie |
| POST | `/api/auth/logout` | Yes | — | `{ message }` |
| GET | `/api/auth/me` | Yes | — | `{ user, has_profile }` |
| POST | `/api/athlete/profile` | Yes | profile + goal | `{ profile, zones }` |
| GET | `/api/athlete/profile` | Yes | — | `{ profile, zones }` |
| PUT | `/api/athlete/profile` | Yes | partial profile | `{ profile, zones }` |

---

## Zone Calculation Reference

### HR Zones (7-zone from LTHR)
| Zone | Name | Min | Max |
|------|------|-----|-----|
| 1 | Recovery | — | 81% LTHR |
| 2 | Aerobic Base | 82% | 88% |
| 3 | Tempo | 89% | 93% |
| 4 | SubThreshold | 94% | 99% |
| 5 | Threshold | 100% | 104% |
| 6 | VO2max | 105% | 110% |
| 7 | Anaerobic | 111% | — |

### Pace Zones (6-zone from FTPace m/s)
| Zone | Name | Min | Max |
|------|------|-----|-----|
| 1 | Recovery | — | 74% FTPace |
| 2 | Easy | 75% | 85% |
| 3 | Tempo | 86% | 95% |
| 4 | Threshold | 96% | 105% |
| 5 | VO2max | 106% | 120% |
| 6 | Sprint | 121% | — |

### CTL Bootstrap
```
pace_factor = beginner:0.65, intermediate:0.75, advanced:0.85
weekly_tss = weekly_km * pace_factor * 5
daily_tss = weekly_tss / 7
CTL = ATL = daily_tss
```

---

## Onboarding Steps
1. **Basic Info**: name, age, weight
2. **Physiology**: resting HR, max HR, LTHR
3. **Running**: FTPace (or race result), weekly volume, experience level
4. **Goal**: race distance, date, name
5. **Review**: show calculated zones, confirm and submit
