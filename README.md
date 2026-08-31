# leafyPuff

A personal diary app. Warm sage palette, hand-drawn bunny faces as moods, light and dark themes, and everything the UI reads but should not compute pushed into a shared Rust core.

Local-first: the app works with no network and no account. Sync is additive, and nothing the server does may break that.

## Architecture

Hexagonal (ports and adapters) inside each crate, one Cargo workspace across them. The two are separate concerns — layering is intra-crate, the workspace is how crates are organised.

```
apps/
├── core/      leafypuff-core  — shared Rust core, hexagonal, reused on both tiers
├── api/       leafypuff-api   — Axum sync API, hexagonal
├── android/   Kotlin + Jetpack Compose client (Gradle, not a workspace member)
└── web/       TanStack CMS (pnpm, not a workspace member)
```

Inside `apps/core` and `apps/api`:

```
domain/          entities, ports, errors — imports nothing internal
application/     use cases, one per file, each exposing execute()
infrastructure/  adapters implementing the ports
http/            transport only; DTOs live here and never leak inward   (api only)
main.rs          the only file that wires all four                      (api only)
```

`domain` imports nothing internal · `application` imports `domain` · `infrastructure` and `http` import `domain` · the composition root wires them. A persistence model never crosses out of `infrastructure`; domain types are separate, with `From` mappers.

## Stack

| Tier | Choice |
|---|---|
| Client | Kotlin, Jetpack Compose |
| Shared core | Rust, exposed to Kotlin over UniFFI |
| Sync API | Rust, Axum |
| CMS web | TanStack Router / Query / Store, `ts-pattern` |
| Database | PostgreSQL on the server, SQLite on the device — one SeaORM entity model over both |
| Object storage | MinIO, S3-compatible, via `aws-sdk-s3` |
| Image processing | `image` for derivatives; `imagekit2-core` for watermarking |

## Conventions

**Rust** — no `unwrap`/`expect`/`panic!` on request-reachable paths · migrations generated, committed with the schema change, never edited after apply · enums over booleans · response envelope on every route · snake_case wire keys · no inline comments.

**Web** — no React hooks: server state is TanStack Query, client state is TanStack Store, URL state is TanStack Router. `type` only, `T`-prefixed, no `any`, no `as`. No ternaries in UI — `ts-pattern` instead. Text goes through `Typography`, never a raw `<span>`. Every action and every read view is wrapped in `<Guard>`; permission keys are `PERMISSION.*` constants, never raw strings.

**Compose** — prototype `px` reads as `.dp`, font `px` as `.sp`. Both palettes ship verbatim as hand-picked pairs; the dark one is never derived from the light one.

## Development

```bash
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

pnpm -C apps/web install
pnpm -C apps/web dev
pnpm -C apps/web typecheck
```

Git hooks run through lefthook: formatting on commit, and the full gate — fmt, clippy, tests, typecheck, lint, and a grep that rejects React hooks — on push. Commits follow Conventional Commits with a mandatory scope on `feat` and `fix`.

## Status

Scaffold. The layering, workspace, theme tokens, permission catalog and health endpoints are in place; features land per module from here.
