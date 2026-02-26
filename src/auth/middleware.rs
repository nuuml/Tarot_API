use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use log::info;
use crate::auth::TokenStore;

pub async fn validate_token(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];

            let token_store = request
                .extensions()
                .get::<TokenStore>()
                .cloned();

            if let Some(store) = token_store {
                if store.is_valid(token) {
                    Ok(next.run(request).await)
                } else {
                    info!("Invalid token provided");
                    Err(StatusCode::UNAUTHORIZED)
                }
            } else {
                info!("Invalid token provided");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => {
            info!("auth: missing or invalid Authorization header");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn generate_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const TOKEN_LEN: usize = 32;
    let mut rng = rand::rng();

    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
