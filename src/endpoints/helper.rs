use async_trait::async_trait;
use axum::extract::{FromRequestParts, Query};
use reqwest::StatusCode;
use serde::Deserialize;
//use serde_with::{serde_as, DeserializeFromStr};

use crate::utils::datetime::{UtcDateTime, try_from as try_get_date_from_string};

/**
 * Parse the "YYYY-MM-DD" date to "YYYY-MM-DDT00:00:00Z"
 */
pub fn parse_date(date: Option<String>) -> Result<Option<UtcDateTime>, String> {
    match date {
        Some(s) => try_get_date_from_string(s).map(Some),
        None => Ok(None)
    }
}

/*
impl std::str::FromStr for UtcDateTime {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UtcDateTime::try_from(s.to_string())
    }
}*/


/*  serde_as is not recognized... */
/*
#[serde_as]
#[derive(Deserialize)]
pub struct DateQuery {
    #[serde_as(as = "Option<DeserializeFromStr>")]
    pub date: Option<UtcDateTime>,
}
*/

use serde::{Deserializer};


#[derive(Deserialize)]
pub struct DateQuery {
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub date: Option<UtcDateTime>,
}


fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<UtcDateTime>, D::Error>
where D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        //Some(s) => UtcDateTime::try_from(s)
        Some(s) => try_get_date_from_string(s)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub struct DateTimeQuery(pub Option<UtcDateTime>);

/*
#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for DateTimeQuery
where S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts, 
        state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct QueryParams {
            date: Option<String>,
        }

        let axum::extract::Query(params) = axum::extract::Query::<QueryParams>::from_request_parts(parts, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid query params: {}", e)))?;

        let parsed_date = parse_date(params.date)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date format: {}", e)))?;

        Ok(DateTimeQuery(parsed_date))
    }
}
    */