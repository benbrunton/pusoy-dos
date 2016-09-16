use cards::card::Card;

/// A player
#[derive(Clone, Debug, PartialEq)]
pub struct Player{
    hand: Vec<Card>
}

impl Player{
    
    /// creates a new `Player`
    pub fn new() -> Player{

        Player{
            hand: vec!()
        }
    }

    /// give a player their hand
    pub fn set_hand(&self, hand:Vec<Card>) -> Player {
        Player{
            hand: hand.clone()
        }
    }

    /// number of cards player has left
    pub fn remaining_cards(&self) -> usize {
       self.hand.len() 
    }

    /// get the cards for a player
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

}
