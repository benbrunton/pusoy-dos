use game::player_move::*;
use cards::card::{ Card, PlayerCard };
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
    let ace_of_spades = card!(Ace, Spades);
    let cards = vec!(ace_of_spades);

    let mv = build_move(cards);
    let single = Some(Move::Single(ace_of_spades.to_card()));

    assert_eq!(single, mv);
}

#[test]
pub fn two_same_cards_are_a_pair(){
    let ace_of_spades = card!(Ace, Spades);
    let ace_of_diamonds = card!(Ace, Diamonds);
    let cards = vec!(ace_of_spades, ace_of_diamonds);

    let mv = build_move(cards);
    let pair = Some(Move::Pair(ace_of_spades.to_card(), ace_of_diamonds.to_card()));

    assert_eq!(pair, mv);
}

#[test]
pub fn two_different_cards_are_invalid(){
    let ace_of_spades = card!(Ace, Spades);
    let two_of_diamonds = card!(Two, Diamonds);
    let cards = vec!(ace_of_spades, two_of_diamonds);

    let mv = build_move(cards);
    let none = None;

    assert_eq!(none, mv);
}

#[test]
pub fn three_of_a_kind_is_a_prial(){
    let ace_of_spades = card!(Ace, Spades);
    let ace_of_diamonds = card!(Ace, Diamonds);
    let ace_of_hearts = card!(Ace, Hearts);

    let cards = vec!(ace_of_spades, ace_of_diamonds, ace_of_hearts);

    let mv = build_move(cards);
    let prial = Some(Move::Prial(ace_of_spades.to_card(),
                                 ace_of_diamonds.to_card(),
                                 ace_of_hearts.to_card()));

    assert_eq!(prial, mv);
}

#[test]
pub fn three_different_cards_is_invalid(){
    let ace_of_spades = card!(Ace, Spades);
    let two_of_spades = card!(Two, Spades);
    let three_of_diamonds = card!(Three, Diamonds);

    let cards = vec!(ace_of_spades, two_of_spades, three_of_diamonds);

    let mv = build_move(cards);

    assert_eq!(mv, None);
}

#[test]
pub fn four_cards_is_not_a_valid_hand(){
    let ace_of_spades = card!(Ace, Spades);
    let ace_of_diamonds = card!(Ace, Diamonds);
    let ace_of_hearts = card!(Ace, Hearts);
    let ace_of_clubs = card!(Ace, Clubs);

    let cards = vec!(ace_of_spades,
                     ace_of_diamonds,
                     ace_of_hearts,
                     ace_of_clubs);
    let mv = build_move(cards);

    assert_eq!(mv, None);
}

#[test]
pub fn four_of_a_kind_is_a_five_card_trick(){
    let ace_of_spades = card!(Ace, Spades);
    let ace_of_diamonds = card!(Ace, Diamonds);
    let ace_of_hearts = card!(Ace, Hearts);
    let ace_of_clubs = card!(Ace, Clubs);
    let two_of_clubs = card!(Two, Clubs);

    let cards = vec!(ace_of_spades,
                     ace_of_diamonds,
                     ace_of_hearts,
                     ace_of_clubs,
                     two_of_clubs);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(
                                        Trick{
                                            trick_type: TrickType::FourOfAKind,
                                            cards: [
                                                   ace_of_clubs.to_card(),
                                                   ace_of_hearts.to_card(),
                                                   ace_of_diamonds.to_card(),
                                                   ace_of_spades.to_card(),
                                                   two_of_clubs.to_card()]}));

    assert_eq!(five_card_trick, mv);
}

#[test]
pub fn five_of_a_kind_is_a_five_card_trick(){
    let ace_of_spades = card!(Ace, Spades);
    let ace_of_diamonds = card!(Ace, Diamonds);
    let ace_of_hearts = card!(Ace, Hearts);
    let ace_of_clubs = card!(Ace, Clubs);

    let cards = vec!(ace_of_spades,
                     ace_of_spades,
                     ace_of_hearts,
                     ace_of_clubs,
                     ace_of_diamonds);

    let mv = build_move(cards);

    let five_card_trick = Some(Move::FiveCardTrick(
                                        Trick{
                                            trick_type: TrickType::FiveOfAKind,
                                            cards:[
                                                   ace_of_clubs.to_card(),
                                                   ace_of_hearts.to_card(),
                                                   ace_of_diamonds.to_card(),
                                                   ace_of_spades.to_card(),
                                                   ace_of_spades.to_card()]}));
    assert_eq!(five_card_trick, mv);
}

#[test]
pub fn five_of_the_same_suit_is_a_flush(){
    let ace_of_spades = card!(Ace, Spades);
    let three_of_spades = card!(Three, Spades);
    let five_of_spades = card!(Five, Spades);
    let seven_of_spades = card!(Seven, Spades);
    let nine_of_spades = card!(Nine, Spades);

    let cards = vec!(ace_of_spades,
                     three_of_spades,
                     five_of_spades,
                     seven_of_spades,
                     nine_of_spades);

    let mv = build_move(cards);

    let five_card_trick = Some(Move::FiveCardTrick(
                                Trick{
                                    trick_type: TrickType::Flush,
                                    cards: [
                                    three_of_spades.to_card(),
                                    five_of_spades.to_card(),
                                    seven_of_spades.to_card(),
                                    nine_of_spades.to_card(),
                                    ace_of_spades.to_card()]}));

    assert_eq!(five_card_trick, mv);

}

#[test]
pub fn a_straight_is_a_five_card_trick(){
    let three_of_clubs = card!(Three, Clubs);
    let four_of_hearts  = card!(Four, Hearts);
    let five_of_spades  = card!(Five, Spades);
    let six_of_spades  = card!(Six, Spades);
    let seven_of_diamonds = card!(Seven, Diamonds);

    let cards = vec!(three_of_clubs,
                     four_of_hearts,
                     five_of_spades,
                     six_of_spades,
                     seven_of_diamonds);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(
                Trick{
                    trick_type: TrickType::Straight,
                    cards: [
                     three_of_clubs.to_card(),
                     four_of_hearts.to_card(),
                     five_of_spades.to_card(),
                     six_of_spades.to_card(),
                     seven_of_diamonds.to_card()]}));

    assert_eq!(mv, five_card_trick);

}

#[test]
pub fn a_straight_flush_is_a_five_card_trick(){
    let three_of_clubs = card!(Three, Clubs);
    let four_of_clubs  = card!(Four, Clubs);
    let five_of_clubs  = card!(Five, Clubs);
    let six_of_clubs  = card!(Six, Clubs);
    let seven_of_clubs = card!(Seven, Clubs);

    let cards = vec!(three_of_clubs,
                     four_of_clubs,
                     five_of_clubs,
                     six_of_clubs,
                     seven_of_clubs);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(
                Trick{
                    trick_type: TrickType::StraightFlush,
                    cards: [
                     three_of_clubs.to_card(),
                     four_of_clubs.to_card(),
                     five_of_clubs.to_card(),
                     six_of_clubs.to_card(),
                     seven_of_clubs.to_card()]}));

    assert_eq!(mv, five_card_trick);

}
#[test]
pub fn full_house_is_a_five_card_trick(){
    let three_of_clubs = card!(Three, Clubs);
    let three_of_hearts  = card!(Three, Hearts);
    let three_of_spades  = card!(Three, Spades);
    let two_of_spades  = card!(Two, Spades);
    let two_of_diamonds = card!(Two, Diamonds);

    let cards = vec!(three_of_clubs,
                     three_of_hearts,
                     three_of_spades,
                     two_of_spades,
                     two_of_diamonds);

    let mv = build_move(cards);
    let five_card_trick = Some(Move::FiveCardTrick(
                Trick{
                    trick_type: TrickType::FullHouse,
                    cards:[
                     three_of_clubs.to_card(),
                     three_of_hearts.to_card(),
                     three_of_spades.to_card(),
                     two_of_diamonds.to_card(),
                     two_of_spades.to_card()]}));

    assert_eq!(mv, five_card_trick);
}

#[test]
pub fn straight_can_be_selected_in_any_order(){

    let five_diamonds = card!(Five, Diamonds);
    let three_clubs = card!(Three, Clubs);
    let four_spades = card!(Four, Spades);
    let seven_clubs = card!(Seven, Clubs);
    let six_hearts = card!(Six, Hearts);

    let mv = build_move(vec!(five_diamonds, three_clubs, four_spades, seven_clubs, six_hearts));

    let five_card_trick = Some(Move::FiveCardTrick(
                                Trick{
                                    trick_type: TrickType::Straight,
                                    cards:[three_clubs.to_card(),
                                    four_spades.to_card(),
                                    five_diamonds.to_card(),
                                    six_hearts.to_card(),
                                    seven_clubs.to_card()]}));

    assert_eq!(mv, five_card_trick);
}

#[test]
pub fn full_house_can_be_in_any_order(){
    let mv = build_move(vec!(
                card!(Five, Clubs),
                card!(Queen, Hearts),
                card!(Queen, Spades),
                card!(Five, Diamonds),
                card!(Queen, Diamonds)));

    let full_house = Some(Move::FiveCardTrick(
                            Trick{
                                trick_type: TrickType::FullHouse,
                                cards: [
                                    card!(Five, Clubs).to_card(),
                                    card!(Five, Diamonds).to_card(),
                                    card!(Queen, Hearts).to_card(),
                                    card!(Queen, Diamonds).to_card(),
                                    card!(Queen, Spades).to_card()
                                ]
                            }));

    assert_eq!(mv, full_house);
}
