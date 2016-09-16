use game::round::Round;
use game::player::Player;
use cards::deck::Deck;
use cards::types::{Rank, Suit};
use cards::card::Card;
use game::player_move::{Move, build_move};

/// A definition of a game in progress
pub struct GameDefinition{
    /// players
    pub players: Vec<Player>,
    /// round
    pub round: Round
}

/// The Game module
pub struct Game { 
    players: Vec<Player>,
    round: Round
}

impl Game{
 
    /// create a new Game
    pub fn setup(player_count:usize) -> Result<GameDefinition, &'static str>{
        let deck = Deck::new();

        let dealt_cards = deck.deal(player_count);

        let players:Vec<Player> = (0..player_count).map(|n: usize| {
            let player = Player::new(0);
            player.set_hand(dealt_cards[n].clone())
        }).collect();

        Ok(
            GameDefinition{
                players: players.clone(),
                round: Game::get_empty_round()
            }
        )
    }

    /// load an existing game from a `GameDefinition`
    pub fn load(game_definition: GameDefinition) -> Result<Game, &'static str>{

        Ok(
            Game{
                players: game_definition.players,
                round: Game::get_empty_round()
            }
        )
    }

    /// takes a player_id and a vec of cards for a move
    pub fn player_move(&self, player_id:i32, cards:Vec<Card>) -> Result<GameDefinition, &'static str> {
       let p_move = build_move(cards.clone());

        // only allow valid hands
       if p_move == None {
            return Err("Invalid move!");
       }

        // get player from id
       let mut current_player = Player::new(-1);

       for player in &self.players {
            if player.get_id() == player_id {
                current_player = player.clone();
            }
       }

        // only allow cards in player hand
       for card in cards {
           if !current_player.get_hand().contains(&card){
                return Err("Cannot play cards you do not have");
           }
       }

       let round = match self.round.play(player_id, p_move.unwrap()){
            Ok(r) => r,
            Err(r) => r
       };

       Ok(GameDefinition{
          players: self.players.clone(),
          round: round
       })
       
    }
  
    /// get a player for querying information
    pub fn get_player(&self, n: usize) -> Option<&Player> {
       self.players.get(n)
    }

    /// get the next player to play
    pub fn get_next_player(&self) -> Option<Player> {
        let three_of_clubs = card!(Three, Clubs);
        
        for player in &self.players {
            if player.get_hand().contains(&three_of_clubs){
                return Some(player.clone());
            }
        }
        
        None
    }

    fn get_empty_round() -> Round {
        Round::new(vec!(0, 1), 0, Move::Pass, 0, false)
    }

}
