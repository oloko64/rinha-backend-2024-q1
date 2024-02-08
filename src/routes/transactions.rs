use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    errors::ApiError,
    models::{ClientRepository, TransactionType},
    requests::TransactionRequest,
    responses::{ClientResponse, ExtratoResponse},
    DbPool,
};

pub async fn get_transactions(
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

pub async fn make_transaction(
    Path(id): Path<i64>,
    State(state): State<DbPool>,
    Json(transaction_req): Json<TransactionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let transaction_desc_size = transaction_req.description.chars().count();
    if !(1..=10).contains(&transaction_desc_size) {
        return Err(ApiError::unprocessable_entity("Invalid description size"));
    }

    let client = state.db.get_client(id).await?;

    if let Some(client) = client {
        let mut balance = client.balance;
        match transaction_req.transaction_type {
            TransactionType::Debit => {
                balance -= TryInto::<i64>::try_into(transaction_req.amount)?;
            }
            TransactionType::Credit => {
                balance += TryInto::<i64>::try_into(transaction_req.amount)?;
            }
        }
        if balance < -client.balance_limit {
            return Err(ApiError::unprocessable_entity("Balance limit exceeded"));
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
