use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                cards.push(Card { rank, suit })
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Debug)]
pub struct Card {
    pub(crate) rank: Rank,
    pub(crate) suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Heart => write!(f, "❤"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Club => write!(f, "♣"),
            Suit::Spade => write!(f, "♠"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Deck;

    #[test]
    fn deck_holds_52_cards() {
        assert_eq!(Deck::new().cards.len(), 52)
    }

    #[test]
    fn deck_hold_one_less_after_drawing() {
        let mut deck = Deck::new();
        let original_len = deck.cards.len();

        deck.draw_card();
        assert_eq!(deck.cards.len(), original_len - 1)
    }
}
