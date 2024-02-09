use sqlx::{Acquire, PgConnection};

use crate::{database::Database, errors::ApiError};

use super::{TransactionModel, TransactionType};

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

pub trait ClientRepository {
    async fn get_client(&self, id: i32, conn: &mut PgConnection) -> Result<ClientModel, ApiError>;
    async fn update_client_balance(
        &self,
        id: i32,
        balance: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
        conn: &mut PgConnection,
    ) -> Result<ClientModel, ApiError>;
    async fn get_extrato(&self, id: i32, conn: &mut PgConnection)
        -> Result<ExtratoModel, ApiError>;
}

impl ClientRepository for Database {
    async fn get_client(&self, id: i32, conn: &mut PgConnection) -> Result<ClientModel, ApiError> {
        Ok(
            sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", id)
                .fetch_one(conn)
                .await?,
        )
    }

    async fn update_client_balance(
        &self,
        id: i32,
        balance: i32,
        transaction_amount: u32,
        description: String,
        transaction_type: TransactionType,
        conn: &mut PgConnection,
    ) -> Result<ClientModel, ApiError> {
        let mut transaction = conn.begin().await?;
        sqlx::query!("UPDATE clients SET balance = $1 WHERE id = $2", balance, id)
            .execute(&mut *transaction)
            .await?;

        let transaction_type: &str = transaction_type.into();
        // TODO: Not a good idea to use cast u32 to i32 but for this test context is ok, as all the values are in range of i32
        let transaction_amount = transaction_amount as i32;
        sqlx::query!(
                "INSERT INTO transactions (client_id, amount, description, type) VALUES ($1, $2, $3, $4)",
                id,
                transaction_amount,
                description,
                transaction_type
            )
            .execute(&mut *transaction)
            .await?;

        let client = self.get_client(id, &mut transaction).await?;

        transaction.commit().await?;

        Ok(client)
    }

    async fn get_extrato(
        &self,
        id: i32,
        conn: &mut PgConnection,
    ) -> Result<ExtratoModel, ApiError> {
        let client = self.get_client(id, conn).await?;

        let transactions = sqlx::query_as!(
            TransactionModel,
            "SELECT * FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10",
            client.id
        )
        .fetch_all(conn)
        .await?;

        let res: ExtratoModel = (client, transactions).into();

        Ok(res)
    }
}
