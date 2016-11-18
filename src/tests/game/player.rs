use cards::types::*;
use cards::card::{PlayerCard, Card};
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

#[test]
pub fn players_can_have_cards_taken_away(){
    let p1 = Player::new(0).set_hand(vec!(card!(Three, Clubs), card!(Two, Spades), card!(Four, Diamonds)));

    let remove_cards = vec!(card!(Three, Clubs));
    let p1_updated = p1.remove(&remove_cards);

    assert_eq!(p1_updated.get_hand().len(), 2);
}
