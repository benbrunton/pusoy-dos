use cards::types::*;
use cards::card::Card;

#[test]
pub fn card_macro_makes_it_easy_to_declare_card(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);

    assert_eq!(card!(Ace, Spades), ace_of_spades);
}
