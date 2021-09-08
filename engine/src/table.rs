use crate::cards::Card;
use crate::table::Errors::PlayerLimitExceeded;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Table<'a, 'b> {
    players: Vec<&'a Player<'a, 'b>>,
    player_limit: usize,
}

impl<'a, 'b> Table<'a, 'b> {
    pub fn new() -> Self {
        Table {
            players: Vec::new(),
            player_limit: 10,
        }
    }

    pub fn add_player(&mut self, player: &'a Player<'a, 'b>) -> Result<(), Errors> {
        if self.players.len() >= self.player_limit {
            return Err(PlayerLimitExceeded);
        }

        self.players.push(player);
        Ok(())
    }
}

impl<'a, 'b> Display for Table<'a, 'b> {
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

#[derive(Eq, PartialEq, Debug)]
pub struct Player<'a, 'b> {
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

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    PlayerLimitExceeded,
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Rank, Suit};
    use crate::table::{Errors, Player, Table};

    #[test]
    fn player_display_prints_name() {
        let expected_name = "Nic";
        let player = Player::new(expected_name);

        assert_eq!(expected_name, format!("{}", player))
    }

    #[test]
    fn player_can_be_assigned_cards() {
        let mut player = Player::new("Nic");

        let expected_cards = &[Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        }];

        player.cards = expected_cards;

        assert_eq!(expected_cards, player.cards);

        let expected_cards = &[
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];

        player.cards = expected_cards;

        assert_eq!(expected_cards, player.cards);
    }

    #[test]
    fn can_create_new_game_with_defaults() {
        let expected_table = Table::new();

        assert_eq!(expected_table, Table::new())
    }

    #[test]
    fn can_add_new_player_to_game() {
        let expected_player = Player::new("Nic");

        let mut table = Table::new();
        table.add_player(&expected_player);

        assert!(table.players.contains(&&expected_player))
    }

    #[test]
    fn returns_err_when_adding_player_past_table_capacity() {
        let expected_player = Player::new("Nic");

        let mut table = Table::new();

        table.player_limit = 0;

        let result = table.add_player(&expected_player);

        assert_eq!(result.unwrap_err(), Errors::PlayerLimitExceeded)
    }
}
