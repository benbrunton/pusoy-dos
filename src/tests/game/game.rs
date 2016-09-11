// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::Game;
use game::player::Player;
use cards::deck::Deck;

#[test]
pub fn game_should_deal_cards_to_each_player_on_setup(){

    let mut player1 = Player::new();
    let mut player2 = Player::new();

    let mut game = Game::new();

    let player1_index = game.add_player(&mut player1);
    let player2_index = game.add_player(&mut player2);

    game.start();

    let player1_cards = game.check_player(player1_index).get_hand();
    let player2_cards = game.check_player(player2_index).get_hand();

    assert_eq!(player1_cards.len(), 26);
    assert_eq!(player2_cards.len(), 26);

}

// this test needs work
pub fn deck_should_be_shuffled_on_start(){
    let mut deck = Deck::new();    
    let mut player = Player::new();
    let mut game = Game::new();

    game.add_player(&mut player);
    game.add_deck(&deck);

    game.start();

    let mut player_hand = game.check_player(0).get_hand();
    
    let original = &deck.deal().unwrap();   
    let new = player_hand.pop().unwrap();

    assert!(*original != new);
}
