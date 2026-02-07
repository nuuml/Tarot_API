use crate::card;
use crate::card::{Card, };
use rand::{rng};
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<card::Card>,
}
impl Deck {
    pub fn new() -> Self {
        let suits = [
            card::Suit::Pentacles,
            card::Suit::Swords,
            card::Suit::Cups,
            card::Suit::Wands,
        ];

        let ranks = [
            card::MinorRank::Ace,
            card::MinorRank::Two,
            card::MinorRank::Three,
            card::MinorRank::Four,
            card::MinorRank::Five,
            card::MinorRank::Six,
            card::MinorRank::Seven,
            card::MinorRank::Eight,
            card::MinorRank::Nine,
            card::MinorRank::Ten,
            card::MinorRank::Page,
            card::MinorRank::Knight,
            card::MinorRank::Queen,
            card::MinorRank::King
        ];

        let majors = [
            card::MajorArcana::TheFool,
            card::MajorArcana::TheMagician,
            card::MajorArcana::TheHighPriestess,
            card::MajorArcana::TheEmpress,
            card::MajorArcana::TheEmperor,
            card::MajorArcana::TheHierophant,
            card::MajorArcana::TheLovers,
            card::MajorArcana::TheChariot,
            card::MajorArcana::Strength,
            card::MajorArcana::TheHermit,
            card::MajorArcana::WheelOfFortune,
            card::MajorArcana::Justice,
            card::MajorArcana::TheHangedMan,
            card::MajorArcana::Death,
            card::MajorArcana::Temperance,
            card::MajorArcana::TheDevil,
            card::MajorArcana::TheTower,
            card::MajorArcana::TheStar,
            card::MajorArcana::TheMoon,
            card::MajorArcana::TheSun,
            card::MajorArcana::Judgment,
            card::MajorArcana::TheWorld,
        ];

        let mut cards = Vec::with_capacity(78);
        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(Card::Minor { suit: *suit, rank: *rank });
            }
        }
        for major in majors.iter() {
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