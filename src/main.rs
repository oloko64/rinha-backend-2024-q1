use std::{env, str::FromStr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::{
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Acquire, Sqlite,
};
use tokio::sync::Mutex;

type DbPool = Arc<Mutex<sqlx::SqlitePool>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let conn = SqlitePoolOptions::new()
        .max_connections(100)
        .connect_with(SqliteConnectOptions::from_str(&env::var("DATABASE_URL")?)?)
        .await?;

    // sqlx::migrate!()
    //     .run(&conn)
    //     .await
    //     .expect("Failed to run migrations");

    let state = Arc::new(Mutex::new(conn));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/clientes/:id/transacoes", post(make_transaction))
        .with_state(Arc::clone(&state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn make_transaction(
    Path(id): Path<i64>,
    State(pool): State<DbPool>,
    Json(transaction): Json<Transaction>,
) -> impl IntoResponse {
    let transaction_desc_size = transaction.description.chars().count();
    if transaction_desc_size < 1 || transaction_desc_size > 10 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid description size".to_string(),
        );
    }

    let mut pool = pool.lock().await.acquire().await.unwrap();

    let client = get_client(id, &mut pool).await;

    if let Ok(client) = client {
        let mut balance = client.balance;
        if transaction.transaction_type == TransactionType::Debit {
            balance -= transaction.amount as i64;
        } else {
            balance += transaction.amount as i64;
        }
        if balance < -client.balance_limit {
            return (
                StatusCode::BAD_REQUEST,
                "Balance limit exceeded".to_string(),
            );
        }

        let mut db_transaction = pool.begin().await.unwrap();

        update_client_balance(id, balance, &mut db_transaction)
            .await
            .unwrap();

        insert_transaction(
            id,
            transaction.amount as i64,
            &transaction.description,
            transaction.transaction_type.into(),
            &mut db_transaction,
        )
        .await
        .unwrap();

        let client = get_client(id, &mut db_transaction).await.unwrap();

        db_transaction.commit().await.unwrap();

        return (StatusCode::OK, format!("{:?}", client));
    }

    (
        StatusCode::NOT_FOUND,
        format!("Client with id {} not found", id),
    )
}

async fn get_client(id: i64, pool: &mut PoolConnection<Sqlite>) -> Result<Client, sqlx::Error> {
    sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = ?", id)
        .fetch_one(&mut **pool)
        .await
}

async fn update_client_balance(
    id: i64,
    amount: i64,
    pool: &mut sqlx::Transaction<'_, Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE clients SET balance = ? WHERE id = ?", amount, id)
        .execute(&mut **pool)
        .await?;

    Ok(())
}

async fn insert_transaction(
    client_id: i64,
    amount: i64,
    description: &str,
    transaction_type: &str,
    pool: &mut sqlx::Transaction<'_, Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO transactions (client_id, amount, description, type) VALUES (?, ?, ?, ?)",
        client_id,
        amount,
        description,
        transaction_type
    )
    .execute(&mut **pool)
    .await?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Client {
    #[serde(skip_serializing)]
    id: i64,

    balance_limit: i64,
    balance: i64,
    created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Transaction {
    #[serde(rename = "valor")]
    amount: u64,

    #[serde(rename = "descricao")]
    description: String,

    #[serde(rename = "tipo")]
    transaction_type: TransactionType,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum TransactionType {
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

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(Self::Debit),
            "c" => Ok(Self::Credit),
            _ => Err("Invalid transaction type".to_string()),
        }
    }
}
