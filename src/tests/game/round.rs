use game::round::{Round, RoundDefinition};
use game::player_move::{ Move, build_move };
use cards::card::{ PlayerCard, Card };
use cards::types::*;

#[test]
pub fn it_returns_the_current_player_id(){
    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass, 0, false);

    assert_eq!(r.get_next_player(), 1);
}

#[test]
pub fn it_returns_a_new_round_when_passed_a_valid_move(){
    // Pass is always a valid move

    let r = Round::new(vec!(1, 2, 3), 1, build_move(vec!(card!(Two, Hearts))).unwrap(), 0, false);

    let new_round = match r.play(1, Move::Pass) {
        Ok(r) => r,
        Err(r) => r
    };

    assert!(new_round != r);
}

#[test]
pub fn it_returns_an_error_when_a_move_is_invalid(){

    let mv = Move::Pair(
        Card::new(Rank::Ace, Suit::Spades, false),
        Card::new(Rank::Ace, Suit::Hearts, false)
    );

    let new_move = Move::Single(
        Card::new(Rank::Three, Suit::Clubs, false)
    );
    let r = Round::new(vec!(1, 2, 3), 1, mv, 0, false);

    let new_round = r.play(1, new_move);

    assert_eq!(new_round, Err(r));
}

#[test]
pub fn a_valid_move_rotates_the_players(){

    let r = Round::new(vec!(7, 13, 3), 7, build_move(vec!(card!(Two, Hearts))).unwrap(), 0, false);

    let new_round = match r.play(7, Move::Pass) {
        Ok(r) => r,
        Err(r) => r
    };

    assert_eq!(new_round.get_next_player(), 13);

}

#[test]
pub fn a_valid_first_move_rotates_the_players(){

    let r = Round::new(vec!(5, 4), 5, Move::Pass, 0, true);

    let new_round = match r.play(5, Move::Single(card!(Three, Clubs).to_card())) {
        Ok(r) => r,
        Err(r) => r
    };

    assert_eq!(new_round.get_next_player(), 4);

}

#[test]
pub fn rotating_the_player_will_bring_it_back_to_beginning_of_vec(){

    let r = Round::new(vec!(8, 15, 3), 3, build_move(vec!(card!(Three, Clubs))).unwrap(), 0, false);

    let new_round = match r.play(3, build_move(vec!(card!(Three, Hearts))).unwrap()) {
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
   Round::new(vec!(1, 2, 3), 4, Move::Pass, 0, false);
}

#[test]
pub fn only_the_current_player_can_make_a_move(){
    let r = Round::new(vec!(1, 2), 2, Move::Pass, 0, false);

    let invalid_move = match r.play(1, Move::Pass) {
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn any_hand_can_be_passed_onto_an_emtpy_round(){

    let r = Round::new(vec!(1, 2), 1, Move::Pass, 0, false);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades, false);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds, false);

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

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs, false);
    let four_of_diamonds = Card::new(Rank::Four, Suit::Diamonds, false);

    let r = Round::new(vec!(1, 2), 1, Move::Single(three_of_clubs), 0, false);

    let valid_move = match r.play(1, Move::Single(four_of_diamonds)) {
        Ok(_)   => true,
        _       => false
    };

    assert!(valid_move);
}

#[test]
pub fn single_cannot_be_beaten_by_a_higher_single(){

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs, false);
    let four_of_diamonds = Card::new(Rank::Four, Suit::Diamonds, false);

    let r = Round::new(vec!(1, 2), 1, Move::Single(four_of_diamonds), 0, false);

    let invalid_move = match r.play(1, Move::Single(three_of_clubs)) {
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn single_card_respects_suit_order() {
    let tc = Card::new(Rank::Three, Suit::Clubs, false);
    let th = Card::new(Rank::Three, Suit::Hearts, false);
    let td = Card::new(Rank::Three, Suit::Diamonds, false);

    let r = Round::new(vec!(1,2,3), 1, Move::Single(th), 0, false);

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

    let three_of_clubs = Card::new(Rank::Three, Suit::Clubs, false);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades, false);
    let ace_of_diamonds = Card::new(Rank::Ace, Suit::Diamonds, false);

    let r = Round::new(vec!(1, 2), 1, Move::Single(three_of_clubs), 0, false);

    let invalid_move = match r.play(1, Move::Pair(ace_of_spades, ace_of_diamonds)){
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn when_every_player_passes_the_last_player_to_move_starts_the_round(){

    let r = Round::new(vec!(1, 2, 3), 1, Move::Pass, 0, false);

    let ace_of_spades = Card::new(Rank::Ace, Suit::Spades, false);
    let two_of_hearts = Card::new(Rank::Two, Suit::Hearts, false);

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

    let three_of_clubs = card!(Three, Clubs);
    let four_of_spades = card!(Four, Spades);
    let five_of_clubs = card!(Five, Clubs);
    let six_of_clubs = card!(Six, Clubs);
    let seven_of_clubs = card!(Seven, Clubs);
    let nine_of_clubs = card!(Nine, Clubs);

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

    let r = Round::new(vec!(1, 2, 3), 1, straight, 0, false);

    let valid_move = match r.play(1, flush){
        Ok(_)   => true,
        _       => false
    };

    assert!(valid_move);

}

#[test]
pub fn full_house_beats_a_flush(){
    let three_of_clubs = card!(Three, Clubs);
    let three_of_hearts = card!(Three, Hearts);
    let three_of_diamonds = card!(Three, Diamonds);
    let four_of_clubs = card!(Four, Clubs);
    let four_of_hearts = card!(Four, Hearts);

    let five_of_clubs = card!(Five, Clubs);
    let six_of_clubs = card!(Six, Clubs);
    let eight_of_clubs = card!(Eight, Clubs);

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

    let r = Round::new(vec!(1, 2), 1, flush, 0, false);

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

    let r = Round::new(vec!(1, 2), 1, full_house, 0, false);

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

    let r = Round::new(vec!(1, 2), 1, four_of_kind, 0, false);

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

    let r = Round::new(vec!(1, 2), 1, straight_flush, 0, false);

    let valid_move = match r.play(1, five_of_kind){
        Ok(_) => true,
        _ => false
    };

    assert!(valid_move);

}

#[test]
pub fn pass_count_controls_when_round_is_reset(){

    let r = Round::new(vec!(1, 2, 3), 
                3, 
                build_move(vec!(card!(Four, Clubs))).unwrap(), 
                2,
                false);

    let new_round = match r.play(3, Move::Pass){
        Ok(new_round)   => new_round,
        _               => r
    };

    assert_eq!(new_round.get_last_move(), Move::Pass);
    assert_eq!(new_round.get_next_player(), 1);


}

#[test]
pub fn first_round_has_to_include_three_of_clubs(){

    let r = Round::new(vec!(0, 1, 2), 0, Move::Pass, 0, true);

    let invalid_move = match r.play(0, build_move(vec!(card!(Four, Clubs))).unwrap()) {
        Err(_)  => true,
        _       => false
    };

    let valid_move = match r.play(0, build_move(vec!(card!(Three, Clubs))).unwrap()) {
        Ok(_)  => true,
        _       => false
    };

    let valid_move2 = match r.play(0, 
        build_move(vec!(card!(Three, Clubs), card!(Three, Hearts))).unwrap()) {
        Ok(_)   => true,
        _       => false
    };


    assert!(invalid_move);
    assert!(valid_move);
    assert!(valid_move2);
}

#[test]
pub fn single_four_cannot_beat_a_queen(){
    let r = Round::new(vec!(0, 1),
                        0,
                        build_move(vec!(card!(Queen, Spades))).unwrap(),
                        0,
                        false);

    let invalid_move = match r.play(0, build_move(vec!(card!(Four, Diamonds))).unwrap()) {
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn round_can_export_to_a_definition(){
    let r = Round::new(vec!(0, 1), 0, Move::Pass, 0, false);

    let r_def = RoundDefinition{
        players: vec!(0, 1),
        current_player: 0,
        last_move: Move::Pass,
        pass_count: 0,
        first_round: false
    };

    assert_eq!(r.export(), r_def);
}

#[test]
pub fn round_can_have_players_updated(){
    let r = Round::new(vec!(1, 2, 3, 4), 1, Move::Pass, 0, false);
    let new_round = r.update_players(vec!(1, 2, 3));

    assert_eq!(new_round.export().players.len(), 3);

}

#[test]
pub fn fours_on_aces_beats_threes_on_twos(){
    let r = Round::new(vec!(0, 1),
                    0,
                    build_move(vec!(card!(Three, Clubs), 
                        card!(Three, Spades), 
                        card!(Three, Diamonds), 
                        card!(Two, Spades),
                        card!(Two, Diamonds))).unwrap(),
                    0,
                    false);

    let valid_move = match r.play(0, build_move(vec!(card!(Four, Clubs),
                                                    card!(Four, Diamonds),
                                                    card!(Four, Hearts),
                                                    card!(Ace, Spades),
                                                    card!(Ace, Hearts))).unwrap()) {
            Ok(_) => true,
            _     => false
    };

    assert_eq!(valid_move, true);

}

#[test]
pub fn kings_on_fours_beats_fives_on_tens(){
    let r = Round::new(vec!(0, 1),
                    0,
                    build_move(vec!(card!(Five, Clubs), 
                        card!(Five, Spades), 
                        card!(Five, Diamonds), 
                        card!(Ten, Spades),
                        card!(Ten, Diamonds))).unwrap(),
                    0,
                    false);

    let valid_move = match r.play(0, build_move(vec!(card!(King, Clubs),
                                                    card!(King, Diamonds),
                                                    card!(King, Hearts),
                                                    card!(Four, Spades),
                                                    card!(Four, Hearts))).unwrap()) {
            Ok(_) => true,
            _     => false
    };

    assert_eq!(valid_move, true);


}

#[test]
pub fn ten_club_high_flush_beats_nine_spade_high(){

    let ten_club_high_flush = build_move(vec!(card!(Ten, Clubs),
                                    card!(Three, Clubs),
                                    card!(Four,Clubs),
                                    card!(Five, Clubs),
                                    card!(Six, Clubs))).unwrap();

    let nine_spade_high_flush = build_move(vec!(card!(Nine, Spades),
                                    card!(Four, Spades),
                                    card!(Five, Spades),
                                    card!(Six, Spades),
                                    card!(Seven, Spades))).unwrap();


    let r = Round::new(vec!(0, 1),
                        0,
                        nine_spade_high_flush,
                        0,
                        false);

    let valid_move = match r.play(0, ten_club_high_flush){
        Ok(_) => true,
        _     => false
    };

    assert_eq!(valid_move, true);
}

#[test]
pub fn only_top_card_counts_in_full_house(){
    let unbeatable_full_house = build_move(vec!(card!(Two, Spades),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs),
                                                card!(Three, Clubs),
                                                card!(Three, Clubs))).unwrap();

    let beatable_full_house = build_move(vec!(card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Ace, Spades),
                                                card!(Ace, Spades))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_full_house,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_full_house){
        Ok(_) => true,
        _     => false
    };

    assert_eq!(valid_move, true);
}

#[test]
pub fn only_top_card_counts_in_four_of_a_kind(){
    let unbeatable_four_of_kind = build_move(vec!(card!(Two, Spades),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs),
                                                card!(Three, Clubs))).unwrap();

    let beatable_four_of_kind = build_move(vec!(card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Ace, Spades))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_four_of_kind,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_four_of_kind){
        Ok(_) => true,
        _     => false
    };

    assert_eq!(valid_move, true);
}

#[test]
pub fn only_top_card_counts_in_five_of_a_kind(){
    let unbeatable_five_of_kind = build_move(vec!(card!(Two, Spades),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs),
                                                card!(Two, Clubs))).unwrap();

    let beatable_five_of_kind = build_move(vec!(card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds),
                                                card!(Two, Diamonds))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_five_of_kind,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_five_of_kind){
        Ok(_) => true,
        _     => false
    };

    assert_eq!(valid_move, true);
}

#[test]
pub fn only_top_card_counts_in_a_pair(){
    let unbeatable_pair_of_eights = build_move(vec!(card!(Eight, Clubs), card!(Eight, Spades))).unwrap();
    let beatable_pair_of_eights = build_move(vec!(card!(Eight, Diamonds), card!(Eight, Diamonds))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_pair_of_eights,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_pair_of_eights){
        Ok(_) => true,
        _     => false
    };

    assert!(valid_move);
}

#[test]
pub fn only_bottom_card_counts_in_reversed_pair(){
    let unbeatable_pair_of_fours = build_move(vec!(card!(Four, Clubs, true), card!(Four, Hearts, true))).unwrap();
    let beatable_pair_of_fours = build_move(vec!(card!(Four, Spades, true), card!(Four, Diamonds, true))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_pair_of_fours,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_pair_of_fours){
        Ok(_) => true,
        _     => false
    };

    assert!(valid_move);
}

#[test]
pub fn only_bottom_card_counts_in_reversed_pair_scenario_2(){
    let unbeatable_pair_of_fours = build_move(vec!(card!(Four, Clubs, true), card!(Four, Spades, true))).unwrap();
    let beatable_pair_of_fours = build_move(vec!(card!(Four, Spades, true), card!(Four, Diamonds, true))).unwrap();

    let r = Round::new(vec!(0, 1),
                        0,
                        beatable_pair_of_fours,
                        0,
                        false);

    let valid_move = match r.play(0, unbeatable_pair_of_fours){
        Ok(_) => true,
        _     => false
    };

    assert!(valid_move);
}

#[test]
pub fn higher_four_of_a_kind_is_invalid_after_reversal(){
    let unbeatable_fours = build_move(vec!(card!(Four, Clubs), card!(Four, Hearts), card!(Four, Spades), card!(Four, Diamonds), card!(Five, Hearts))).unwrap();
    let beatable_fives = build_move(vec!(card!(Five, Clubs, true), card!(Five, Hearts, true), card!(Five, Spades, true), card!(Five, Diamonds, true), card!(Four, Hearts))).unwrap();
    let r = Round::new(vec!(0, 1),
                        0,
                        unbeatable_fours,
                        0,
                        false);

    let invalid_move = match r.play(0, beatable_fives){
        Err(_) => true,
        _     => false
    };

    assert!(invalid_move);
}

#[test]
pub fn thou_shalt_not_pass_on_an_empty_table(){
    let r = Round::new(vec!(0, 1), 0, build_move(vec!()).unwrap(), 0, false);

    let invalid_move = match r.play(0, build_move(vec!()).unwrap()){
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);
}

#[test]
pub fn skip_allows_it_though(){
    let r = Round::new(vec!(0, 1), 0, build_move(vec!()).unwrap(), 0, false);

    let invalid_move = match r.skip(0){
        Err(_)  => false,
        _       => true
    };

    assert!(invalid_move);

}
