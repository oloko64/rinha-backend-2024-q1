#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
pub enum TransactionType {
    #[serde(rename = "d")]
    Debit,

    #[serde(rename = "c")]
    Credit,
}

impl From<TransactionType> for &'static str {
    fn from(t: TransactionType) -> &'static str {
        match t {
            TransactionType::Debit => "d",
            TransactionType::Credit => "c",
        }
    }
}

#[derive(Debug)]
pub struct TransactionModel {
    pub id: i64,
    pub client_id: i64,
    pub amount: i64,
    pub description: String,
    pub r#type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}
