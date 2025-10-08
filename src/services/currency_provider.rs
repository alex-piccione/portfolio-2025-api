use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::{
    entities::currency::Currency
};

#[allow(dead_code)]
static CURRENCIES: Lazy<RwLock<Vec<Currency>>> = Lazy::new(|| RwLock::new(vec![]));

#[allow(dead_code)]
pub struct CurrencyProvider;

impl CurrencyProvider {    

    pub fn _fill(currencies: Vec<Currency>) {
        let mut cache = CURRENCIES.write().unwrap();
        *cache = currencies;
    }

    pub fn __try_get(id: i32) -> Option<Currency> {
        let cache = CURRENCIES.read().unwrap();
        cache.iter().find(|c| c.id == id).cloned()
    }

    pub fn _get(id: i32) -> Currency {
        let cache = CURRENCIES.read().unwrap();
        cache.iter()
            .find(|c| c.id == id)
            .cloned()  // ✅ Clone before returning
            .unwrap()  // Panic if not found
    }
}