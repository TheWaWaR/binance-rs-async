use crate::client::Client;
use crate::errors::Result;
use crate::futures::rest_model::{FundingRate, HistoryQuery, Trades};
use crate::rest_model::{KlineSummaries, KlineSummary, OrderBook, PairQuery};
use crate::util::{to_f64, to_i64};
use serde_json::Value;

use super::rest_model::MarkPrice;

// TODO: merge futures/market.rs AND coin_futures/market.rs

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

    /// Get historical trades
    pub async fn get_historical_trades<S1, S2, S3>(&self, symbol: S1, from_id: S2, limit: S3) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/dapi/v1/historicalTrades",
                Some(HistoryQuery {
                    start_time: None,
                    end_time: None,
                    from_id: from_id.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
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

    /// Returns up to 'limit' premium index klines for given symbol and interval ("1m", "5m", ...)
    /// https://binance-docs.github.io/apidocs/futures/en/#premium-index-kline-data
    pub async fn get_premium_index_klines<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String> + Send,
        S2: Into<String> + Send,
        S3: Into<u16> + Send,
        S4: Into<Option<u64>> + Send,
        S5: Into<Option<u64>> + Send,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            interval: Some(interval.into()),
            from_id: None,
            period: None,
        };
        let data: Vec<Vec<Value>> = self.client.get_d("/dapi/v1/premiumIndexKlines", Some(query)).await?;

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| KlineSummary {
                    open_time: to_i64(&row[0]),
                    open: to_f64(&row[1]),
                    high: to_f64(&row[2]),
                    low: to_f64(&row[3]),
                    close: to_f64(&row[4]),
                    volume: to_f64(&row[5]),
                    close_time: to_i64(&row[6]),
                    quote_asset_volume: to_f64(&row[7]),
                    number_of_trades: to_i64(&row[8]),
                    taker_buy_base_asset_volume: to_f64(&row[9]),
                    taker_buy_quote_asset_volume: to_f64(&row[10]),
                })
                .collect(),
        );
        Ok(klines)
    }
}
