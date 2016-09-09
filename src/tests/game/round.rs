use game::round::Round;
use game::player_move::Move;
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
