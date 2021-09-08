use crate::cards::Deck;

mod cards;
mod game;

pub fn testme() {
    let mut deck = Deck::new();
    // // println!("{:?}", deck);
    //
    deck.shuffle();
    // println!("{:?}", deck);

    println!("{}", deck.draw_card().unwrap());

    let game = game::Game::new();
    println!("{}", game)
}
