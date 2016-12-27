use game::round::Round;
use game::player::Player;
use cards::deck::Deck;
use cards::types::{Rank, Suit};
use cards::card::{PlayerCard, Card};
use game::player_move::{Move, TrickType, build_move};

/// A definition of a game in progress
#[derive(Clone, Debug)]
pub struct GameDefinition{
    /// players
    pub players: Vec<Player>,
    /// round
    pub round: Round,
    /// order of winners
    pub winners: Vec<u64>,
    pub reversed: bool
}

/// The Game module
pub struct Game { 
    players: Vec<Player>,
    round: Round,
    winners: Vec<u64>,
    reversed: bool
}

impl Game{
 
    /// create a new Game
    pub fn setup(player_ids:Vec<u64>) -> Result<GameDefinition, &'static str>{
        let mut deck = Deck::new();

        deck.shuffle();

        let player_count = player_ids.len();

        let dealt_cards = deck.deal(player_count);
        let mut cards_iter = dealt_cards.iter();

        let mut players = vec!();

        for id in player_ids.iter() {
            let player = Player::new(*id);
            let hand = cards_iter.next().unwrap();
            players.push(player.set_hand(hand.clone()));
        }

        let next_player = Game::get_next(&players).unwrap().get_id();

        Ok(
            GameDefinition{
                players: players,
                round: Game::get_empty_round(player_ids.clone(), next_player),
                winners: vec!(),
                reversed: false
            }
        )
    }

    /// load an existing game from a `GameDefinition`
    pub fn load(game_definition: GameDefinition) -> Result<Game, &'static str>{

        Ok(
            Game{
                players: game_definition.players,
                round: game_definition.round, 
                winners: game_definition.winners,
                reversed: game_definition.reversed
            }
        )
    }

    /// takes a player_id and a vec of cards for a move
    pub fn player_move(&self, player_id:u64, cards:Vec<PlayerCard>) -> Result<GameDefinition, &'static str> {
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
        if !self.player_has_card(&current_player, cards.clone()) {
            return Err("Cannot play cards you do not have");
       
        }

       let mut players = self.players.clone();
       let mut round = match self.round.play(player_id, p_move.unwrap()){
            Ok(r) => {
                current_player = current_player.remove(&cards);
                players = self.replace_current_player(&current_player);
                let player_ids = self.get_players_for_next_round(&players);
                r.update_players(player_ids)
            },
            Err(r) => r
       };
       let mut reversed = self.reversed;


       // check for Four of a kind / Five of a kind and reverse cards
       // this means manually switching the cards in the players hands
       // and the last played hand on the round
        match p_move {
            Some(Move::FiveCardTrick(t)) => match t.trick_type {
               TrickType::FourOfAKind
               | TrickType::FiveOfAKind => {
                    // update round to reverse last_move cards
                    round = round.reverse_last_move();
                    // update players to reverse cards in hand    
                    players = players.iter().map(|ref p|{ p.reverse_hand() }).collect::<Vec<Player>>(); 
                    reversed = !self.reversed;
               },
               _ => ()
            },
            _ => ()
            
        }


       let winners = self.get_winners(&current_player);

       Ok(GameDefinition{
          players: players.clone(),
          round: round,
          winners: winners,
          reversed: reversed
       })
       
    }
  
    /// get a player for querying information
    pub fn get_player(&self, id: u64) -> Option<Player> {
       self.get_current_player(id)
    }

    /// get the next player to play
    pub fn get_next_player(&self) -> Option<Player> {

        let id = match Game::get_next(&self.players) {
            Some(player) => player.get_id(),
            _ => self.round.get_next_player()
        };

        self.get_player(id)
    }

    fn get_next(players: &Vec<Player>) -> Option<Player> {
        let three_of_clubs = card!(Three, Clubs);
        
        for player in players {
            if player.get_hand().contains(&three_of_clubs){
                return Some(player.clone());
            }
        }

        None
    }

    fn get_empty_round(player_ids:Vec<u64>, next:u64) -> Round {
        Round::new(player_ids, next, Move::Pass, 0, true)
    }

    fn get_winners(&self, current_player: &Player) -> Vec<u64> {
        let mut winners = self.winners.clone();
        if current_player.get_hand().len() == 0 {
            winners.push(current_player.get_id());
       }
       winners
    }

    fn get_current_player(&self, player_id:u64) -> Option<Player> {

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

    fn get_players_for_next_round(&self, players: &Vec<Player>) -> Vec<u64> {

        players.iter()
            .filter(|player|{ player.get_hand().len() > 0 })
            .map(|player|{ player.get_id() }).collect()
    }
    
    fn player_has_card(&self, player:&Player, cards:Vec<PlayerCard>) -> bool { 
        for card in &cards {
            match *card {
                PlayerCard::Card(_) => {
                    let hand = player.get_hand();
                    let reversed_hand = player.reverse_hand().get_hand();
                    if !hand.contains(&card) && !reversed_hand.contains(&card){
                        return false;
                    }
                },
                PlayerCard::Wildcard(_) => {
                    // check player has a joker
                },
                PlayerCard::Joker(_) => { panic!("Joker is not a valid move - do you mean Wildcard?"); }
            }
        }

        true
    }
}
