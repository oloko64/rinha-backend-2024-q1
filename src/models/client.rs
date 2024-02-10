use sqlx::PgConnection;

use crate::{
    errors::ApiError,
    models::{TransactionModel, TransactionType},
};

#[derive(Debug)]
pub struct ClientModel {
    pub id: i32,
    pub balance_limit: i32,
    pub balance: i32,
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
    pub balance: i32,
    pub balance_limit: i32,
    pub date: sqlx::types::chrono::NaiveDateTime,
}

pub trait Client {
    async fn find_client(&mut self, id: i32) -> Result<Option<ClientModel>, ApiError>;
    async fn update_balance(
        &mut self,
        id: i32,
        balance: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<ClientModel, ApiError>;
    async fn find_extrato(&mut self, id: i32) -> Result<Option<ExtratoModel>, ApiError>;
}

/// Handles all the operations related to the client
pub struct ClientRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> ClientRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl Client for ClientRepository<'_> {
    async fn find_client(&mut self, id: i32) -> Result<Option<ClientModel>, ApiError> {
        Ok(
            sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", id)
                .fetch_optional(&mut *self.conn)
                .await?,
        )
    }

    async fn update_balance(
        &mut self,
        id: i32,
        balance: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<ClientModel, ApiError> {
        let transaction_type: &str = transaction_type.into();
        // TODO: Not a good idea to use cast u32 to i32 but for this test context is ok, as all the values are in range of i32
        let transaction_amount = transaction_amount as i32;
        let client = sqlx::query_as!(ClientModel, "WITH updated_transaction AS (INSERT INTO transactions (client_id, amount, description, type) VALUES ($1, $2, $3, $4)) UPDATE clients SET balance = $5 WHERE id = $1 RETURNING *", id, transaction_amount, description, transaction_type, balance)
            .fetch_one(&mut *self.conn)
            .await?;

        Ok(client)
    }

    async fn find_extrato(&mut self, id: i32) -> Result<Option<ExtratoModel>, ApiError> {
        let client = self.find_client(id).await?;

        if let Some(client) = client {
            let transactions = sqlx::query_as!(
                TransactionModel,
                "SELECT * FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10",
                client.id
            )
            .fetch_all(&mut *self.conn)
            .await?;

            let res: ExtratoModel = (client, transactions).into();

            return Ok(Some(res));
        }

        Ok(None)
    }
}
