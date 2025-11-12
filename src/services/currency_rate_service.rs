use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{constants, info, warn};
use crate::repositories::currency_rate_repository::CurrencyRateRepository;
use crate::repositories::schemas::currency_rate_record::CurrencyRateRecord;
use crate::services::Coingecko::coingecko_api::CoingeckoApi;
use crate::services::Coingecko::currencies_map::COINGECKO_QUOTE_IDS;
use crate::services::currency_service::CurrencyService;
use crate::utils::datetime::{Date, now, today};

#[derive(Clone)]
pub struct CurrencyRateService {
    repository: CurrencyRateRepository,
    currency_service: CurrencyService,
    coingecko_api: CoingeckoApi
}

impl CurrencyRateService {
    pub fn new(repository: CurrencyRateRepository, currency_service: CurrencyService, coingecko_api: CoingeckoApi) -> Self {
        Self { repository, currency_service, coingecko_api }
    }

    pub async fn create(&self, record: &CurrencyRateRecord) -> Result<(), String> {
        self.repository.create(record).await
    }

    pub async fn search(&self, base_currency_id: i32, quote_currency_id: i32, date: Option<Date>) -> Result<Vec<CurrencyRateRecord>, String> {
        self.repository.search(base_currency_id, quote_currency_id, date).await
    }

    pub async fn list_at_date(&self, date: Date) -> Result<Vec<CurrencyRateRecord>, String> {
        self.repository.list_at_date(date).await
    }

    /// Retrieve the currency rates from Coingecko.
    /// Coingecko uses its ids to identify Base currencies and has fixed ids for the Quote currencies.
    /// Base currencies are Cypto and stable coin, and Quote currencies are fiat.
    /// We use a fixed map for mapping Coingecko currency ID to currency symbols.    
    pub async fn get_rates_from_coingecko(&self) -> Result<Vec<CurrencyRateRecord>, String> {

        let base_ids: Vec<String> = self.currency_service.crypto_and_stable_currencies
            .iter()
            .filter_map(|entry| entry.value().coingecko_id.clone())
            .collect();

        // We use our coins as base currencies
        /*let __base_ids = self.currency_service.crypto_and_stable_currencies
            .iter()
            .filter(|cur| cur.coingecko_id.is_some())
            .map(|f| f.coingecko_id.unwrap())  // Safe because we filtered out None values
            //.map(|x| &x)
            .collect::<Vec<_>>();*/

        /*
        let _base_ids = self.currency_service.crypto_and_stable_currencies
            .iter()
            .filter_map(|cur|cur.coingecko_id.clone().as_deref())  // This will automatically filter out None and convert Option<&String> to Option<&str>
            .collect::<Vec<_>>();       
        */

        // We use a fixed list of coins as quote
        let quote_ids = COINGECKO_QUOTE_IDS
            .iter()
            .map(|(_, cg_id)|*cg_id)
            .collect::<Vec<_>>();

        info!("base_ids {}", base_ids.iter().count());
        info!("base_ids {}", base_ids.iter().count());

        let result = self.coingecko_api.get_rates(&base_ids, &quote_ids).await;

        match result {
            Ok(coingecko_rates) => {
                //let rate: f64 = serde_json::from_str(&text).unwrap_or(0.0);
                info!("got coingecko rates");

                let mut rates: Vec<CurrencyRateRecord> = vec![];
                for (coin, map) in  coingecko_rates {

                    if let Some(base_currency) = self.currency_service.try_get_by_Coingecko_id(&coin) {
                        for (symbol, rate) in map {
                            if let Some(quote_currency) = self.currency_service.try_get_by_symbol_CI(&symbol) {

                                info!("Add currency rate for {}/{}: {}", base_currency.symbol, quote_currency.symbol, rate);

                                let rate = CurrencyRateRecord {
                                    base_currency_id: base_currency.id,
                                    quote_currency_id: quote_currency.id,
                                    date: today(),
                                    rate: Decimal::from_f64(rate).expect(&format!("f64 to Decimal conversion failed for {}", rate)),
                                    source: constants::external_services::COINGECKO.to_owned(),
                                    created_at: now()
                                };

                                rates.push(rate);

                            } else { warn!("Currency not found with symbol: {}", symbol); }
                        }
                    } else { warn!("Currency not found with Coingecko name: {}", coin); }
                } 

                Ok(rates)
            },
            Err(e) => Err(format!("Failed to get rates. {}", e))
        }
    }
}