// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::{ Game, GameDefinition};
use cards::card::*;
use cards::types::*;
use game::player::Player;
use game::round::Round;

#[test]
pub fn game_can_deal_cards_to_each_player_on_setup(){

    let new_game = Game::setup(2).unwrap();

    let player1_cards = new_game.get_player(0).unwrap().get_hand();
    let player2_cards = new_game.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 26);
    assert_eq!(player2_cards.len(), 26);

}

#[test]
pub fn game_can_load_in_any_state(){

    let player1 = Player::new().set_hand(vec!(card!(Ace, Spades)));
    let player2 = Player::new().set_hand(vec!(card!(Two, Hearts), card!(Two, Clubs)));

    let game_definition = GameDefinition{
        players: vec!(player1, player2)
    };

    let existing_game = Game::load(game_definition).unwrap();

    let player1_cards = existing_game.get_player(0).unwrap().get_hand();
    let player2_cards = existing_game.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 1);
    assert_eq!(player2_cards.len(), 2);
}

pub fn making_a_valid_move_returns_a_new_game(){

//    let game = Game::setup(2);

 //   game.player_move(0, vec!(card!(

}
