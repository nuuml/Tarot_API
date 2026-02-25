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
use env_logger;
use log::{error, info};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    let message = "Welcome to the Tarot API";
    info!("{}", message);
    dotenv().ok();


    let token_store_path = env::var("TOKEN_STORE_PATH")
        .unwrap_or_else(|_| "data/tokens.json".to_string());
    let token_store = match TokenStore::new(token_store_path.into()) {
        Ok(store) => store,
        Err(err) => {
            error!("Failed to initialize token store: {err}");
            std::process::exit(1);
        }
    };

    
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
