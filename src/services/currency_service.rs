use dashmap::DashMap;
//e std::sync::{LazyLock, RwLock}; // Rust doesn't allow "static mut" :-(
use crate::{entities::currency::{Currency}, repositories::{currency_repository::CurrencyRepository}};
use crate::endpoints::models::currency_models::CurrencyOfUser;

#[derive(Clone)]
pub struct CurrencyService {
    repository: CurrencyRepository,
    // Thread-safe cache
    currencies: DashMap<i32, Currency>,
}

/*
static CURRENCIES: LazyLock<RwLock<HashMap<i32, Currency>>> = 
    LazyLock::new(|| RwLock::new(HashMap::new())); */

impl CurrencyService {

   /*ub async fn new(repository: CurrencyRepository) -> Self {
        let instance = Self { repository };

        // Load the cache
        instance.load_cache().await;

        instance
    }*/

    pub fn new(repository: CurrencyRepository) -> Self {
        Self {
            repository,
            currencies: DashMap::new(), // Initialize empty
        }
    }

    // Initialize cache at startup (called only once)
    pub async fn init_cache(&self) -> Result<(), String> {
        let items: Vec<Currency> = self.repository.list().await?;
        
        self.currencies.clear();
        for currency in items {
            self.currencies.insert(currency.id, currency);
        }
        Ok(())
    }

    pub fn get(&self, id: i32) -> Currency {
        match self.currencies.get(&id) {
            Some(c) => c.value().clone(),
            None => panic!("Currency not found with id={}.", id)
        }
    }

    pub fn try_get(&self, id: i32) -> Option<Currency> {
        self.currencies.get(&id).map(|entry| entry.clone())
    }

    pub fn all(&self) -> Vec<Currency> {
        self.currencies.iter().map(|entry| entry.value().clone()).collect()
    }

    // Database operations with cache updates
    pub async fn create(&self, item: Currency) -> Result<i32, String> {
        let id = self.repository.create(item.clone()).await?;
        
        // Update cache - thread-safe, no locks!
        self.currencies.insert(id, item);
        
        Ok(id)
    }

    pub async fn update(&self, item: Currency) -> Result<(), String> {
        self.repository.update(item.clone()).await?;
        
        // Update the cache
        self.currencies.insert(item.id, item);
        
        Ok(())
    }

    pub async fn list_for_user(&self, user_id: &str) -> Result<Vec<CurrencyOfUser>, String> {

        let used_by_user: Vec<i32> = self.repository
            .list_of_user(user_id).await?
            .into_iter()
            .map(|record| record.currency_id)
            .collect();

        let items = self.currencies.clone()
            .iter()
                .filter(|currency| currency.is_active)
                .map(|currency| 
                    CurrencyOfUser {
                        id: currency.id,
                        symbol: currency.symbol.clone(),
                        name: currency.name.clone(),
                        kind: currency.kind.clone().into(),
                        is_used: used_by_user.contains(&currency.id),
                    }
                )
                .collect();

        Ok(items)
    }
        // 3. get all the currencies: Ok.
        // 4. use only the active currency and return a list of CurrencyOfUser, setting the is_used property based on currency id existing in the map/vac/hashlist created at point 2.
        //let mut items = Vec::with_capacity(self.currencies.len());
        //for currency in self.currencies.iter().any( c -> c.is_active) {
        //    //if(currency.is_active)

        //}
        
        // 5. return the list
        //Ok(items)

    /*
    pub async fn delete(&self, id: i32) -> Result<(), String> {
        self.repository.delete(id).await?;
        
        // Remove from cache
        self.currencies.remove(&id);
        
        Ok(())
    }
    */


/*
    async fn load_cache(&self) {
        match self.repository.list().await {
            Ok(currencies) => {
                let map = currencies.into_iter().map(|c|(c.id, c)).collect();
                CURRENCIES.(map);
            }
            Err(e) => {
                eprintln!("Failed to load currencies into cache: {}", e);
            }
        }
    }

    pub async fn create(&self, item: Currency) -> Result<i32, String> {
        let result = self.repository.create(item).await;
        self.load_cache().await; // Update cache
        result
    }

    pub async fn update(&self, item: Currency) -> Result<(), String> {
        let result = self.repository.update(item).await;
        self.load_cache().await; // Update cache
        result
    }

    pub fn all() -> Vec<Currency> { CURRENCIES.values().cloned().collect() }

    pub fn get(id: i32) -> Option<Currency> {
        CURRENCIES.get(&id).cloned()
    }
    */
}