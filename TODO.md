# TODO

## In Progress

_Nothing currently in progress._

## Backlog

- [ ] **[feat/03_service_layer_tests]** Add unit tests for the services layer (pure logic)
  - [ ] Survey services layer (auth, currency, custodian, holding, session, user, password_hashing) and identify pure-logic units testable without DB/network
  - [ ] Add dev-dependencies (`tokio` test features, etc.) to Cargo.toml
  - [ ] Write unit tests per service module (incl. password_hashing, token/session logic)
  - [ ] Run `cargo test` and wire into existing PR Check workflow (already covered: workflow runs `cargo test`)
- [ ] **Testing** Integration tests for endpoints with test Postgres container (follow-up after unit tests)
  - [ ] Add testcontainers-based Postgres fixture
  - [ ] Integration tests for main endpoints
- [ ] **Auth** Review session/refresh-token flow (Sessions table, refresh date migration)
- [ ] **Jobs** Document and test scheduled jobs (currency rates via CoinGecko)

## Completed

- **[feat/04_readme_docs]** Update README documentation — fixed typos (`scghema cheks`, `continuosly`, `thte`, `instruciton`, `colelctions`, `paly`, `avantages`, `RUST_LOG=info=info`), fixed badge repo name and broken `devop/README.md` link, added env-var reference table (`DATABASE_URL`, `CONFIGURATION_FILE`, `RUST_LOG`; runtime settings come from the configuration JSON) and clarified SQLx offline-mode note for CI.
- **[feat/02_ci_workflow]** Add PR Check workflow — `.github/workflows/pr_check.yml`: cargo check, build, test on every PR to `main`, with `SQLX_OFFLINE=true` using the committed `.sqlx` cache (no DB in CI). Committed directly on `main` by the owner (PAT lacks `workflow` scope).

_Nothing else completed yet._
