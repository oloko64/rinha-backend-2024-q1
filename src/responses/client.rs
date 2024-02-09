use crate::models::ClientModel;

#[derive(serde::Serialize, Debug)]
pub struct ClientResponse {
    #[serde(rename = "limite")]
    balance_limit: i32,

    #[serde(rename = "saldo")]
    balance: i32,
}

impl From<ClientModel> for ClientResponse {
    fn from(client: ClientModel) -> Self {
        Self {
            balance_limit: client.balance_limit,
            balance: client.balance,
        }
    }
}
