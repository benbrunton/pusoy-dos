use cards::card::{PlayerCard, Card};

/// A player
#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Player{
    hand: Vec<PlayerCard>,
    id: u64
}

impl Player{
    
    /// creates a new `Player`
    pub fn new(id: u64) -> Player{

        Player{
            hand: vec!(),
            id: id
        }
    }

    /// get the player id
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// give a player their hand
    pub fn set_hand(&self, hand:Vec<PlayerCard>) -> Player {
        Player{
            hand: hand.clone(),
            id: self.id
        }
    }

    /// number of cards player has left
    pub fn remaining_cards(&self) -> usize {
       self.hand.len() 
    }

    /// get the cards for a player
    pub fn get_hand(&self) -> Vec<PlayerCard> {
        self.hand.clone()
    }

    /// take some cards from a player
    pub fn remove(&self, cards:&Vec<PlayerCard>) -> Player {
        let new_hand = self.hand.iter().filter(|&card| {
           !cards.contains(card) 
        }).map(|&card|{
            card.clone()  
        }).collect();

        Player {
            id: self.id,
            hand: new_hand
        }
    }

}
