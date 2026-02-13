use axum::{ Json};
use crate::card::Card;
use serde::Deserialize;
use crate::deck::Deck;

pub async fn draw_card() -> Json<Option<Card>> {
    let mut deck = Deck::new();
    deck.shuffle();
    Json(deck.draw())
}

pub async fn draw_card_with_options(
    Json(payload): Json<DrawOptions>,
) -> Json<Vec<Card>> {
    let mut deck = if payload.major_only {
        Deck::new_major()
    } else {
        Deck::new()
    };

    deck.shuffle();

    let draws = payload.number.unwrap_or(1);

    let mut output: Vec<Card> = Vec::new();

    for _ in 0..draws {
        if let Some(card) = deck.draw() {
            output.push(card);
        } else {
            break;
        }
    }
    Json(output)
}


#[derive(Deserialize)]
pub struct DrawOptions {
    number: Option<usize>,
    major_only: bool,
}
