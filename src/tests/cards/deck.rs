
use cards::deck::*;

#[test]
pub fn it_gets_a_full_deck(){
    let deck = Deck::new(); 

    assert_eq!(deck.count(), 52);
}

