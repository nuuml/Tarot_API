mod card;
mod deck;
mod services;
mod auth;
mod admin;

use axum::{
    routing::get,
    routing::post,
    Router,
    middleware,
    Extension,
};
use tokio::net::TcpListener;
use auth::TokenStore;

#[tokio::main]
async fn main() {
    let message = "Welcome to the Tarot API";
    
    let token_store = TokenStore::new();
    
    let protected_routes = Router::new()
        .route("/draw", get(services::draw_card))
        .route("/customDraw", post(services::draw_card_with_options))
        .layer(middleware::from_fn(auth::validate_token))
        .layer(Extension(token_store.clone()));
    
    let public_routes = Router::new()
        .route("/", get(move || async move { message }))
        .route("/admin/token/create", post(admin::create_token))
        .route("/admin/token/revoke", post(admin::revoke_token))
        .with_state(token_store);

    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
