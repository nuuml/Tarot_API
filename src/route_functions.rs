use axum::Json;
use crate::card::Card;
use crate::deck::Deck;

pub async fn draw_card() -> Json<Option<Card>> {
    let mut deck = Deck::new();
    deck.shuffle();
    Json(deck.draw())
}