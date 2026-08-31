# TODO

## In Progress

- **[feat/06_endpoint_integration_tests]** ✅ COMPLETE — integration tests with testcontainers + real Postgres
  - Branch: `feat/06_endpoint_integration_tests` — PR #9 open: https://github.com/alex-piccione/portfolio-2025-api/pull/9
  - All compile errors fixed; `cargo check --tests` passes (unit + integration tests)
  - Unit tests pass (17/17)
  - CI workflow updated (`pr_check.yml`) to run Docker-based integration tests
  - Ready for merge

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
- **[refactor/05_extract_generate_token]** DRY extraction of `generate_token` (implementation + unit tests) from auth_service/session_service into shared `src/utils/token.rs`.
- **[feat/04_readme_docs]** Update README documentation — typo and broken-link fixes, new environment-variable reference section, and clarified SQLx setup notes.
- **[feat/03_service_layer_tests]** Add unit tests for the services layer (pure logic)
  - Survey services layer — most services are DB-bound; pure-logic units: `password_hashing`, `generate_token` (auth_service + session_service), `utils::datetime`
  - Write unit tests: `password_hashing` (hash/verify roundtrip, wrong password rejection, salted uniqueness, invalid format panic), `generate_token` (non-empty, 64-char length, uniqueness, URL-safe alphabet), `datetime::now()` (within tolerance), `datetime::today()` (matches today), `datetime::try_from()` (RFC3339, ISO with microseconds, date-only, error on invalid input)
  - 17 tests passing via `cargo test`
- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).
