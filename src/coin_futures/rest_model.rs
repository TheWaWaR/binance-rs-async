use crate::rest_model::string_or_float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    pub pair: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float")]
    pub estimated_settle_price: f64,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    #[serde(with = "string_or_float")]
    pub interest_rate: f64,
    pub time: u64,
}
