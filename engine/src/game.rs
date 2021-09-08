use crate::cards::Card;
use std::fmt::{Display, Formatter};

pub struct Game {}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line1 = " -------------------------------------";
        let line2 = "/                                      \\";
        let line3 = "|                                      |";
        let line4 = "|                                      |";
        let line5 = "\\                                     /";
        let line6 = " -------------------------------------";

        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}",
            line1, line2, line3, line4, line5, line6
        )
    }
}

struct Player<'a, 'b> {
    name: &'a str,
    cards: &'b [Card],
}

impl<'a, 'b> Player<'a, 'b> {
    fn new(name: &'a str) -> Self {
        Player { name, cards: &[] }
    }
}

impl<'a, 'b> Display for Player<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Rank, Suit};
    use crate::game::{Game, Player};

    #[test]
    fn player_display_prints_name() {
        let expected_name = "Nic";
        let player = Player::new(expected_name);

        assert_eq!(expected_name, format!("{}", player))
    }

    #[test]
    fn player_can_be_assigned_cards() {
        let mut player = Player::new("Nic");

        let expectedCards = &[Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        }];

        player.cards = expectedCards;

        assert_eq!(expectedCards, player.cards);

        let expectedCards = &[
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];

        player.cards = expectedCards;

        assert_eq!(expectedCards, player.cards);
    }
}
