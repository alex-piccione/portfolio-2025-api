# TODO

## In Progress

_Nothing currently in progress._

## Backlog

- [ ] **Testing** Add unit/integration test suite (N/A framework choice for Rust: `cargo test` + `tokio::test`)
  - [ ] Tests for services layer (pure logic)
  - [ ] Integration tests for endpoints with test Postgres container
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Docs** Update README setup instructions (typo fixes, .env reference)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).

_Nothing else completed yet._
