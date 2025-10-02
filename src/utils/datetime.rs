// we use SQLx OffsetDateTime because the other type (PrimitiveDateTime) is ambiguous, it doesnt have timezone so it will be stored as TIMESTAMP, not TIMESTAMPTZ.
// TIMESYAMPTZ is required to have UTC date in Postgres.
//
// usage: UtcDateTime::now() ✅ 

use sqlx::types::time::OffsetDateTime;

//pub use sqlx::types::time::OffsetDateTime as UtcDateTime;

/*
pub fn utc_now() -> OffsetDateTime { 
    OffsetDateTime::now_utc() 
}*/

//#[derive(Debug, Clone, Copy)]
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

    pub fn from_timestamptz(&self, value:OffsetDateTime) -> UtcDateTime {
        UtcDateTime(value)
    }
}