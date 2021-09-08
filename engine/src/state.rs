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

use crate::cards::Deck;
use std::marker::PhantomData;

struct TexasHoldemHand<S: State> {

    deck: Deck,
    // state: S,
    marker: std::marker::PhantomData<S>
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

impl TexasHoldemHand<Start> {
    fn new() -> TexasHoldemHand<Start> {
        TexasHoldemHand {
            deck: Deck::new(),
            marker: PhantomData::<Start>,
        }
    }

    fn shuffle(&mut self) -> TexasHoldemHand<Shuffled> {
        self.deck.shuffle();

        TexasHoldemHand {
            deck: self.deck,
            marker: PhantomData::<Shuffled>
        }
    }
}



trait Bet {
    fn raise(amount: u8);
    fn call();
    fn check();
    fn fold();
}

#[cfg(test)]
mod tests {
    use crate::state::TexasHoldemHand;

    #[test]
    fn typestate_stuff() {
        let hand = TexasHoldemHand::new().shuffle();
        hand.shuffle();
        hand.shuffle();

    }
}