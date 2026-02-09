mod card;
mod deck;
mod route_functions;

use deck::Deck;
use card::Card;
use axum::{
    routing::get,
    Json,
    Router
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let message = "Welcome to the Tarot API";

    let app = Router::new()
        .route("/", get(move || async move { message }))
        .route("/draw", get(route_functions::draw_card))
        .route("/customDraw", get(route_functions::draw_card));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
