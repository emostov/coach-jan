# Phase 0: Project Setup

## Goal

Set up the project skeleton so the app starts, serves a page, connects to SQLite, and runs migrations. No feature logic — just infrastructure.

## Completion Requirements

- [ ] Rust project initializes with `cargo init --name coachjan`
- [ ] All dependencies in `Cargo.toml` (see Architecture doc)
- [ ] `src/main.rs`: Axum app starts on port 3000 (configurable via `PORT` env var)
- [ ] `src/config.rs`: Loads `DATABASE_URL`, `HOST`, `PORT`, `ANTHROPIC_API_KEY` from env
- [ ] `src/error.rs`: `AppError` enum maps to HTTP status codes (404, 400, 401, 409, 500)
- [ ] SQLite connection pool via sqlx with `PRAGMA foreign_keys = ON`
- [ ] `migrations/001_create_users.sql`: `users` + `sessions` tables
- [ ] Migrations run automatically on startup
- [ ] `GET /` returns "Hello from CoachJan"
- [ ] Tracing/logging configured with env-filter support
- [ ] Frontend: Vite + React 19 + TypeScript initialized
- [ ] Frontend dependencies installed (react-router, TanStack Query, Tailwind CSS 4, Recharts, Zustand, react-hook-form, zod)
- [ ] Tailwind CSS 4 configured (`@import "tailwindcss"` + `@tailwindcss/vite` plugin)
- [ ] Vite proxy: `/api` → `localhost:3000`
- [ ] `.env.example` with all required env vars
- [ ] `data/fit_files/` directory structure created
- [ ] `.gitignore` covers .env, database, node_modules, target, dist, fit_files
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `npm run build` passes
- [ ] Tests: config loading, error type mapping

## Files Created

### Backend
| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust dependencies |
| `src/main.rs` | Axum app entry point, DB setup, routing |
| `src/config.rs` | Environment config loading |
| `src/error.rs` | Shared error type → HTTP response mapping |
| `migrations/001_create_users.sql` | Initial DB schema (users + sessions) |

### Frontend
| File | Purpose |
|------|---------|
| `frontend/package.json` | Node dependencies |
| `frontend/vite.config.ts` | Vite config with Tailwind + API proxy |
| `frontend/src/index.css` | Tailwind CSS import |
| `frontend/src/App.tsx` | Hello page with Tailwind styling |
| `frontend/src/main.tsx` | React entry point |

### Config
| File | Purpose |
|------|---------|
| `.env.example` | Template for environment variables |
| `.gitignore` | Git ignore rules |
| `data/fit_files/.gitkeep` | FIT file storage directory |

## How to Build & Run

### Backend
```bash
# Build
cargo build

# Run (creates SQLite DB automatically)
DATABASE_URL="sqlite:data/coachjan.db?mode=rwc" cargo run

# Test
cargo test

# Lint
cargo clippy
```

### Frontend
```bash
# Install deps
npm install --prefix frontend

# Dev server (proxies /api to backend)
npm run dev --prefix frontend

# Production build
npm run build --prefix frontend
```

### Environment Variables
```
DATABASE_URL=sqlite:data/coachjan.db?mode=rwc
HOST=0.0.0.0
PORT=3000
ANTHROPIC_API_KEY=sk-ant-...  # Not needed until Phase 2
RUST_LOG=coachjan=debug,tower_http=debug
```

## Database Schema (Phase 0)

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE sessions (
    id TEXT PRIMARY KEY,     -- UUID v4
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_sessions_expires ON sessions(expires_at);
```

## Key Architecture Decisions

1. **SQLite with sqlx**: Single-file database, `create_if_missing(true)`, compile-time query checking
2. **Foreign keys pragma**: Must be set on every connection (`SqliteConnectOptions::pragma("foreign_keys", "ON")`)
3. **Tracing**: Using `tracing` + `tracing-subscriber` with env-filter for log level control
4. **Error mapping**: `AppError` implements `IntoResponse` to produce JSON error bodies with appropriate HTTP status codes
5. **Static frontend serving**: In production, Axum will serve the built React SPA. In dev, Vite dev server proxies API calls to the backend.

## Tests

### Rust Tests
- `config::tests::test_default_config` — verifies default values
- `config::tests::test_listen_addr` — verifies address formatting
- `error::tests::test_error_responses` — verifies HTTP status code mapping for each error variant

### Verification
```bash
# Full verification
cargo test && cargo clippy && npm run build --prefix frontend
```

## Next Phase

Phase 1: Authentication & Athlete Profile — see `docs/phases/PHASE_1_AUTH_PROFILE.md` (to be generated).
