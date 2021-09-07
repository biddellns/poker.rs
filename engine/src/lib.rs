use crate::cards::{Deck};

mod cards;
mod game;

pub fn testme() {
    // let mut deck = Deck::new();
    // println!("{:?}", deck);
    //
    // deck.shuffle();
    // println!("{:?}", deck);

    let game = game::Game{};
    println!("{}", game)
}