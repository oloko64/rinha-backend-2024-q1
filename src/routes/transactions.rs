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
    let conn = &mut *state.db.get_pool().acquire().await?;
    let extrato = state.db.get_extrato(id, conn).await?;
    let extrato_response: ExtratoResponse = extrato.into();

    Ok((StatusCode::OK, Json(extrato_response)).into_response())
}

pub async fn make_transaction(
    Path(id): Path<i64>,
    State(state): State<DbPool>,
    Json(transaction_req): Json<TransactionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_transaction_desc_size(&transaction_req.description)?;

    let conn = &mut *state.db.get_pool().acquire().await?;
    let client = state
        .db
        .get_client(id, conn)
        .await?
        .ok_or(ApiError::not_found())?;

    let new_balance = validate_transaction_balance(
        client.balance,
        transaction_req.amount,
        client.balance_limit,
        &transaction_req.transaction_type,
    )?;

    let client = state
        .db
        .update_client_balance(
            id,
            new_balance,
            transaction_req.amount,
            transaction_req.description,
            transaction_req.transaction_type,
            conn,
        )
        .await?;

    let client_response: ClientResponse = client.into();
    Ok((StatusCode::OK, Json(client_response)).into_response())
}

#[inline]
fn validate_transaction_desc_size(desc: &str) -> Result<bool, ApiError> {
    if (1..=10).contains(&desc.chars().count()) {
        return Ok(true);
    }
    Err(ApiError::unprocessable_entity())
}

#[inline]
fn validate_transaction_balance(
    balance: i64,
    amount: u64,
    balance_limit: i64,
    transaction_type: &TransactionType,
) -> Result<i64, ApiError> {
    match transaction_type {
        TransactionType::Debit => {
            let new_balance = balance - TryInto::<i64>::try_into(amount)?;
            if new_balance < -balance_limit {
                return Err(ApiError::unprocessable_entity());
            }
            Ok(new_balance)
        }
        TransactionType::Credit => Ok(balance + TryInto::<i64>::try_into(amount)?),
    }
}
