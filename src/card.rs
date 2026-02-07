use serde::Serialize;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize)]
pub enum Suit {
    Pentacles,
    Swords,
    Cups,
    Wands
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize)]
pub enum MinorRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize)]
pub enum MajorArcana {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Strength,
    TheHermit,
    WheelOfFortune,
    Justice,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgment,
    TheWorld
}

impl MajorArcana {
    pub fn number(self) -> u8 {
        match self {
            MajorArcana::TheFool => 0,
            MajorArcana::TheMagician => 1,
            MajorArcana::TheHighPriestess => 2,
            MajorArcana::TheEmpress => 3,
            MajorArcana::TheEmperor => 4,
            MajorArcana::TheHierophant => 5,
            MajorArcana::TheLovers => 6,
            MajorArcana::TheChariot => 7,
            MajorArcana::Strength => 8,
            MajorArcana::TheHermit => 9,
            MajorArcana::WheelOfFortune => 10,
            MajorArcana::Justice => 11,
            MajorArcana::TheHangedMan => 12,
            MajorArcana::Death => 13,
            MajorArcana::Temperance => 14,
            MajorArcana::TheDevil => 15,
            MajorArcana::TheTower => 16,
            MajorArcana::TheStar => 17,
            MajorArcana::TheMoon => 18,
            MajorArcana::TheSun => 19,
            MajorArcana::Judgment => 20,
            MajorArcana::TheWorld => 21,
        }
    }
}

impl MinorRank {
    pub fn number(self) -> u8 {
        match self {
            MinorRank::Ace => 1,
            MinorRank::Two => 2,
            MinorRank::Three => 3,
            MinorRank::Four => 4,
            MinorRank::Five => 5,
            MinorRank::Six => 6,
            MinorRank::Seven => 7,
            MinorRank::Eight => 8,
            MinorRank::Nine => 9,
            MinorRank::Ten => 10,
            MinorRank::Page => 11,
            MinorRank::Knight => 12,
            MinorRank::Queen => 13,
            MinorRank::King => 14,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize)]
pub enum Card {
    Minor  {
        suit: Suit,
        rank: MinorRank,
    },
    Major(MajorArcana),
}