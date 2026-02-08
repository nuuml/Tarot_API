use crate::card;
use crate::card::{Card, };
use rand::{rng};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<card::Card>,
}
impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(78);
        for suit in card::Suit::iter() {
            for rank in card::MinorRank::iter() {
                cards.push(Card::Minor { suit, rank });
            }
        }
        for major in card::MajorArcana::iter() {
            (cards).push(Card::Major(major.clone()));
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}