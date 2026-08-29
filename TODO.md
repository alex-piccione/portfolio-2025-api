# TODO

## In Progress

_Nothing currently in progress._

## Backlog

- [ ] **Testing** Integration tests for endpoints with test Postgres container (follow-up after unit tests)
  - [ ] Add testcontainers-based Postgres fixture
  - [ ] Integration tests for main endpoints
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Docs** Update README setup instructions (typo fixes, .env reference)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

- **[refactor/05_extract_generate_token]** DRY extraction of `generate_token` (implementation + unit tests) from auth_service/session_service into shared `src/utils/token.rs`.
- **[feat/04_readme_docs]** Update README documentation — typo and broken-link fixes, new environment-variable reference section, and clarified SQLx setup notes.
- **[feat/03_service_layer_tests]** Add unit tests for the services layer (pure logic)
  - Survey services layer — most services are DB-bound; pure-logic units: `password_hashing`, `generate_token` (auth_service + session_service), `utils::datetime`
  - Write unit tests: `password_hashing` (hash/verify roundtrip, wrong password rejection, salted uniqueness, invalid format panic), `generate_token` (non-empty, 64-char length, uniqueness, URL-safe alphabet), `datetime::now()` (within tolerance), `datetime::today()` (matches today), `datetime::try_from()` (RFC3339, ISO with microseconds, date-only, error on invalid input)
  - 17 tests passing via `cargo test`
- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).

_Nothing else completed yet._
