use game::player_move::Move;
use cards::card::{ Card, PlayerCard };
use cards::types::{Rank, Suit};

/// definition
#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct RoundDefinition{
    pub players: Vec<u64>,
    pub current_player: u64,
    pub last_move: Move,
    pub pass_count: i64,
    pub first_round: bool
}

/// single round
#[derive(Clone, Debug, PartialEq)]
pub struct Round{
    players: Vec<u64>,
    current_player: u64,
    last_move: Move,
    pass_count: i64,
    first_round: bool
}

impl Round {

    /// create new round
    pub fn new(players: Vec<u64>, 
                current_player: u64, 
                last_move: Move, 
                passes: i64,
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

        if self.last_move == Move::Pass && new_move == Move::Pass {
            return Err(self.clone());
        }

        let next_player = self.determine_next_player();
        if self.last_move == Move::Pass || new_move == Move::Pass {

            let pass_count = if new_move == Move::Pass {
                self.pass_count + 1
            } else {
                0
            };

            // this essentially passes the exiting players move to the next player
            // giving them the benefit of starting if everyone passes
            let last_move = if self.pass_count == -1 && new_move == Move::Pass {
                self.last_move
            } else if pass_count  >= self.players.len() as i64 - 1 {
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

            // if unbeatable -- set last move to pass and current_player to current_player
            // .. next_player = player_id
            // .. move_on_table = Move::Pass

            let (next_current, move_on_table) = if self.is_unbeatable_move(new_move) {
                (player_id, Move::Pass)
            } else {
                (next_player, new_move)
            };

            Ok(Round{
                players: self.players.clone(), 
                current_player: next_current,
                last_move: move_on_table,
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

    pub fn reverse_last_move(&self) -> Round {
       Round::new(self.players.clone(), self.current_player, self.last_move.reverse(), self.pass_count, self.first_round) 
    }
    
    pub fn set_pass_count(&self, c:i64) -> Round {
        Round::new(self.players.clone(), self.current_player, self.last_move, c, self.first_round)
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
            if *card == card!(Three, Clubs).to_card(){
                return true;
            }
        }

        return false;
    }

    // TODO - ultimate edge case of 5 of a kind with reversed 3s or re-reversed 2s
    fn is_unbeatable_move(&self, new_move: Move) -> bool {
        let top_two = card!(Two, Spades).to_card();
        let bottom_three = card!(Three, Clubs).to_card();
        // todo Wildcard
        match new_move {
            Move::Single(x) => { x == top_two || x == bottom_three },
            Move::Pair(x, y) => { x == top_two || y == top_two || x == bottom_three || y == bottom_three },
            Move::Prial(x, y, z) => { x == top_two || y == top_two || x == bottom_three || y == bottom_three || z == top_two || z == bottom_three},
            _ => false
        }
    }
}
