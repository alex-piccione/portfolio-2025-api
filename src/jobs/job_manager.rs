use crate::{configuration::Configuration, info, jobs::Coingecko::coingecko_api::CoingeckoApi, utils::dependency_injection::AppState, warn};
use tokio_cron_scheduler::{Job, JobScheduler};
use async_trait::async_trait;

pub async fn schedule_jobs(config: &Configuration, app_state: AppState) {

    let scheduler = JobScheduler::new().await.unwrap();

    // Update Exchange Rate
    let update_exchange_rates_job = UpdateExchangeRatesJob::new(&config, app_state.clone());

    scheduler.add(
        Job::new_async(&config.jobs.update_exchange_rate_cron, move |_id, mut _lock| {
            let job = update_exchange_rates_job.clone();
            Box::pin(async move {
                job.run().await;
            })
        }).unwrap()
    ).await.unwrap();

    scheduler.start().await.unwrap();

    println!("'Update Exchange Rate' job scheduled ({})", &config.jobs.update_exchange_rate_cron);

    // Spawn the service in the background instead of awaiting it
    /*tokio::spawn(async move {
        service.await;
    });*/
}

/*
fn parse_cron(cron_expression: &str) -> Job<chrono::offset::Utc> {
    if cron_expression.split(" ").count() != 6 {
        panic!("CRON expression must be 6-field format. '{}' is not.", cron_expression)
    }

    Job::cron(cron_expression)
        .map_err(|e| format!("Failed to parse cron expression '{}'. {e}", cron_expression))
        .unwrap()
}*/

// async_trait] is necessary because Rust async traits are not yet natively supported in stable.
// Added Send + Sync bounds, which are typically required for types shared across threads.
#[async_trait]
trait RecurringJob: Send + Sync {
    async fn run(&self) -> ();
}


#[derive(Clone)]
struct UpdateExchangeRatesJob {
    config: Configuration,
    app_state: AppState,
    api: CoingeckoApi
}

impl UpdateExchangeRatesJob {
    pub fn new(config: &Configuration, app_state: AppState) -> Self {
        let api = CoingeckoApi::new(config);
        Self { 
            config: config.clone(), 
            app_state, 
            api 
        }
    }
}

#[async_trait]
impl RecurringJob for UpdateExchangeRatesJob {
    async fn run(&self) -> () {
        println!("URL: {}", self.config.secrets.coingecko_api_key.chars().count());
        let _currencies = self.app_state.currency_service.all();
        
        match self.api.ping().await {
            true => info!("Coingecko Pink OK"),
            false => warn!("Coingecko ping FAIL. ")
        };
        ()
        //println!("URL: {}", self.config.jobs)
    }
}
    /*fn run (&self) -> () {
        println!("Job Update Exchange Rates RUN");
        crate::info!("UpdateExchangeRatesJob RUN");

        self.run_internal();
    }*/
