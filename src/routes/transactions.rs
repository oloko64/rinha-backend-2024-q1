use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    errors::ApiError,
    models::{Client, ClientRepository, TransactionType},
    requests::TransactionRequest,
    responses::{ClientResponse, ExtratoResponse},
    DbPool,
};

pub async fn get_transactions(
    Path(id): Path<i32>,
    State(state): State<DbPool>,
) -> Result<impl IntoResponse, ApiError> {
    let conn = state.db.get_pool();
    let client_repository = ClientRepository::new(conn);
    let extrato = client_repository
        .find_extrato(id)
        .await?
        .ok_or(ApiError::not_found())?;
    let extrato_response: ExtratoResponse = extrato.into();

    Ok((StatusCode::OK, Json(extrato_response)).into_response())
}

pub async fn make_transaction(
    Path(id): Path<i32>,
    State(state): State<DbPool>,
    Json(transaction_req): Json<TransactionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_transaction_desc_size(&transaction_req.description)?;

    let conn = state.db.get_pool();
    let client_repository = ClientRepository::new(conn);
    let client = client_repository
        .find_client(id)
        .await?
        .ok_or(ApiError::not_found())?;

    let new_balance = validate_transaction_balance(
        client.balance,
        transaction_req.amount,
        client.balance_limit,
        &transaction_req.transaction_type,
    )?;

    let client = client_repository
        .update_balance(
            id,
            new_balance,
            transaction_req.amount,
            transaction_req.description,
            transaction_req.transaction_type,
        )
        .await?;

    let client_response: ClientResponse = client.into();
    Ok((StatusCode::OK, Json(client_response)).into_response())
}

#[inline]
fn validate_transaction_desc_size(desc: &str) -> Result<bool, ApiError> {
    if (1..11).contains(&desc.len()) {
        return Ok(true);
    }
    Err(ApiError::unprocessable_entity())
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
