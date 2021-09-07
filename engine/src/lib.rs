use crate::cards::{Deck};

mod cards;

pub fn testme() {
    let deck = Deck::new();

    println!("{:?}", deck)

}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
