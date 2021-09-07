use strum::{EnumIter, IntoEnumIterator};

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
}
