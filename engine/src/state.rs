// Texas Hold'em (ignoring all-in)
// 1. Deal 2 cards to err'body
// 2. Bet pre-flop -> If only one left standing: DONE
// 3. Burn and flop
// 4. Bet post-flop -> If only one left standing: DONE
// 5. Burn and river
// 6. Bet post-river -> If only one left standing: DONE
// 7. Burn and turn
// 8. Bet post-turn -> If only one left standing: DONE
// 9. Eval cards for winner
//10. DONE

use std::marker::PhantomData;

use crate::cards::Deck;
use crate::state::Errors::NotEnoughCardsToDeal;
use crate::table::Player;

const CARDS_PER_PLAYER: u32 = 2;

struct TexasHoldemHand<'a, S: State> {
    deck: Deck,
    players: &'a mut [Player<'a>],
    // state: S,
    marker: std::marker::PhantomData<S>,
}

struct Start {}

struct Shuffled {}

struct DealtToPlayers {}

struct DealtFlop {}

struct DealtRiver {}

struct DealTurn {}

struct EvalWinner {}

trait State {}

impl State for Start {}

impl State for Shuffled {}

impl State for DealtToPlayers {}

impl<'a> TexasHoldemHand<'a, Start> {
    pub fn new(players: &'a mut [Player<'a>]) -> TexasHoldemHand<'a, Start> {
        TexasHoldemHand {
            deck: Deck::new(),
            players,
            marker: PhantomData::<Start>,
        }
    }

    fn shuffle(&'a mut self) -> TexasHoldemHand<'a, Shuffled> {
        self.deck.shuffle();

        TexasHoldemHand {
            deck: self.deck.clone(),
            players: self.players,
            marker: PhantomData::<Shuffled>,
        }
    }
}

impl<'a> TexasHoldemHand<'a, Shuffled> {
    fn deal_to_players(&'a mut self) -> Result<TexasHoldemHand<'a, DealtToPlayers>, Errors> {
        for _ in 0..CARDS_PER_PLAYER {
            for player in self.players.iter_mut() {
                match self.deck.draw_card() {
                    Some(c) => player.receive_card(c),
                    None => return Err(NotEnoughCardsToDeal),
                }
            }
        }

        Ok(TexasHoldemHand {
            deck: self.deck.clone(),
            players: self.players,
            marker: PhantomData::<DealtToPlayers>,
        })
    }
}

trait Bet {
    fn raise(amount: u8);
    fn call();
    fn check();
    fn fold();
}

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    NotEnoughCardsToDeal,
}

#[cfg(test)]
mod tests {
    use crate::state::{Shuffled, Start, TexasHoldemHand, CARDS_PER_PLAYER};
    use crate::table::Player;

    #[test]
    fn initial_typestate_only_allows_one_shuffle() {
        let players = &mut [];
        let mut hand = TexasHoldemHand::new(players).shuffle();
        let hand = hand.deal_to_players().unwrap();
        //not a great test, but need to check into testing type IDs
    }
}
