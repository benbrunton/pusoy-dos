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
    pub round: Round,
    /// winner of round
    pub winner: Option<i32>
}

/// The Game module
pub struct Game { 
    players: Vec<Player>,
    round: Round,
    winner: Option<i32>
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
                round: Game::get_empty_round(),
                winner: None
            }
        )
    }

    /// load an existing game from a `GameDefinition`
    pub fn load(game_definition: GameDefinition) -> Result<Game, &'static str>{

        Ok(
            Game{
                players: game_definition.players,
                round: game_definition.round, 
                winner: None
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
       let current_player = self.get_current_player(player_id);

       if current_player == None {
            return Err("Invalid player!");
       }      

       let mut current_player = current_player.unwrap();

        // only allow cards in player hand
       for card in &cards {
           if !current_player.get_hand().contains(&card){
                return Err("Cannot play cards you do not have");
           }
       }

       let mut players = self.players.clone();
       let round = match self.round.play(player_id, p_move.unwrap()){
            Ok(r) => {
                current_player = current_player.remove(&cards);
                players = self.replace_current_player(&current_player);
                let player_ids = self.get_players_for_next_round(&players);
                r.update_players(player_ids)
            },
            Err(r) => r
       };


       let winner = self.get_winner(&current_player);

       Ok(GameDefinition{
          players: players.clone(),
          round: round,
          winner: winner
       })
       
    }
  
    /// get a player for querying information
    pub fn get_player(&self, id: i32) -> Option<Player> {
       self.get_current_player(id)
    }

    /// get the next player to play
    pub fn get_next_player(&self) -> Option<Player> {
        let three_of_clubs = card!(Three, Clubs);
        
        for player in &self.players {
            if player.get_hand().contains(&three_of_clubs){
                return Some(player.clone());
            }
        }

        let id = self.round.get_next_player();

        self.get_player(id)
    }

    fn get_empty_round() -> Round {
        Round::new(vec!(0, 1), 0, Move::Pass, 0, false)
    }

    fn get_winner(&self, current_player: &Player) -> Option<i32> {
        if current_player.get_hand().len() == 0 {
            Some(current_player.get_id())
       }else{
            self.winner
       }
    }

    fn get_current_player(&self, player_id:i32) -> Option<Player> {

       for player in &self.players {
            if player.get_id() == player_id {
                return Some(player.clone());
            }
       }

       None
    }

    fn replace_current_player(&self, current_player: &Player) -> Vec<Player> {

        let mut players = vec!();
        for player in &self.players{
            if player.get_id() == current_player.get_id(){
                players.push(current_player.clone());
            }else{
                players.push(player.clone()); 
            }
        }

        players

    }

    fn get_players_for_next_round(&self, players: &Vec<Player>) -> Vec<i32> {

        players.iter()
            .filter(|player|{ player.get_hand().len() > 0 })
            .map(|player|{ player.get_id() }).collect()
    }
}
