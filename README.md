# CoachJan

AI running coach built on Jan Olbrecht's training philosophy.

## Quick Start

### Prerequisites
- Rust (latest stable)
- Node.js 20+
- SQLite3

### Backend
```bash
cp .env.example .env
cargo run
# Server starts at http://localhost:3000
```

### Frontend
```bash
npm install --prefix frontend
npm run dev --prefix frontend
# Dev server starts at http://localhost:5173, proxies /api to backend
```

### Testing
```bash
cargo test          # Rust unit + integration tests
cargo clippy        # Lint
npm run build --prefix frontend  # Frontend build check
```

## Project Structure

See `docs/ARCHITECTURE.md` for full details.

```
src/main.rs          # Axum server entry point
src/config.rs        # Environment config
src/error.rs         # Error types → HTTP responses
migrations/          # SQLite migrations (run on startup)
frontend/            # React + TypeScript SPA
```
