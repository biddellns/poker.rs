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

struct Player<'a> {
    name: &'a str,
    cards: Vec<Card>,
}

impl<'a> Player<'a> {
    fn new(name: &'a str) -> Self {
        Player {
            name,
            cards: Vec::new()
        }
    }
}

impl<'a> Display for Player<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, Player};
    use crate::cards::{Card, Rank, Suit};

    #[test]
    fn player_display_prints_name() {
        let expected_name = "Nic";
        let player = Player::new(expected_name);

        assert_eq!(expected_name, format!("{}", player))
    }

    #[test]
    fn player_can_be_assigned_cards() {
        let mut player = Player::new("Nic");

        let expectedCards = vec![Card {rank: Rank::Ace, suit: Suit::Heart}];
        player.cards = expectedCards.clone();

        assert_eq!(expectedCards, player.cards)
    }
}
