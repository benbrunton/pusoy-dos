use cards::*;
use game::player::Player;

#[test]
pub fn giving_a_player_a_card_increases_number_in_hand(){
    let mut p1 = Player::new();

    assert_eq!(p1.remaining_cards(), 0);
    
    p1.receive(card::Card::new(types::Rank::Ace, types::Suit::Spades));

    assert_eq!(p1.remaining_cards(), 1);
}
