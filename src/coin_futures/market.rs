use crate::client::Client;
use crate::errors::Result;
use crate::futures::rest_model::{FundingRate, HistoryQuery};
use crate::rest_model::{OrderBook, PairQuery};

use super::rest_model::MarkPrice;

#[derive(Clone)]
pub struct CoinFuturesMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl CoinFuturesMarket {
    /// Order book (Default 100; max 1000)
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/dapi/v1/depth", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    pub async fn get_mark_prices(&self, symbol: Option<String>, pair: Option<String>) -> Result<Vec<MarkPrice>> {
        let mut query = serde_json::json!({});
        if let Some(symbol) = symbol {
            query["symbol"] = symbol.into();
        }
        if let Some(pair) = pair {
            query["pair"] = pair.into();
        }
        self.client.get_d("/dapi/v1/premiumIndex", Some(query)).await
    }

    /// Get funding rate history
    pub async fn get_funding_rate<S1, S3, S4, S5>(
        &self,
        symbol: S1,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<FundingRate>>
    where
        S1: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/dapi/v1/fundingRate",
                Some(HistoryQuery {
                    start_time: start_time.into(),
                    end_time: end_time.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    from_id: None,
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
            .await
    }
}
