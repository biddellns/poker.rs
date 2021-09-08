use crate::cards::Card;
use crate::table::Errors::PlayerLimitExceeded;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Table<'a> {
    players: Vec<&'a Player<'a>>,
    player_limit: usize,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Table {
            players: Vec::new(),
            player_limit: 10,
        }
    }

    pub fn add_player(&mut self, player: &'a Player<'a>) -> Result<(), Errors> {
        if self.players.len() >= self.player_limit {
            return Err(PlayerLimitExceeded);
        }

        self.players.push(player);
        Ok(())
    }

    pub fn set_player_limit(&mut self, limit: usize) {
        self.player_limit = limit
    }
}

impl<'a> Display for Table<'a> {
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Player<'a> {
    name: &'a str,
    cards: Vec<Card>,
}

impl<'a> Player<'a> {
    pub(crate) fn new(name: &'a str) -> Self {
        Player {
            name,
            cards: Vec::new(),
        }
    }

    pub(crate) fn receive_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl<'a, 'b> Display for Player<'a> {
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
    use crate::cards::Suit::Heart;
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

        let expected_cards = vec![Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        }];

        player.cards = expected_cards.clone();

        assert_eq!(expected_cards, player.cards);

        let expected_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];

        player.cards = expected_cards.clone();

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
        let _ = table.add_player(&expected_player);

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

    #[test]
    fn can_set_tables_player_limit() {
        let mut table = Table::new();

        let expected_player_limit = 5;
        table.set_player_limit(expected_player_limit);

        assert_eq!(expected_player_limit, table.player_limit)
    }

    #[test]
    fn player_receives_dealt_cards() {
        let mut player = Player::new("Nic");
        assert!(player.cards.is_empty());

        let expected_card = Card {
            rank: Rank::King,
            suit: Suit::Club,
        };
        player.receive_card(expected_card);

        assert!(player.cards.contains(&expected_card))
    }
}
