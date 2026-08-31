# TODO

## In Progress

- **[feat/06_endpoint_integration_tests]** Integration tests for main endpoints using testcontainers + real Postgres
  - Branch: `feat/06_endpoint_integration_tests` (pushed; PR #9 open: https://github.com/alex-piccione/portfolio-2025-api/pull/9)
  - Dev-dependencies added and build verified: `testcontainers = "0.26"`, `testcontainers-modules = "0.14"`, `tokio-test = "0.4.5"`
  - File created: `tests/integration_tests.rs` (~260 lines) with a `postgres_pool()` fixture that:
      - spins up `Postgres::default()` (user `portfolio_user`, password `portfolio_password`, db `portfolio`)
      - runs `sqlx::migrate!("./migrations")` on the pool so the schema is ready
      - returns a `sqlx::PgPool`
  - Test functions written:
      - `home_returns_ok`, `config_returns_ok`
      - `signup_creates_user_and_returns_session`, `login_returns_session`, `refresh_token_returns_new_tokens`
      - `currency_list_all_returns_currencies`, `currency_single_returns_one`, `currency_create_and_delete_cycle`
      - `custodian_list_returns_custodians`
      - `holding_create_and_list_cycle`, `holding_list_returns_items`
  - Compile errors fixed:
      - Added `src/lib.rs` with `pub mod` declarations for all public modules (`configuration`, `constants`, `endpoints`, `entities`, `repositories`, `services`, `utils`, `jobs`) plus `pub use utils::dependency_injection`
      - Changed `tests/integration_tests.rs` imports from `crate::…` to `portfolio_api::…` (integration tests resolve `crate::` to the test crate, not the library)
      - Fixed `sqlx::PoolOptions` → `sqlx::PgPool::connect` (sqlx 0.8 removed `PgPoolOptions`)
      - Fixed `container.get_host_port_ipv4()` / `get_host()` → `.await` (returns `Future`)
      - Fixed `Session` type alias confusion: `utils::auth_middleware::Session` is `axum::Extension<SessionWithUser>`, not `entities::session::Session`; construct `SessionWithUser` from the login result
      - Fixed endpoint function args: pass `axum::extract::State(state.clone())` instead of `AppState::clone(&state)`
      - Fixed `LoginError` not implementing `Debug`: use `.unwrap_or_else()` instead of `.expect()`
  - `cargo check --tests` now passes (both unit and integration tests compile)
  - Unit tests pass (17/17)
  - NOTES / next step:
      - Integration tests require Docker to run: `cargo test --test integration_tests -- --test-threads=1`
      - The `login` helper creates a real user in DB (`testuser`) — use `--test-threads=1` to avoid duplicate-key failures on parallel runs
      - If Docker is unavailable, use `--skip` or run on a machine with Docker

## Backlog

- [ ] **Testing** Integration tests for endpoints with test Postgres container (follow-up after unit tests)
  - [ ] Add testcontainers-based Postgres fixture  ← DONE (see In Progress)
  - [ ] Integration tests for main endpoints (DB-backed variant)  ← COMPILE PASS (tests written; run on Docker-enabled machine)
  - [ ] `list_of_user` endpoint integration test
- [ ] **[refactor/05_extract_generate_token]** Extract `generate_token` from `auth_service` + `session_service` into `src/utils/token.rs` (DRY)
  - [ ] Create `src/utils/token.rs` with shared implementation (+ module declaration in `utils/mod.rs`), move its unit tests along
  - [ ] Remove duplicated implementations and their test blocks from `auth_service.rs` / `session_service.rs`; import from `crate::utils::token`
  - [ ] Verify locally (`cargo build`/`test`, `SQLX_OFFLINE=true`) and via PR Check workflow
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Docs** Update README setup instructions (typo fixes, .env reference)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

- **[feat/06_endpoint_integration_tests]** Add integration tests with testcontainers Postgres fixture — all compile errors fixed, `cargo check --tests` passes
  - Added `src/lib.rs` with `pub mod` declarations + `pub use utils::dependency_injection`
  - Fixed dev-deps (`testcontainers-modules = { version = "0.14", features = ["postgres"] }`)
  - Rewrote `tests/integration_tests.rs` to use `portfolio_api::…` imports, `sqlx::PgPool::connect`, `Postgres::default()` (no arg), `with_db_name`, `container.get_host().await`
  - Fixed `Session` type alias and `State<AppState>` constructor issues
- **[feat/04_readme_docs]** Update README documentation — typo and broken-link fixes, new environment-variable reference section, and clarified SQLx setup notes.
- **[feat/03_service_layer_tests]** Add unit tests for the services layer (pure logic)
  - Survey services layer — most services are DB-bound; pure-logic units: `password_hashing`, `generate_token` (auth_service + session_service), `utils::datetime`
  - Write unit tests: `password_hashing` (hash/verify roundtrip, wrong password rejection, salted uniqueness, invalid format panic), `generate_token` (non-empty, 64-char length, uniqueness, URL-safe alphabet), `datetime::now()` (within tolerance), `datetime::today()` (matches today), `datetime::try_from()` (RFC3339, ISO with microseconds, date-only, error on invalid input)
  - 17 tests passing via `cargo test`
- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).
