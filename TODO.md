# TODO

## In Progress

- **[feat/06_endpoint_integration_tests]** Integration tests for main endpoints using testcontainers + real Postgres
  - Branch: `feat/06_endpoint_integration_tests` (pushed; PR #9 open: https://github.com/alex-piccione/portfolio-2025-api/pull/9)
  - Dev-dependencies added and build verified: `testcontainers = "0.26"`, `testcontainers-modules = "0.14"`, `tokio-test = "0.4.5"`
  - File created: `tests/integration_tests.rs` (~260 lines) with a `postgres_pool()` fixture that:
      - spins up `Postgres::default("17.6")` (user `portfolio_user`, password `portfolio_password`, db `portfolio`)
      - runs `sqlx::migrate!("./migrations")` on the pool so the schema is ready
      - returns a `sqlx::PgPool`
  - Test functions written (compile individually but cannot run because Docker is unavailable in this environment):
      - `home_returns_ok`, `config_returns_ok`
      - `signup_creates_user_and_returns_session`, `login_returns_session`, `refresh_token_returns_new_tokens`
      - `currency_list_all_returns_currencies`, `currency_single_returns_one`, `currency_create_and_delete_cycle`
      - `custodian_list_returns_custodians`
      - `holding_create_and_list_cycle`, `holding_list_returns_items`
  - NOTES / next step for tomorrow:
      1. Docker must be running. If Docker is unavailable, skip `#[tokio::test]` functions that need the container or add a `--skip` filter.
      2. `cargo test --test integration_tests` will currently FAIL at compile time because helper modules (`endpoints`, `services`, `utils`, `dependency_injection`, `configuration`, `entities`, `repositories`) are referenced as `crate::…` but are NOT re-exported from the crate root — the binary crate (`src/main.rs`) uses `mod` declarations, not `pub mod` + re-exports. Fix options:
         - Add a `src/lib.rs` that re-exports the public modules (`pub mod endpoints; pub mod services; …`) so integration tests can use `crate::endpoints::…`, OR
         - Convert integration tests to inline `#[test]` modules inside `src/` (but then they can't use `#[tokio::test]` + testcontainers as cleanly), OR
         - Add `pub use` re-exports in `src/main.rs` and annotate the crate as both lib+bin.
      3. The `cargo build --tests` and `cargo check --tests` both pass for the UNIT tests (17 tests passing). Integration tests compile ONLY if the crate-root re-export issue is fixed.
      4. After fixing the re-export issue, run `cargo test --test integration_tests` to validate end-to-end.
      5. The `login` helper creates a real user in DB — consider adding a `#[ctor::test]` or a `once_cell` user-fixture to avoid duplicate-username failures across parallel test runs (run tests with `--test-threads=1` or use unique usernames).

## Backlog

- [ ] **Testing** Integration tests for endpoints with test Postgres container (follow-up after unit tests)
  - [ ] Add testcontainers-based Postgres fixture  ← DONE (see In Progress)
  - [ ] Integration tests for main endpoints (DB-backed variant)  ← IN PROGRESS (tests written; crate-root re-export fix needed)
- [ ] **[refactor/05_extract_generate_token]** Extract `generate_token` from `auth_service` + `session_service` into `src/utils/token.rs` (DRY)
  - [ ] Create `src/utils/token.rs` with shared implementation (+ module declaration in `utils/mod.rs`), move its unit tests along
  - [ ] Remove duplicated implementations and their test blocks from `auth_service.rs` / `session_service.rs`; import from `crate::utils::token`
  - [ ] Verify locally (`cargo build`/`test`, `SQLX_OFFLINE=true`) and via PR Check workflow
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Docs** Update README setup instructions (typo fixes, .env reference)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

- **[feat/04_readme_docs]** Update README documentation — typo and broken-link fixes, new environment-variable reference section, and clarified SQLx setup notes.
- **[feat/03_service_layer_tests]** Add unit tests for the services layer (pure logic)
  - Survey services layer — most services are DB-bound; pure-logic units: `password_hashing`, `generate_token` (auth_service + session_service), `utils::datetime`
  - Write unit tests: `password_hashing` (hash/verify roundtrip, wrong password rejection, salted uniqueness, invalid format panic), `generate_token` (non-empty, 64-char length, uniqueness, URL-safe alphabet), `datetime::now()` (within tolerance), `datetime::today()` (matches today), `datetime::try_from()` (RFC3339, ISO with microseconds, date-only, error on invalid input)
  - 17 tests passing via `cargo test`
- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).
