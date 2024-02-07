use crate::{database::Database, errors::ApiError};

use super::{TransactionModel, TransactionType};

#[derive(Debug)]
pub struct ClientModel {
    pub id: i64,
    pub balance_limit: i64,
    pub balance: i64,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct ExtratoModel {
    pub saldo: SaldoModel,
    pub transactions: Vec<TransactionModel>,
}

impl From<(ClientModel, Vec<TransactionModel>)> for ExtratoModel {
    fn from((client, transactions): (ClientModel, Vec<TransactionModel>)) -> Self {
        let saldo = SaldoModel {
            balance: client.balance,
            balance_limit: client.balance_limit,
            date: client.created_at,
        };

        Self {
            saldo,
            transactions,
        }
    }
}

#[derive(Debug)]
pub struct SaldoModel {
    pub balance: i64,
    pub balance_limit: i64,
    pub date: sqlx::types::chrono::NaiveDateTime,
}

pub trait ClientRepository {
    async fn get_client(&self, id: i64) -> Result<Option<ClientModel>, ApiError>;
    async fn update_client_balance(
        &self,
        id: i64,
        balance: i64,
        transaction_amount: u64,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<Option<ClientModel>, ApiError>;
    async fn get_extrato(&self, client: ClientModel) -> Result<ExtratoModel, ApiError>;
}

impl ClientRepository for Database {
    async fn get_client(&self, id: i64) -> Result<Option<ClientModel>, ApiError> {
        let mut conn = self.get_pool().acquire().await?;

        Ok(
            sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", id)
                .fetch_optional(&mut *conn)
                .await?,
        )
    }

    async fn update_client_balance(
        &self,
        id: i64,
        balance: i64,
        transaction_amount: u64,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<Option<ClientModel>, ApiError> {
        let mut transaction = self.get_pool().begin().await?;

        sqlx::query!("UPDATE clients SET balance = $1 WHERE id = $2", balance, id)
            .execute(&mut *transaction)
            .await?;

        let transaction_type: &str = transaction_type.into();
        let transaction_amount: i64 = transaction_amount.try_into()?;
        sqlx::query!(
            "INSERT INTO transactions (client_id, amount, description, type) VALUES ($1, $2, $3, $4)",
            id,
            transaction_amount,
            description,
            transaction_type
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        let client = self.get_client(id).await?;

        Ok(client)
    }

    async fn get_extrato(&self, client: ClientModel) -> Result<ExtratoModel, ApiError> {
        let mut conn = self.get_pool().acquire().await?;
        let transactions = sqlx::query_as!(
            TransactionModel,
            "SELECT * FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10",
            client.id
        )
        .fetch_all(&mut *conn)
        .await?;

        let res: ExtratoModel = (client, transactions).into();

        Ok(res)
    }
}
