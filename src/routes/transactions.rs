use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    errors::ApiError,
    models::{Client, ClientRepository},
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
    let extrato = client_repository.find_extrato(id).await?;
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
        .update_balance(
            id,
            transaction_req.amount,
            transaction_req.description,
            transaction_req.transaction_type,
        )
        .await?;

    let client: ClientResponse = client.into();

    Ok((StatusCode::OK, Json(client)).into_response())
}

#[inline]
fn validate_transaction_desc_size(desc: &str) -> Result<bool, ApiError> {
    if (1..11).contains(&desc.len()) {
        return Ok(true);
    }
    Err(ApiError::unprocessable_entity())
}
