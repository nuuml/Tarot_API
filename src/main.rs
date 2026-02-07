mod card;
mod deck;

use deck::Deck;
use card::Card;
use axum::{
    routing::get,
    extract::State,
    Json,
    Router
};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() {
    let deck = Arc::new(Mutex::new(Deck::new()));

    let message = "bitch, please";

    let app = Router::new()
        .route("/", get(move || async move { message }))
        .route("/draw", get(draw_card))
        .with_state(deck);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn draw_card(
    State(deck): State<Arc<Mutex<Deck>>>,
) -> Json<Option<Card>> {
    let mut deck = deck.lock().unwrap();
    deck.shuffle();
    Json(deck.draw())
}