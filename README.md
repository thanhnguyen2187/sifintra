# Sifintra

Simple Finance Tracker.

Technical stack:

- Moon: task runner and monorepo management
- Rust + Axum: backend
- SvelteKit + DaisyUI: frontend

## Development

### Backend

- Make sure that Rust is available (`cargo`, `rustc`, etc.)

- Make sure that and `diesel` is available:

```shell
cargo install diesel_cli \
    --no-default-features \
    --features sqlite-bundled

export PATH=$PATH:/home/thanh/.cargo/bin

diesel --version
# diesel
#  Version: 2.2.12
#  Supported Backends: sqlite
```

- Make sure that `packages/backend/.env` is ready with `DATABASE_URL`
  points to a valid SQLite3 database file:

```shell
DATABASE_URL=sifintra.db
```

- Run migrations:

```shell
diesel setup
```

- Start the server:

```shell
moon backend:serve
```

Equivalent to moving into `packages/backend`, then

```shell
cargo run
# 2025-08-17T11:59:19.105350Z  INFO backend: Listening on http://127.0.0.1:3000
```

### Frontend

- Make sure that `pnpm` is available:

```shell
pnpm --version
# 10.12.1
```

- Install dependencies:

```shell
moon 
```

- Make sure that `packages/app/.env` is ready with `PUBLIC_API_BASE_URL` points
  to the backend development endpoint:

```
PUBLIC_API_BASE_URL=http://localhost:3000
```

- Start development server:

```shell
pnpm run dev
#  VITE v7.1.1  ready in 1065 ms
#
#  ➜  Local:   http://localhost:5173/
#  ➜  Network: use --host to expose
#  ➜  press h + enter to show help
```

## Production Build

TBA
