//! Background scheduled jobs.
//!
//! Jobs are registered at startup by [`job_manager::schedule_jobs`] and run
//! on cron schedules defined in the configuration file (`jobs.*_cron`).
//! Cron expressions use 6 fields (seconds first), e.g. `"0 */10 * * * *"`
//! runs at second 0 of every 10th minute.
//!
//! Currently implemented: [`update_currency_rates_job`] — fetches currency
//! exchange rates from CoinGecko and persists them via the currency rate service.

pub mod job_manager;

mod update_currency_rates_job;
