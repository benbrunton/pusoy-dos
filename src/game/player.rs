use cards::card::Card;

/// A player
pub struct Player{
    /// id
    pub id: u32,
    hand: Vec<Card>
}

impl Player{
    
    /// creates a new `Player`
    pub fn new() -> Player{
        let id = 100;

        Player{
            id: id,
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

}
