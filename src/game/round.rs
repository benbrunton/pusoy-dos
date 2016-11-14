use game::player_move::{ Move, Trick};
use cards::card::Card;
use cards::types::{Rank, Suit};

/// definition
#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct RoundDefinition{
    pub players: Vec<u64>,
    pub current_player: u64,
    pub last_move: Move,
    pub pass_count: u64,
    pub first_round: bool
}

/// single round
#[derive(Clone, Debug, PartialEq)]
pub struct Round{
    players: Vec<u64>,
    current_player: u64,
    last_move: Move,
    pass_count: u64,
    first_round: bool
}

impl Round {

    /// create new round
    pub fn new(players: Vec<u64>, 
                current_player: u64, 
                last_move: Move, 
                passes: u64,
                first_round: bool) -> Round {

        if !players.contains(&current_player){
            panic!("current player needs to be in the pool of players");
        }

        Round{
            players: players,
            current_player: current_player,
            last_move: last_move,
            pass_count: passes,
            first_round: first_round
        }
    }

    /// play a move in the current round
    pub fn play(&self, player_id: u64, new_move: Move) -> Result<Round, Round> {

        if player_id != self.current_player {
            return Err(self.clone());
        }

        if self.first_round && !self.includes_three(new_move){
            return Err(self.clone());
        }

        let next_player = self.determine_next_player();
        if self.last_move == Move::Pass || new_move == Move::Pass {

            let pass_count = if new_move == Move::Pass {
                self.pass_count + 1
            } else {
                0
            };

            let last_move = if pass_count as usize >= self.players.len() - 1 {
                Move::Pass
            } else if new_move == Move::Pass {
                self.last_move
            } else {
                new_move    
            };
           
            Ok(Round{
                players: self.players.clone(), 
                current_player: next_player,
                last_move: last_move,
                pass_count: pass_count,
                first_round: false
            })
 
        } else if self.valid_move(new_move) {

            Ok(Round{
                players: self.players.clone(), 
                current_player: next_player,
                last_move: new_move,
                pass_count: 0,
                first_round: false
            })
        } else {
            Err(self.clone())
        }
    }
    
    /// check who should play the next move
    pub fn get_next_player(&self) -> u64 {
        self.current_player
    }

    /// check the last move
    pub fn get_last_move(&self) -> Move {
        self.last_move
    }

    /// export round def
    pub fn export(&self) -> RoundDefinition {
        RoundDefinition{
            players: self.players.clone(),
            current_player: self.current_player,
            last_move: self.last_move,
            pass_count: self.pass_count,
            first_round: self.first_round
        }
    }

    /// update the players in the round
    pub fn update_players(&self, players:Vec<u64>) -> Round {
        let current_player = if players.contains(&self.current_player){
            self.current_player
        } else {
            self.determine_next_player()
        };

        Round::new(players, 
            current_player, 
            self.last_move, 
            self.pass_count, 
            self.first_round)
    }
    
    fn determine_next_player(&self) -> u64 {
        if self.current_player == *self.players.last().unwrap() {
            *self.players.first().unwrap()
        } else {

            let mut index = 0;
            let mut i = 0; 

            for player in &self.players{
                i = i + 1;
                if self.current_player == *player {
                    index = i;
                }
            }

            *self.players.get(index).unwrap()
        }
    }


    fn valid_move(&self, new_move: Move) -> bool {
        let matching_type = match self.last_move {
            Move::Single(_) => {
                match new_move {
                    Move::Single(_) => true,
                    _               => false
                }
            },
            Move::Pair(_, _) => {
                match new_move {
                    Move::Pair(_,_) => true,
                    _               => false
                }
            },
            Move::Prial(_, _, _) => {
                match new_move {
                    Move::Prial(_,_,_) => true,
                    _               => false
                }
            },
            Move::FiveCardTrick(_) => {
                match new_move {
                    Move::FiveCardTrick(_) => true,
                    _               => false
                }
            },
            _ => false
        };

        matching_type && new_move > self.last_move
    }

    fn includes_three(&self, new_move: Move) -> bool {
        let cards = match new_move {
            Move::Single(x) => vec!(x),
            Move::Pair(x, y) => vec!(x, y),
            Move::Prial(x, y, z) => vec!(x, y, z),
            Move::FiveCardTrick(t) => t.cards.to_vec(),
            Move::Pass => vec!()
        };

        for card in cards.iter() {
            if *card == card!(Three, Clubs){
                return true;
            }
        }

        return false;
    }
}
