use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::auth::{TokenStore, generate_token};
use log::error;

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RevokeRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct RevokeResponse {
    pub success: bool,
}

pub async fn create_token(
    State(store): State<TokenStore>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let token = generate_token();
    store.add_token(token.clone()).map_err(|err| {
        error!("Failed to persist token: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(TokenResponse { token }))
}

pub async fn revoke_token(
    State(store): State<TokenStore>,
    Json(payload): Json<RevokeRequest>,
) -> Result<(StatusCode, Json<RevokeResponse>), StatusCode> {
    let success = store.remove_token(&payload.token).map_err(|err| {
        error!("Failed to persist token revocation: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((
        StatusCode::OK,
        Json(RevokeResponse { success })
    ))
}
