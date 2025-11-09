use crate::{configuration::Configuration, utils::dependency_injection::AppState};
use async_cron_scheduler::{Job, Scheduler};

pub async fn schedule_jobs(config: &Configuration, app_state: AppState) {

    let (mut scheduler, service) = Scheduler::<chrono::offset::Utc>::launch(smol::Timer::after);

    // Update Exchange Rate
    let update_exchange_rates_job = UpdateExchangeRatesJob::new(config.clone(), app_state.clone());
    scheduler.insert(
        parse_cron(&config.jobs.update_exchange_rate_cron),
     move |_id | update_exchange_rates_job.run()
        ).await;
    println!("'Update Exchange Rate' job scheduled ({})", &config.jobs.update_exchange_rate_cron);

    // Spawn the service in the background instead of awaiting it
    tokio::spawn(async move {
        service.await;
    });
}

fn parse_cron(cron_expression: &str) -> Job<chrono::offset::Utc> {
    if cron_expression.split(" ").count() != 6 {
        panic!("CRON expression must be 6-field format. '{}' is not.", cron_expression)
    }

    Job::cron(cron_expression)
        .map_err(|e| format!("Failed to parse cron expression '{}'. {e}", cron_expression))
        .unwrap()
}

trait RecurringJob {
    fn run(&self) -> ();
}


impl RecurringJob for UpdateExchangeRatesJob {
    fn run (&self) -> () {
        println!("Job Update Exchange Rates RUN");
        crate::info!("UpdateExchangeRatesJob RUN");

        self.run_internal();
    }
}

#[derive(Clone)]
struct UpdateExchangeRatesJob {
    config: Configuration,
    app_state: AppState
}

impl UpdateExchangeRatesJob {
    pub fn new(config: Configuration, app_state: AppState) -> Self {
        Self { config, app_state }
    }

    pub fn run_internal(&self) -> () {
        println!("URL: {}", self.config.secrets.coingecko_api_key.chars().count());
        let _currencies =self.app_state.currency_service.all();
        //println!("URL: {}", self.config.jobs)
    }
}