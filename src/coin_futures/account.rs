use std::collections::BTreeMap;

use crate::client::Client;
use crate::errors::Result;
use crate::futures::rest_model::AccountBalance;
use crate::util::build_signed_request;

#[derive(Clone)]
pub struct CoinFuturesAccount {
    pub client: Client,
    pub recv_window: u64,
}

impl CoinFuturesAccount {
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::<String, String>::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed_d("/dapi/v1/balance", request.as_str()).await
    }
}
