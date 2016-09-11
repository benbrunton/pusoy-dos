use game::player::Player;
use cards::deck::Deck;

/// The Game module
pub struct Game<'a> { 
    players: Vec<&'a mut Player>,
    deck: Deck    
}

impl<'a> Game<'a>{

    pub fn new() -> Game<'a> {
        Game{
            players: vec!(),    
            deck: Deck::new()
        }
    }

    /// returns index of latest added player
    pub fn add_player(&mut self, player: &'a mut Player) -> usize{
        self.players.push(player);
        self.players.len() - 1
    }

    /// returns a clone of a player for checking
    pub fn check_player(&self, n: usize) -> Player{
        self.players[n].clone()
    }

    pub fn add_deck(&mut self, deck: &Deck){
        self.deck = deck.clone();
    }

    pub fn start(&mut self){

        let mut player_index = 0;

        while let Some(card) = self.deck.deal() {
            self.players[player_index].receive(card);

            player_index = player_index + 1;
            if player_index >= self.players.len() {
                player_index = 0;
            }
        }
    }

}
