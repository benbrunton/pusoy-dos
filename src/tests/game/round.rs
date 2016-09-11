use game::round::Round;
use game::player_move::{ Move, build_move };
use cards::card::Card;
use cards::types::*;

#[test]
pub fn it_returns_the_current_player_id(){
    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass);

    assert_eq!(r.get_next_player(), 1);
}

#[test]
pub fn it_returns_a_new_round_when_passed_a_valid_move(){
    // Pass is always a valid move

    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass);

    let new_round = match r.play(1, Move::Pass) {
        Ok(r) => r,
        Err(r) => r
    };

    assert!(new_round != r);
}

#[test]
pub fn it_returns_an_error_when_a_move_is_invalid(){

    let mv = Move::Pair(
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Ace, Suit::Hearts)
    );

    let new_move = Move::Single(
        Card::new(Rank::Three, Suit::Clubs)
    );
    let r = Round::new(vec!(1, 2, 3), 1, mv);

    let new_round = r.play(1, new_move);

    assert_eq!(new_round, Err(r));
}

#[test]
pub fn a_valid_move_rotates_the_players(){

    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass);

    let new_round = match r.play(1, Move::Pass) {
        Ok(r) => r,
        Err(r) => r
    };

    assert_eq!(new_round.get_next_player(), 2);

}

#[test]
pub fn rotating_the_player_will_bring_it_back_to_beginning_of_vec(){

    let r = Round::new(vec!(8, 15, 3), 3, Move::Pass);

    let new_round = match r.play(3, Move::Pass) {
        Ok(r) => r,
        Err(r) => r
    };

    assert_eq!(new_round.get_next_player(), 8);

    let new_round = match new_round.play(8, Move::Pass){
        Ok(r) => r,
        Err(r) => r
    };

    let new_round = match new_round.play(15, Move::Pass){
        Ok(r) => r,
        Err(r) => r
    };

    assert_eq!(new_round.get_next_player(), 3);

}

#[test]
#[should_panic]
pub fn creating_a_round_with_an_invalid_current_player_causes_panic(){
   Round::new(vec!(1, 2, 3), 4, Move::Pass);
}

#[test]
pub fn only_the_current_player_can_make_a_move(){
    let r = Round::new(vec!(1, 2), 2, Move::Pass);

    let invalid_move = match r.play(1, Move::Pass) {
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn any_hand_can_be_passed_onto_an_emtpy_round(){

    let r = Round::new(vec!(1, 2), 1, Move::Pass);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);

    let valid_move = match r.play(1, Move::Single(ace_of_spades)) {
        Ok(_)   => true,
        _       => false
    };

    assert!(valid_move);

    let valid_move = match r.play(1, Move::Pair(ace_of_spades, ace_of_diamonds)) {
       Ok(_)   => true,
        _       => false

    };

    assert!(valid_move);

}

#[test]
pub fn single_can_be_beaten_by_a_higher_single(){

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    let four_of_diamonds = Card::new(Rank::Four, Suit::Diamonds);

    let r = Round::new(vec!(1, 2), 1, Move::Single(three_of_clubs));

    let valid_move = match r.play(1, Move::Single(four_of_diamonds)) {
        Ok(_)   => true,
        _       => false
    };

    assert!(valid_move);
}

#[test]
pub fn single_cannot_be_beaten_by_a_higher_single(){

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    let four_of_diamonds = Card::new(Rank::Four, Suit::Diamonds);

    let r = Round::new(vec!(1, 2), 1, Move::Single(four_of_diamonds));

    let invalid_move = match r.play(1, Move::Single(three_of_clubs)) {
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn single_card_respects_suit_order() {
    let tc = Card::new(Rank::Three, Suit::Clubs);
    let th = Card::new(Rank::Three, Suit::Hearts);
    let td = Card::new(Rank::Three, Suit::Diamonds);

    let r = Round::new(vec!(1,2,3), 1, Move::Single(th));

    let invalid_move = match r.play(1, Move::Single(tc)) {
         Err(_) => true,
         _      => false
    };

    assert!(invalid_move);

    let valid_move = match r.play(1, Move::Single(td)) {
         Err(_) => false,
         _      => true
    };

    assert!(valid_move);
}

#[test]
pub fn single_cannot_be_beaten_by_non_single_move(){

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds);

    let r = Round::new(vec!(1, 2), 1, Move::Single(three_of_clubs));

    let invalid_move = match r.play(1, Move::Pair(ace_of_spades, ace_of_diamonds)){
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn when_every_player_passes_the_last_player_to_move_starts_the_round(){

    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    let two_of_hearts = Card::new(Rank::Two, Suit::Hearts);

    let next_round = match r.play(1, Move::Single(ace_of_spades)){
        Ok(r)  => r,
        Err(r) => r
    };

    let next_round = match next_round.play(2, Move::Single(two_of_hearts)){
        Ok(r) => r,
        Err(r) => r
    };

    let next_round = match next_round.play(3, Move::Pass){
        Ok(r) => r,
        Err(r) => r
    };

    let next_round = match next_round.play(1, Move::Pass){
        Ok(r) => r,
        Err(r) => r
    };

    let valid_move = match next_round.play(2, Move::Pair(ace_of_spades, ace_of_spades)){
        Ok(_) => true,
        _     => false
    };

    assert!(valid_move);
}

#[test]
pub fn flush_beats_a_straight(){

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    let four_of_spades = Card::new(Rank::Four, Suit::Spades);
    let five_of_clubs = Card::new(Rank::Five, Suit::Clubs);
    let six_of_clubs = Card::new(Rank::Six, Suit::Clubs);
    let seven_of_clubs = Card::new(Rank::Seven, Suit::Clubs);
    let nine_of_clubs = Card::new(Rank::Nine, Suit::Clubs);

    let straight = build_move(vec!(three_of_clubs,
                        four_of_spades,
                        five_of_clubs,
                        six_of_clubs,
                        seven_of_clubs)).unwrap();

    let flush = build_move(vec!(three_of_clubs,
                        five_of_clubs,
                        six_of_clubs,
                        seven_of_clubs,
                        nine_of_clubs)).unwrap();

    let r = Round::new(vec!(1, 2, 3), 1, straight);

    let valid_move = match r.play(1, flush){
        Ok(_)   => true,
        _       => false
    };

    assert!(valid_move);

}

#[test]
pub fn full_house_beats_a_flush(){
    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    let three_of_hearts = Card::new(Rank::Three, Suit::Hearts);
    let three_of_diamonds = Card::new(Rank::Three, Suit::Diamonds);
    let four_of_clubs = Card::new(Rank::Four, Suit::Clubs);
    let four_of_hearts = Card::new(Rank::Four, Suit::Hearts);

    let five_of_clubs = Card::new(Rank::Five, Suit::Clubs);
    let six_of_clubs = Card::new(Rank::Six, Suit::Clubs);
    let eight_of_clubs = Card::new(Rank::Eight, Suit::Clubs);

    let full_house = build_move(vec!(three_of_clubs,
                                three_of_hearts,
                                three_of_diamonds,
                                four_of_clubs,
                                four_of_hearts)).unwrap();

    let flush = build_move(vec!(three_of_clubs,
                                four_of_clubs,
                                five_of_clubs,
                                six_of_clubs,
                                eight_of_clubs)).unwrap();

    let r = Round::new(vec!(1, 2), 1, flush);

    let valid_move = match r.play(1, full_house){
        Ok(_) => true,
        _     => false
    };

    assert!(valid_move);
}

#[test]
pub fn four_of_a_kind_beats_full_house(){
    let full_house = build_move(vec!(
                        card!(King, Hearts),
                        card!(King, Spades),
                        card!(King, Clubs),
                        card!(Five, Hearts),
                        card!(Five, Diamonds))).unwrap();

    let four_of_kind = build_move(vec!(
                        card!(Three, Spades),
                        card!(Three, Clubs),
                        card!(Three, Diamonds),
                        card!(Three, Hearts),
                        card!(Seven, Hearts))).unwrap();

    let r = Round::new(vec!(1, 2), 1, full_house);

    let valid_move = match r.play(1, four_of_kind){
        Ok(_) => true,
        _ => false
    };

    assert!(valid_move);

}

#[test]
pub fn straight_flush_beats_four_of_a_kind(){
    let straight_flush = build_move(vec!(
                        card!(Three, Clubs),
                        card!(Four, Clubs),
                        card!(Five, Clubs),
                        card!(Six, Clubs),
                        card!(Seven, Clubs))).unwrap();

    let four_of_kind = build_move(vec!(
                        card!(Two, Spades),
                        card!(Two, Clubs),
                        card!(Two, Diamonds),
                        card!(Two, Hearts),
                        card!(Seven, Hearts))).unwrap();

    let r = Round::new(vec!(1, 2), 1, four_of_kind);

    let valid_move = match r.play(1, straight_flush){
        Ok(_) => true,
        _ => false
    };

    assert!(valid_move);


}

#[test]
pub fn five_of_a_kind_is_the_baddest(){
    let straight_flush = build_move(vec!(
                        card!(Three, Clubs),
                        card!(Four, Clubs),
                        card!(Five, Clubs),
                        card!(Six, Clubs),
                        card!(Seven, Clubs))).unwrap();

    let five_of_kind = build_move(vec!(
                        card!(Two, Spades),
                        card!(Two, Clubs),
                        card!(Two, Diamonds),
                        card!(Two, Hearts),
                        card!(Two, Hearts))).unwrap();

    let r = Round::new(vec!(1, 2), 1, straight_flush);

    let valid_move = match r.play(1, five_of_kind){
        Ok(_) => true,
        _ => false
    };

    assert!(valid_move);

}
