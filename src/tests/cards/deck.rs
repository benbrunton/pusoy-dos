use cards::deck::*;

#[test]
pub fn it_gets_a_full_deck_including_jokers(){
    let deck = Deck::new();

    assert_eq!(deck.count(), 54);
}

#[test]
pub fn decks_can_be_combined(){

    let deck1 = Deck::new();
    let deck2 = Deck::new();

    let deck3 = Deck::combine(vec!(deck1, deck2));

    assert_eq!(deck3.count(), 108);

}

#[test]
pub fn decks_can_be_dealt(){
    let deck = Deck::new();

    let dealt = deck.deal(4);

    assert_eq!(dealt.len(), 4);
    assert_eq!(dealt[0].len(), 14);
}
