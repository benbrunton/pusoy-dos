// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::{ Game, GameDefinition};
use game::player_move::Move;
use cards::card::*;
use cards::types::*;
use game::player::Player;
use game::round::Round;

#[test]
pub fn game_can_deal_cards_to_each_player_on_setup(){

    let new_game_definition = Game::setup(2).unwrap();

    let player1_cards = new_game_definition.players[0].get_hand();
    let player2_cards = new_game_definition.players[1].get_hand();

    assert_eq!(player1_cards.len(), 26);
    assert_eq!(player2_cards.len(), 26);

}

#[test]
pub fn game_can_load_in_any_state(){

    let player1 = Player::new(0).set_hand(vec!(card!(Ace, Spades)));
    let player2 = Player::new(1).set_hand(vec!(card!(Two, Hearts), card!(Two, Clubs)));

    let game_definition = GameDefinition{
        players: vec!(player1, player2),
        round:Round::new(vec!(0, 1), 0, Move::Pass, 0, false),
        winner: None
    };

    let existing_game = Game::load(game_definition).unwrap();

    let player1_cards = existing_game.get_player(0).unwrap().get_hand();
    let player2_cards = existing_game.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 1);
    assert_eq!(player2_cards.len(), 2);
}

#[test]
pub fn the_player_with_three_clubs_starts_the_game(){

    let game_def = Game::setup(2).unwrap();

    let game = Game::load(game_def).unwrap();
    
    let p1_cards = game.get_player(0).unwrap().get_hand();

    let next_player = match game.get_next_player(){
        Some(player) => player.clone(),
        None         => Player::new(0)    
    };

    let three_of_clubs = card!(Three, Clubs);

    let three_belongs_to = if p1_cards.contains(&three_of_clubs) {
        game.get_player(0).unwrap().clone()
    } else {
        game.get_player(1).unwrap().clone()
    };

    assert_eq!(three_belongs_to, next_player);

}

#[test]
pub fn valid_moves_return_new_game_definition(){

    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));

    let round = Round::new(vec!(0, 1), 0, Move::Single(card!(Three, Clubs)), 0, false);
    
    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round:round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(0, vec!(card!(Four, Hearts))).unwrap();

    assert_eq!(new_game_def.round.get_next_player(), 1);

    
}

#[test]
pub fn player_can_only_play_cards_in_its_hand(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));

    let round = Round::new(vec!(0, 1), 0, Move::Single(card!(Three, Clubs)), 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let invalid_move = match game.player_move(0, vec!(card!(Four, Diamonds))){
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);

}

#[test]
pub fn player_loses_cards_when_move_is_valid(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs), card!(Three, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));

    let round = Round::new(vec!(0, 1), 0, Move::Single(card!(Three, Clubs)), 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(card!(Three, Hearts))).unwrap();

    assert_eq!(new_game.players[0].get_hand().len(), 2);

}

#[test]
pub fn player_keeps_cards_when_move_is_invalid(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Diamonds), card!(Six, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Five, Clubs)));

    let round = Round::new(vec!(0, 1), 0, Move::Single(card!(Queen, Spades)), 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(card!(Four, Diamonds))).unwrap();

    assert_eq!(new_game.players[0].get_hand(), vec!(card!(Four, Diamonds), card!(Six, Hearts)));
}

#[test]
pub fn player_using_last_card_wins(){
    let player1 = Player::new(0).set_hand(vec!(card!(Two, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));

    let round = Round::new(vec!(0, 1), 0, Move::Single(card!(Ten, Clubs)), 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(0, vec!(card!(Two, Hearts))).unwrap();

    assert_eq!(new_game_def.winner.unwrap(), 0);

}

#[test]
pub fn winner_is_removed_from_play_rotation(){
    let player1 = Player::new(0).set_hand(vec!(card!(Two, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));
    let player3 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));

    let round = Round::new(vec!(0, 1, 2), 0, Move::Single(card!(Ten, Clubs)), 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3),
        round: round,
        winner: None
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(0, vec!(card!(Two, Hearts))).unwrap();

    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(1, vec!()).unwrap();
    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(2, vec!()).unwrap();
    let game = Game::load(new_game_def).unwrap();

    assert_eq!(game.get_next_player().unwrap().get_id(), 1);

}
