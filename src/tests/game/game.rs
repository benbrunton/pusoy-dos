// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::Game;

#[test]
pub fn game_should_be_able_to_deal_cards_to_each_player_on_setup(){

    let new_game = Game::setup(2).unwrap();

    let player1_cards = new_game.get_player(0).unwrap().get_hand();
    let player2_cards = new_game.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 26);
    assert_eq!(player2_cards.len(), 26);

}

