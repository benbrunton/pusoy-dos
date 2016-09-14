// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::{Game, GameDefinition};
use game::player::Player;

#[test]
pub fn game_should_be_able_to_deal_cards_to_each_player_on_setup(){

    let player1 = Player::new();
    let player2 = Player::new();
    let game_definition = GameDefinition {
        players: vec!(player1, player2)   
    };

    let game = Game::new(game_definition);

    let game_updated = game.setup().unwrap();

    let player1_cards = game_updated.get_player(0).unwrap().get_hand();
    let player2_cards = game_updated.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 26);
    assert_eq!(player2_cards.len(), 26);

}

