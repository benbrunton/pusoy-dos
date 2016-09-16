use cards::types::*;
use cards::card::Card;
use game::player::Player;

#[test]
pub fn a_player_needs_an_id(){

    let p1 = Player::new(0);

    assert_eq!(p1.get_id(), 0);
}

#[test]
pub fn a_player_can_have_a_hand_set(){
    let p1 = Player::new(0);
    let hand = vec!(card!(Ace, Spades), card!(Three, Clubs));

    assert_eq!(p1.remaining_cards(), 0);
    
    let p2 = p1.set_hand(hand);

    assert_eq!(p2.remaining_cards(), 2);
}

#[test]
pub fn a_player_needs_to_know_what_cards_it_has(){

    let p1 = Player::new(0);
    let p1 = p1.set_hand(vec!(card!(Three, Clubs), card!(Two, Spades)));

    assert_eq!(p1.get_hand(), vec!(card!(Three, Clubs), card!(Two, Spades)));
}

