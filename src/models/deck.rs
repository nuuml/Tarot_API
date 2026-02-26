use crate::models::card::{Card, MajorArcana, MinorRank, Orientation, Suit};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(78);
        for suit in Suit::iter() {
            for rank in MinorRank::iter() {
                cards.push(Card::Minor { suit, rank, orientation: Orientation::Upright });
            }
        }
        for major in MajorArcana::iter() {
            cards.push(Card::Major{ major_arcana: major, orientation: Orientation::Upright });
        }
        Deck { cards }
    }

    pub fn new_major() -> Self {
        let mut cards = Vec::with_capacity(22);
        for major in MajorArcana::iter() {
            cards.push(Card::Major{ major_arcana: major, orientation: Orientation::Upright });
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
        for card in &mut self.cards {
            *card = card.with_random_orientation(&mut rng);
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
