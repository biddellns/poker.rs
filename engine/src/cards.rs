use strum::{EnumIter, IntoEnumIterator};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                cards.push(Card {rank, suit})
            }
        }

        Deck {
            cards
        }
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
    pub(crate) suit: Suit
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
    Ace
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade
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
