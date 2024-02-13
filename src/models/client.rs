use sqlx::{prelude::FromRow, PgPool};

use crate::{
    errors::ApiError,
    models::{TransactionModel, TransactionType},
};

#[derive(Debug, FromRow)]
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
    async fn update_balance(
        &self,
        id: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<ClientModel, ApiError>;
    async fn find_extrato(&self, id: i32) -> Result<ExtratoModel, ApiError>;
}

/// Handles all the operations related to the client
pub struct ClientRepository<'a> {
    pub conn: &'a PgPool,
}

impl<'a> ClientRepository<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        Self { conn }
    }
}

impl Client for ClientRepository<'_> {
    async fn update_balance(
        &self,
        id: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
    ) -> Result<ClientModel, ApiError> {
        let mut transaction = self.conn.begin().await?;
        let client = sqlx::query_as!(
            ClientModel,
            "SELECT * FROM clients WHERE id = $1 FOR UPDATE",
            id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::not_found())?;

        let new_balance = validate_transaction_balance(
            client.balance,
            transaction_amount,
            client.balance_limit,
            &transaction_type,
        )?;

        let transaction_type: &str = transaction_type.into();
        // TODO: Not a good idea to use cast u32 to i32 but for this test context is ok, as all the values are in range of i32
        let transaction_amount = transaction_amount as i32;
        let client = sqlx::query_as!(ClientModel, "WITH updated_transaction AS (INSERT INTO transactions (client_id, amount, description, type) VALUES ($1, $2, $3, $4)) UPDATE clients SET balance = $5 WHERE id = $1 RETURNING *", id, transaction_amount, description, transaction_type, new_balance)
            .fetch_one(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(client)
    }

    async fn find_extrato(&self, id: i32) -> Result<ExtratoModel, ApiError> {
        let mut transaction = self.conn.begin().await?;
        let client = sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", id)
            .fetch_optional(&mut *transaction)
            .await?
            .ok_or(ApiError::not_found())?;

        let transactions = sqlx::query_as!(
            TransactionModel,
            "SELECT * FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10",
            client.id
        )
        .fetch_all(&mut *transaction)
        .await?;

        let res: ExtratoModel = (client, transactions).into();

        Ok(res)
    }
}

#[inline]
fn validate_transaction_balance(
    balance: i32,
    amount: u32,
    balance_limit: i32,
    transaction_type: &TransactionType,
) -> Result<i32, ApiError> {
    match transaction_type {
        TransactionType::Debit => {
            // TODO: Not a good idea to use cast u32 to i32 but for this test context is ok, as all the values are in range of i32
            let new_balance = balance - amount as i32;
            if new_balance < -balance_limit {
                return Err(ApiError::unprocessable_entity());
            }
            Ok(new_balance)
        }
        // TODO: Not a good idea to use cast u32 to i32 but for this test context is ok, as all the values are in range of i32
        TransactionType::Credit => Ok(balance + amount as i32),
    }
}
