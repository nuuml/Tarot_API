use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::auth::{TokenStore, generate_token};

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
) -> Json<TokenResponse> {
    let token = generate_token();
    store.add_token(token.clone());
    Json(TokenResponse { token })
}

pub async fn revoke_token(
    State(store): State<TokenStore>,
    Json(payload): Json<RevokeRequest>,
) -> (StatusCode, Json<RevokeResponse>) {
    let success = store.remove_token(&payload.token);
    (
        StatusCode::OK,
        Json(RevokeResponse { success })
    )
}
