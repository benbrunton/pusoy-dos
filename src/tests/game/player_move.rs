use game::player_move::*;
use cards::card::Card;
use cards::types::*;

#[test]
pub fn passing_no_cards_is_a_pass(){

    let cards = vec!();
    let mv = build_move(cards);

    let pass = Some(Move::Pass);

    assert_eq!(pass, mv);
}

#[test]
pub fn passing_a_single_is_a_single(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let cards = vec!(ace_of_spades);

    let mv = build_move(cards);
    let single = Some(Move::Single(ace_of_spades));

    assert_eq!(single, mv);
}

#[test]
pub fn two_same_cards_are_a_pair(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);
    let cards = vec!(ace_of_spades, ace_of_diamonds);

    let mv = build_move(cards);
    let pair = Some(Move::Pair(ace_of_spades, ace_of_diamonds));

    assert_eq!(pair, mv);
}

#[test]
pub fn two_different_cards_are_invalid(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let two_of_diamonds = Card::new(Rank::Two, Suit::Diamonds);
    let cards = vec!(ace_of_spades, two_of_diamonds);

    let mv = build_move(cards);
    let none = None;

    assert_eq!(none, mv);
}

#[test]
pub fn three_of_a_kind_is_a_prial(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);
    let ace_of_hearts = Card::new(Rank::Ace, Suit::Hearts);

    let cards = vec!(ace_of_spades, ace_of_diamonds, ace_of_hearts);

    let mv = build_move(cards);
    let prial = Some(Move::Prial(ace_of_spades, 
                                 ace_of_diamonds, 
                                 ace_of_hearts));

    assert_eq!(prial, mv);
}

#[test]
pub fn three_different_cards_is_invalid(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let two_of_spades = Card::new(Rank::Two, Suit::Spades);
    let three_of_diamonds = Card::new(Rank::Three, Suit::Diamonds);

    let cards = vec!(ace_of_spades, two_of_spades, three_of_diamonds);

    let mv = build_move(cards);

    assert_eq!(mv, None);
}

#[test]
pub fn four_cards_is_not_a_valid_hand(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);
    let ace_of_hearts = Card::new(Rank::Ace, Suit::Hearts);
    let ace_of_clubs = Card::new(Rank::Ace, Suit::Clubs);

    let cards = vec!(ace_of_spades,
                     ace_of_diamonds,
                     ace_of_hearts,
                     ace_of_clubs);
    let mv = build_move(cards);

    assert_eq!(mv, None);
}

#[test]
pub fn four_of_a_kind_is_a_five_card_trick(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);
    let ace_of_hearts = Card::new(Rank::Ace, Suit::Hearts);
    let ace_of_clubs = Card::new(Rank::Ace, Suit::Clubs);
    let two_of_clubs = Card::new(Rank::Two, Suit::Clubs);

    let cards = vec!(ace_of_spades,
                     ace_of_diamonds,
                     ace_of_hearts,
                     ace_of_clubs,
                     two_of_clubs);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(ace_of_spades,
                                                   ace_of_diamonds,
                                                   ace_of_hearts,
                                                   ace_of_clubs,
                                                   two_of_clubs));

    assert_eq!(five_card_trick, mv);
}

#[test]
pub fn five_of_a_kind_is_a_five_card_trick(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);
    let ace_of_hearts = Card::new(Rank::Ace, Suit::Hearts);
    let ace_of_clubs = Card::new(Rank::Ace, Suit::Clubs);
    
    let cards = vec!(ace_of_spades,
                     ace_of_spades,
                     ace_of_hearts,
                     ace_of_clubs,
                     ace_of_diamonds);

    let mv = build_move(cards);

    let five_card_trick = Some(Move::FiveCardTrick(ace_of_spades,
                                                   ace_of_spades,
                                                   ace_of_hearts,
                                                   ace_of_clubs,
                                                   ace_of_diamonds));
    assert_eq!(five_card_trick, mv);
}

#[test]
pub fn five_of_the_same_suit_is_a_flush(){
    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let three_of_spades = Card::new(Rank::Three, Suit::Spades);
    let five_of_spades = Card::new(Rank::Five, Suit::Spades);
    let seven_of_spades = Card::new(Rank::Seven, Suit::Spades);
    let nine_of_spades = Card::new(Rank::Nine, Suit::Spades);

    let cards = vec!(ace_of_spades,
                     three_of_spades,
                     five_of_spades,
                     seven_of_spades,
                     nine_of_spades);

    let mv = build_move(cards);

    let five_card_trick = Some(Move::FiveCardTrick(ace_of_spades,
                                                    three_of_spades,
                                                    five_of_spades,
                                                    seven_of_spades,
                                                    nine_of_spades));

    assert_eq!(five_card_trick, mv);

}

#[test]
pub fn a_straight_is_a_five_card_trick(){
    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    let four_of_hearts  = Card::new(Rank::Four, Suit::Hearts);
    let five_of_spades  = Card::new(Rank::Five, Suit::Spades);
    let six_of_spades  = Card::new(Rank::Six, Suit::Spades);
    let seven_of_diamonds = Card::new(Rank::Seven, Suit::Diamonds);

    let cards = vec!(three_of_clubs,
                     four_of_hearts,
                     five_of_spades,
                     six_of_spades,
                     seven_of_diamonds);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(three_of_clubs,
                     four_of_hearts,
                     five_of_spades,
                     six_of_spades,
                     seven_of_diamonds));

    assert_eq!(mv, five_card_trick);

}
