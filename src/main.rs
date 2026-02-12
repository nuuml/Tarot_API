mod card;
mod deck;
mod services;

use deck::Deck;
use card::Card;
use axum::{
    routing::get,
    routing::post,
    Json,
    Router
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let message = "Welcome to the Tarot API";

    let app = Router::new()
        .route("/", get(move || async move { message }))
        .route("/draw", get(services::draw_card))
        .route("/customDraw", post(services::draw_card_with_options));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
