use crate::models::TransactionType;

#[derive(serde::Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "valor")]
    pub amount: u64,

    #[serde(rename = "descricao")]
    pub description: String,

    #[serde(rename = "tipo")]
    pub transaction_type: TransactionType,
}
