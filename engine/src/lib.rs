use crate::cards::{Deck};

mod cards;

pub fn testme() {
    let mut deck = Deck::new();
    println!("{:?}", deck);

    deck.shuffle();
    println!("{:?}", deck);
}
