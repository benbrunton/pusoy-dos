use cards::card::Card;

/// A player
#[derive(Clone, Debug)]
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

    /// number of cards player has left
    pub fn remaining_cards(&self) -> usize {
       self.hand.len() 
    }

    /// receive a new card
    pub fn receive(&mut self, card: Card){
       self.hand.push(card); 
    }

    /// get the cards for a player
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

}
