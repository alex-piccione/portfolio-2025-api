// we use SQLx OffsetDateTime because the other type (PrimitiveDateTime) is ambiguous, it doesnt have timezone so it will be stored as TIMESTAMP, not TIMESTAMPTZ.
// TIMESYAMPTZ is required to have UTC date in Postgres.
//
// usage:
// UtcDateTime::now() ✅ 
// UtcDateTime::try_from(date as string) 

use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

// type alias (does not allow impl)
pub type UtcDateTime = DateTime<Utc>;

pub fn now() -> DateTime<Utc> {
    chrono::Utc::now()
}

pub fn try_from(s: String) -> Result<UtcDateTime, String> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ") {
        return Ok(dt.with_timezone(&Utc));
    }
    
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ") {
        return Ok(dt.with_timezone(&Utc));
    }
    
    if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
        let naive_datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc));
    }
    
    Err(format!("Unable to parse datetime from string: {}", s))
}









// Newtype wrapper  (so I can implement traits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Deserialize, Serialize)] 
pub struct AppDateTime(pub DateTime<Utc>);
//pub struct UtcDateTime { pub field1: DateTime<Utc> }

/* 
use std::str::FromStr;

impl From<String> for UtcDateTime {
    fn from(s: String) -> Self {
        // Try parsing as full ISO8601 DateTime first
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return dt.with_timezone(&Utc);
        }
        
        // Try parsing as ISO8601 with various formats
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ") {
            return dt.with_timezone(&Utc);
        }
        
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ") {
            return dt.with_timezone(&Utc);
        }
        
        // Try parsing as simple date format "YYYY-MM-DD"
        if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            let naive_datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);
        }
        
        // Fallback: panic or return epoch (choose based on your error handling strategy)
        panic!("Unable to parse datetime from string: {}", s);
    }
}
*/

use std::convert::TryFrom;

impl TryFrom<String> for AppDateTime {
    type Error = String;
    
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(AppDateTime(dt.with_timezone(&Utc)));
        }
        
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ") {
            return Ok(AppDateTime(dt.with_timezone(&Utc)));
        }
        
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ") {
            return Ok(AppDateTime(dt.with_timezone(&Utc)));
        }
        
        if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            let naive_datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Ok(AppDateTime(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc)));
        }
        
        Err(format!("Unable to parse datetime from string: {}", s))
    }
}


//use serde::{Deserialize, Serialize};
//use sqlx::types::time::OffsetDateTime;

//pub use sqlx::types::time::OffsetDateTime as UtcDateTime;

/*
pub fn utc_now() -> OffsetDateTime { 
    OffsetDateTime::now_utc() 
}*/

//#[derive(Debug, Clone, Copy)]
//#[derive(Serialize, Deserialize)]
/*
pub struct UtcDateTime(OffsetDateTime);

impl UtcDateTime {
    pub fn now() -> UtcDateTime {
        UtcDateTime(OffsetDateTime::now_utc())
    }

    //pub fn utc() -> Self { OffsetDateTime::now_utc() }
    
    // Deref to inner type for database operations (OffsetDateTime is )
    pub fn as_timestamptz(&self) -> OffsetDateTime {
        self.0
    }

    pub fn from_timestamptz(value:OffsetDateTime) -> UtcDateTime {
        UtcDateTime(value)
    }
}
*/