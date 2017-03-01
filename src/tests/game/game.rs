// we should be able to write tests that are 
// more or less sections of games here.
// Maybe even some full games.

use game::game::{ Game, GameDefinition};
use game::player_move::Move;
use cards::card::*;
use cards::types::*;
use game::player::Player;
use game::round::Round;
use game::player_move::build_move;

#[test]
pub fn game_can_deal_cards_to_each_player_on_setup(){

    let new_game_definition = Game::setup(vec!(0, 1)).unwrap();

    let player1_cards = new_game_definition.players[0].get_hand();
    let player2_cards = new_game_definition.players[1].get_hand();

    assert_eq!(player1_cards.len(), 27);
    assert_eq!(player2_cards.len(), 27);

}

#[test]
pub fn game_can_load_in_any_state(){

    let player1 = Player::new(0).set_hand(vec!(card!(Ace, Spades)));
    let player2 = Player::new(1).set_hand(vec!(card!(Two, Hearts), card!(Two, Clubs)));

    let game_definition = GameDefinition{
        players: vec!(player1, player2),
        round:Round::new(vec!(0, 1), 0, Move::Pass, 0, false),
        winners: vec!(),
        reversed: false
    };

    let existing_game = Game::load(game_definition).unwrap();

    let player1_cards = existing_game.get_player(0).unwrap().get_hand();
    let player2_cards = existing_game.get_player(1).unwrap().get_hand();

    assert_eq!(player1_cards.len(), 1);
    assert_eq!(player2_cards.len(), 2);
}

#[test]
pub fn the_player_with_three_clubs_starts_the_game(){

    let game_def = Game::setup(vec!(0,1)).unwrap();

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

    let single_three = build_move(vec!(card!(Three, Clubs))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_three, 0, false);
    
    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round:round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(0, vec!(card!(Four, Hearts))).unwrap();

    assert_eq!(new_game_def.round.get_next_player(), 1);

    
}

#[test]
pub fn player_can_only_play_cards_in_its_hand(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));
    
    let single_three = build_move(vec!(card!(Three, Clubs))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_three, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let invalid_move = match game.player_move(0, vec!(card!(Four, Diamonds))){
        Err(_)  => true,
        _       => false
    };

    assert!(invalid_move);

}

#[test]
pub fn jokers_are_used_as_wildcards(){
    let player1 = Player::new(0).set_hand(vec!(
            card!(Four, Hearts), card!(Five, Clubs), PlayerCard::Joker(0)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));
    
    let single_three = build_move(vec!(card!(Three, Clubs))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_three, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let valid_move = match game.player_move(0, vec!(wildcard!(Four, Diamonds))){
        Ok(_)  => true,
        _       => false
    };

    assert!(valid_move);

}

#[test]
pub fn player_loses_cards_when_move_is_valid(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs), card!(Three, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));

    let single_three = build_move(vec!(card!(Three, Clubs))).unwrap();
    let round = Round::new(vec!(0, 1), 0, single_three, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(card!(Three, Hearts))).unwrap();

    assert_eq!(new_game.players[0].get_hand().len(), 2);

}

#[test]
pub fn player_loses_joker_when_playing_wildcard(){
    let player1 = Player::new(0).set_hand(vec!(
            card!(Four, Hearts), card!(Five, Clubs), PlayerCard::Joker(0)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));
    
    let single_three = build_move(vec!(card!(Three, Clubs))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_three, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(wildcard!(Four, Diamonds))).unwrap();

    assert_eq!(new_game.players[0].get_hand().len(), 2);


}

#[test]
pub fn player_keeps_cards_when_move_is_invalid(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Diamonds), card!(Six, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Five, Clubs)));

    let single_queen = build_move(vec!(card!(Queen, Spades))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_queen, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(card!(Four, Diamonds))).unwrap();

    assert_eq!(new_game.players[0].get_hand(), vec!(card!(Four, Diamonds), card!(Six, Hearts)));
}

#[test]
pub fn player_using_last_card_wins(){
    let player1 = Player::new(0).set_hand(vec!(card!(Two, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));

    let single_ten = build_move(vec!(card!(Ten, Clubs))).unwrap();
    let round = Round::new(vec!(0, 1), 0, single_ten, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(0, vec!(card!(Two, Hearts))).unwrap();

    assert_eq!(new_game_def.winners, vec!(0));

}

#[test]
pub fn players_are_added_to_the_winners_vec_as_they_run_out_of_cards(){
    let player1 = Player::new(0).set_hand(vec!());
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));
    let player3 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));
    let player4 = Player::new(3).set_hand(vec!(card!(Seven, Spades)));

    let single_ten = build_move(vec!(card!(Ten, Clubs))).unwrap();

    let round = Round::new(vec!(1, 2, 3), 2, single_ten, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3, player4),
        round: round,
        winners: vec!(0),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(2, vec!(card!(Two, Clubs))).unwrap();

    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(3, vec!()).unwrap();
    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(1, vec!()).unwrap();

    assert_eq!(new_game_def.winners.len(), 2);
}

#[test]
pub fn winner_is_removed_from_play_rotation(){
    let player1 = Player::new(0).set_hand(vec!(card!(Two, Hearts)));
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));
    let player3 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));

    let single_ten = build_move(vec!(card!(Ten, Clubs))).unwrap();
    let round = Round::new(vec!(0, 1, 2), 0, single_ten, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3),
        round: round,
        winners: vec!(),
        reversed: false
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

#[test]
pub fn subsequent_finishing_players_are_removed(){
    let player1 = Player::new(0).set_hand(vec!());
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));
    let player3 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));
    let player4 = Player::new(3).set_hand(vec!(card!(Seven, Spades)));

    let single_ten = build_move(vec!(card!(Ten, Clubs))).unwrap();

    let round = Round::new(vec!(1, 2, 3), 2, single_ten, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3, player4),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(2, vec!(card!(Two, Clubs))).unwrap();

    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(3, vec!()).unwrap();
    let game = Game::load(new_game_def).unwrap();
    let new_game_def = game.player_move(1, vec!()).unwrap();
    let game = Game::load(new_game_def).unwrap();

    assert_eq!(game.get_next_player().unwrap().get_id(), 3);


}

#[test]
pub fn player_removal_is_reflected_in_the_stored_round(){
    let player1 = Player::new(0).set_hand(vec!());
    let player2 = Player::new(1).set_hand(vec!(card!(Queen, Diamonds)));
    let player3 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));
    let player4 = Player::new(3).set_hand(vec!(card!(Seven, Spades)));

    let single_ten = build_move(vec!(card!(Ten, Clubs))).unwrap();

    let round = Round::new(vec!(1, 2, 3), 2, single_ten, 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3, player4),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(2, vec!(card!(Two, Clubs))).unwrap();

    assert_eq!(new_game_def.round.export().players, vec!(1, 3));


}

#[test]
pub fn playing_a_four_card_trick_reverses_the_cards(){

    let player1 = Player::new(1).set_hand(vec!(
        card!(Queen, Diamonds), 
        card!(Queen, Hearts), 
        card!(Queen, Clubs), 
        card!(Queen, Spades), 
        card!(Four, Hearts), 
        card!(Two, Spades), 
        card!(Six, Diamonds)));
    let player2 = Player::new(2).set_hand(vec!(card!(Two, Clubs)));

    let round = Round::new(vec!(1, 2), 1, build_move(vec!()).unwrap(), 0, false);
    
    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let new_game_def = game.player_move(1, vec!(card!(Queen, Diamonds),
                                                card!(Queen, Hearts),
                                                card!(Queen, Clubs),
                                                card!(Four, Hearts),
                                                card!(Queen, Spades))).unwrap();

    let last_move = new_game_def.round.export().last_move;

    let card = match last_move {
        Move::FiveCardTrick(trick) => trick.cards[0],
        _ => panic!("should be five card trick")
    };

    let player_card = new_game_def.players[0].get_hand()[0].to_card();

    assert_eq!(card.reversed, true);
    assert_eq!(player_card.reversed, true);


}

#[test]
pub fn reversed_and_not_reversed_are_equal_in_terms_of_player_possession(){
    // meaning if you check a player has a QH(reversed), but they only have a QH(not reversed) in hand
    // then they are counted as having that card
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));
    
    let single_two = build_move(vec!(card!(Two, Clubs, true))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_two, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: true
    };

    let game = Game::load(game_def).unwrap();

    let valid_move = match game.player_move(0, vec!(card!(Four, Hearts))){
        Ok(_)  => true,
        _       => false
    };

    assert!(valid_move);

}

#[test]
pub fn an_unbeatable_hand_auto_passes_other_players(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs), card!(Two, Spades)));
    let player2 = Player::new(1).set_hand(vec!(card!(Three, Diamonds)));
    
    let single_two = build_move(vec!(card!(Two, Clubs))).unwrap();

    let round = Round::new(vec!(0, 1), 0, single_two, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: true
    };

    let game = Game::load(game_def).unwrap();

    let new_game = game.player_move(0, vec!(card!(Two, Spades))).unwrap();

    assert_eq!(new_game.round.get_next_player(), 0);
}

#[test]
pub fn when_a_player_exits_the_next_player_benefits_from_a_full_set_of_passes(){
    let player1 = Player::new(0).set_hand(vec!(card!(Four, Hearts), card!(Five, Clubs)));
    let player2 = Player::new(1).set_hand(vec!(card!(Two, Hearts)));
    let player3 = Player::new(2).set_hand(vec!(card!(Three, Hearts))); 

    let single_two = build_move(vec!(card!(Two, Clubs))).unwrap();
    let winning_two = build_move(vec!(card!(Two, Hearts))).unwrap();

    let round = Round::new(vec!(0, 1, 2), 1, single_two, 0, false); 

    let game_def = GameDefinition{
        players: vec!(player1, player2, player3),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let game1_def = game.player_move(1, vec!(card!(Two, Hearts))).unwrap();

    let game1 = Game::load(game1_def).unwrap();

    let game2 = game1.player_move(2, vec!()).unwrap();
    let last_move = game2.round.export().last_move;

    assert_eq!(game2.round.get_next_player(), 0);
    assert_eq!(last_move, winning_two);

}

#[test]
pub fn consecutive_reversals_will_cancel_each_other_out(){

    let player1 = Player::new(0).set_hand(vec!(card!(Six, Hearts), card!(Six, Diamonds), card!(Six, Clubs),
                                                card!(Six, Spades), card!(Four, Hearts), card!(Five, Hearts)));

    let player2= Player::new(1).set_hand(vec!(card!(Five, Hearts), card!(Five, Diamonds), card!(Five, Clubs),
                                                card!(Five, Spades), card!(Ten, Hearts), card!(Jack, Hearts)));

    let round = Round::new(vec!(0, 1), 0, build_move(vec!()).unwrap(), 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let game1_def = game.player_move(0, vec!(card!(Six, Hearts), card!(Six, Diamonds), card!(Six, Clubs), 
                                                card!(Six, Spades), card!(Four, Hearts))).unwrap();

    assert!(game1_def.reversed);

    let game1 = Game::load(game1_def).unwrap();
    let game2 = game1.player_move(1, vec!(card!(Five, Hearts, true), card!(Five, Diamonds, true), card!(Five, Clubs, true),
                                                card!(Five, Spades, true), card!(Ten, Hearts, true))).unwrap();

    assert!(!game2.reversed);
}

#[test]
pub fn immediately_following_a_reversal_with_an_invalid_reversal_bug(){
    let player1 = Player::new(0).set_hand(vec!(card!(Six, Hearts), card!(Six, Diamonds), card!(Six, Clubs),
                                                card!(Six, Spades), card!(Four, Hearts), card!(Five, Hearts)));

    let player2= Player::new(1).set_hand(vec!(card!(Ten, Hearts), card!(Ten, Diamonds), card!(Ten, Clubs),
                                                card!(Ten, Spades), card!(Six, Hearts), card!(Jack, Hearts)));

    let round = Round::new(vec!(0, 1), 0, build_move(vec!()).unwrap(), 0, false);

    let game_def = GameDefinition{
        players: vec!(player1, player2),
        round: round,
        winners: vec!(),
        reversed: false
    };

    let game = Game::load(game_def).unwrap();

    let game1_def = game.player_move(0, vec!(card!(Six, Hearts), card!(Six, Diamonds), card!(Six, Clubs), 
                                                card!(Six, Spades), card!(Four, Hearts))).unwrap();


    let game1 = Game::load(game1_def.clone()).unwrap();
    let game2 = game1.player_move(1, vec!(card!(Ten, Hearts, true), card!(Ten, Diamonds, true), card!(Ten, Clubs, true),
                                                card!(Ten, Spades, true), card!(Six, Hearts, true))).unwrap();

    assert_eq!(game1_def.clone().reversed, true);
    assert_eq!(game2.reversed, true);
    assert_eq!(game1_def.clone().round.get_next_player(), game2.round.get_next_player());

}
