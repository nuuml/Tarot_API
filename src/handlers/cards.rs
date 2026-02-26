use axum::Json;
use serde::Deserialize;
use log::info;
use crate::models::{Card, Deck};

pub async fn draw_card() -> Json<Option<Card>> {
    let mut deck = Deck::new();
    deck.shuffle();
    let drawn = deck.draw();
    info!("method: /draw_card, card_drawn: {:?}", drawn);
    Json(drawn)
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
    let skips = payload.skips.unwrap_or(0);

    let mut output: Vec<Card> = Vec::new();

    for _ in 0..draws {
        for _ in 0..skips {
            deck.draw();
        }
        if let Some(card) = deck.draw() {
            output.push(card);
        } else {
            break;
        }
    }
    info!(
        "method: /draw_card_with_options, options: {:?}, card(s)_drawn: {:?}",
        payload,
        output
    );
    Json(output)
}

#[derive(Deserialize, Debug)]
pub struct DrawOptions {
    number: Option<usize>,
    major_only: bool,
    skips: Option<usize>,
}
