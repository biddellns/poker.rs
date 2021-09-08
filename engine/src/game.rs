use crate::cards::Card;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Game<'a, 'b> {
    players: Vec<&'a Player<'a, 'b>>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        Game { players: Vec::new() }
    }

    pub fn add_player(&mut self, player: &'a Player<'a, 'b>) {
        self.players.push(&player)
    }
}

impl<'a, 'b> Display for Game<'a, 'b> {
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
        let expected_game = Game { players: Vec::new() };

        assert_eq!(expected_game, Game::new())
    }

    #[test]
    fn can_add_new_player_to_game() {
        let expected_player = Player::new("Nic");

        let mut game = Game::new();
        game.add_player(&expected_player);

        assert!(game.players.contains(&&expected_player))
    }
}
