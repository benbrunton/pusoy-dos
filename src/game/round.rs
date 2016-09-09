use cards::card::Card;
use cards::types::*;
use game::player_move::Move;

#[derive(Clone, Debug, PartialEq)]
pub struct Round{
    players: Vec<i32>,
    current_player: i32,
    last_move: Move
}

impl Round {

    pub fn new(players: Vec<i32>, current_player: i32, last_move: Move) -> Round {

        if !players.contains(&current_player){
            panic!("current player needs to be in the pool of players");
        }

        Round{
            players: players,
            current_player: current_player,
            last_move: last_move
        }
    }

    /// play a move in the current round
    pub fn play(&self, player_id: i32, new_move: Move) -> Result<Round, Round> {

        let next_player = if self.current_player == *self.players.last().unwrap() {
            self.players.first().unwrap()
        } else {
            let index = self.players.binary_search(&self.current_player).unwrap();
            self.players.get(index + 1).unwrap()
        };
        
        if new_move == Move::Pass {
        
            Ok(Round{
                players: self.players.clone(), 
                current_player: *next_player,
                last_move: Move::Pass
            })

        } else {
            Err(self.clone())
        }
    }

    pub fn get_next_player(&self) -> i32 {
        self.current_player
    }
}
