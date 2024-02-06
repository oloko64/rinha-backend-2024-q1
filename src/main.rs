mod database;
mod error;
mod models;
mod responses;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use database::{AppState, Database};
use error::ApiError;
use models::{ClientRepository, TransactionRequest, TransactionType};
use responses::{ClientResponse, ExtratoResponse};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{env, str::FromStr, sync::Arc};

type DbPool = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect_with(SqliteConnectOptions::from_str(&env::var("DATABASE_URL")?)?)
        .await?;

    // sqlx::migrate!()
    //     .run(&conn)
    //     .await
    //     .expect("Failed to run migrations");

    let state = Arc::new(AppState {
        db: Database::new(pool),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/clientes/:id/transacoes", post(make_transaction))
        .route("/clientes/:id/extrato", get(get_transactions))
        .with_state(Arc::clone(&state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_transactions(
    Path(id): Path<i64>,
    State(state): State<DbPool>,
) -> Result<impl IntoResponse, ApiError> {
    let client = state.db.get_client(id).await?;

    if let Some(client) = client {
        let extrato = state.db.get_extrato(client).await?;
        let extrato_response: ExtratoResponse = extrato.into();
        return Ok((StatusCode::OK, Json(extrato_response)).into_response());
    }

    Err(ApiError::not_found(format!(
        "Client with id {id} not found"
    )))
}

async fn make_transaction(
    Path(id): Path<i64>,
    State(state): State<DbPool>,
    Json(transaction_req): Json<TransactionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let transaction_desc_size = transaction_req.description.chars().count();
    if !(1..=10).contains(&transaction_desc_size) {
        return Err(ApiError::bad_request("Invalid description size"));
    }

    let client = state.db.get_client(id).await?;

    if let Some(client) = client {
        let mut balance = client.balance;
        match transaction_req.transaction_type {
            TransactionType::Debit => {
                balance -= transaction_req.amount as i64;
            }
            TransactionType::Credit => {
                balance += transaction_req.amount as i64;
            }
        }
        if balance < -client.balance_limit {
            return Err(ApiError::bad_request("Balance limit exceeded"));
        }

        let client = state
            .db
            .update_client_balance(
                id,
                balance,
                transaction_req.amount,
                transaction_req.description,
                transaction_req.transaction_type,
            )
            .await?;

        if let Some(client) = client {
            let client_response: ClientResponse = client.into();
            return Ok((StatusCode::OK, Json(client_response)).into_response());
        }
        return Err(ApiError::internal_server_error("Failed to update client"));
    }

    Err(ApiError::not_found(format!(
        "Client with id {id} not found"
    )))
}
