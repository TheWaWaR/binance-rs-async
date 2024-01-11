use crate::client::Client;
use crate::errors::Result;
use crate::rest_model::{OrderBook, PairQuery};

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
}
