use crate::{configuration::Configuration, info, jobs::update_currency_rates_job::UpdateCurrencyRatesJob, utils::dependency_injection::AppState};
use tokio_cron_scheduler::{Job, JobScheduler};
use async_trait::async_trait;

/// Registers all recurring jobs with the scheduler and starts it.
///
/// Called once at application startup; the scheduler then drives each
/// job according to its cron expression from `config.jobs`.
pub async fn schedule_jobs(config: &Configuration, app_state: AppState) {

    let scheduler = JobScheduler::new().await.unwrap();

    // Update Exchange Rate
    let update_exchange_rates_job = UpdateCurrencyRatesJob::new(&config, app_state.clone());

    scheduler.add(
        Job::new_async(&config.jobs.update_exchange_rate_cron, move |_id, mut _lock| {
            let job = update_exchange_rates_job.clone();
            Box::pin(async move {
                job.run().await;
            })
        }).unwrap()
    ).await.unwrap();

    scheduler.start().await.unwrap();

    info!("'Update Exchange Rate' job scheduled ({})", &config.jobs.update_exchange_rate_cron);

    // Spawn the service in the background instead of awaiting it
    /*tokio::spawn(async move {
        service.await;
    });*/
}


/// A job that can be executed repeatedly by the scheduler.
///
/// `Send + Sync` are required because the scheduler may invoke the job
/// from a different thread than the one that created it.
#[async_trait]
pub trait RecurringJob: Send + Sync {
    /// Executes one run of the job. Implementations should handle their own errors.
    async fn run(&self) -> ();
}

