use cards::types::*;
use cards::card::{ PlayerCard, Card };

#[test]
pub fn card_macro_makes_it_easy_to_declare_card(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades, false);

    assert_eq!(card!(Ace, Spades).to_card(), ace_of_spades);
}


#[test]
pub fn rank_order_is_correct() {
    let three = Card::new(Rank::Three, Suit::Spades, false);
    let four = Card::new(Rank::Four, Suit::Spades, false);
    let five = Card::new(Rank::Five, Suit::Spades, false);
    let six = Card::new(Rank::Six, Suit::Spades, false);
    let seven = Card::new(Rank::Seven, Suit::Spades, false);
    let eight = Card::new(Rank::Eight, Suit::Spades, false);
    let nine = Card::new(Rank::Nine, Suit::Spades, false);
    let ten = Card::new(Rank::Ten, Suit::Spades, false);
    let jack = Card::new(Rank::Jack, Suit::Spades, false);
    let queen = Card::new(Rank::Queen, Suit::Spades, false);
    let king = Card::new(Rank::King, Suit::Spades, false);
    let ace = Card::new(Rank::Ace, Suit::Spades, false);
    let two = Card::new(Rank::Two, Suit::Spades, false);

    assert_eq!(three.previous_rank(), None);
    assert_eq!(three.next_rank(), Some(Rank::Four));
    assert_eq!(four.previous_rank(), Some(Rank::Three));
    assert_eq!(four.next_rank(), Some(Rank::Five));
    assert_eq!(five.previous_rank(), Some(Rank::Four));
    assert_eq!(five.next_rank(), Some(Rank::Six));
    assert_eq!(six.previous_rank(), Some(Rank::Five));
    assert_eq!(six.next_rank(), Some(Rank::Seven));
    assert_eq!(seven.previous_rank(), Some(Rank::Six));
    assert_eq!(seven.next_rank(), Some(Rank::Eight));
    assert_eq!(eight.previous_rank(), Some(Rank::Seven));
    assert_eq!(eight.next_rank(), Some(Rank::Nine));
    assert_eq!(nine.previous_rank(), Some(Rank::Eight));
    assert_eq!(nine.next_rank(), Some(Rank::Ten));
    assert_eq!(ten.previous_rank(), Some(Rank::Nine));
    assert_eq!(ten.next_rank(), Some(Rank::Jack));
    assert_eq!(jack.previous_rank(), Some(Rank::Ten));
    assert_eq!(jack.next_rank(), Some(Rank::Queen));
    assert_eq!(queen.previous_rank(), Some(Rank::Jack));
    assert_eq!(queen.next_rank(), Some(Rank::King));
    assert_eq!(king.previous_rank(), Some(Rank::Queen));
    assert_eq!(king.next_rank(), Some(Rank::Ace));
    assert_eq!(ace.previous_rank(), Some(Rank::King));
    assert_eq!(ace.next_rank(), Some(Rank::Two));
    assert_eq!(two.previous_rank(), Some(Rank::Ace));
    assert_eq!(two.next_rank(), None);
}


