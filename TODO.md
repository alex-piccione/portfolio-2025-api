# TODO

## In Progress

_Nothing currently in progress._

## Backlog

- [ ] **Testing** Add unit/integration test suite (N/A framework choice for Rust: `cargo test` + `tokio::test`)
  - [ ] Tests for services layer (pure logic)
  - [ ] Integration tests for endpoints with test Postgres container
- [ ] **CI** Add/verify GitHub Actions workflow for build + test on every PR to `main`
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Docs** Update README setup instructions (typo fixes, .env reference)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

_Nothing completed yet._
