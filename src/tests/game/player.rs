use cards::types::*;
use cards::card::Card;
use cards::*;
use game::player::Player;

#[test]
pub fn giving_a_player_a_card_increases_number_in_hand(){
    let mut p1 = Player::new();

    assert_eq!(p1.remaining_cards(), 0);
    
    p1.receive(card::Card::new(types::Rank::Ace, types::Suit::Spades));

    assert_eq!(p1.remaining_cards(), 1);
}

#[test]
pub fn a_player_needs_to_know_what_cards_it_has(){

    let mut p1 = Player::new();
    p1.receive(card!(Three, Clubs));
    p1.receive(card!(Two, Spades));

    assert_eq!(p1.get_hand(), vec!(card!(Three, Clubs), card!(Two, Spades)));
}

