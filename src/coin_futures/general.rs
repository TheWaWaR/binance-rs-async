use crate::client::Client;
use crate::errors::Result;
use crate::futures::rest_model::ExchangeInformation;

#[derive(Clone)]
pub struct CoinFuturesGeneral {
    pub client: Client,
}

impl CoinFuturesGeneral {
    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("dapi/v1/exchangeInfo", None).await
    }
}
