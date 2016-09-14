use game::player::Player;
use cards::deck::Deck;

pub struct GameDefinition {
    pub players: Vec<Player>
}

/// The Game module
pub struct Game { 
    players: Vec<Player>
}

impl Game{

    pub fn new(game_definition: GameDefinition) -> Game {
        Game{
            players: game_definition.players.clone()
        }
    }
    
    /// get a player for querying information
    pub fn get_player(&self, n: usize) -> Option<&Player> {
       self.players.get(n)
    }

    /// create a new Game
    pub fn setup(&self) -> Result<Game, &str>{
        let mut deck = Deck::new();
        let mut player1 = Player::new();
        let mut player2 = Player::new();
        let mut index = 0;

        while let Some(card) = deck.deal() {
            if index == 0 {
                player1.receive(card);
                index = 1;
            } else {
                player2.receive(card);
                index = 0;
            }
        }

        Ok(
            Game::new(GameDefinition{
                players: vec!(player1, player2)
            })
        )
    }

}
