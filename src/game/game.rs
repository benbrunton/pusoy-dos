use game::player::Player;
use cards::deck::Deck;

/// A definition of a game in progress
pub struct GameDefinition{
    /// players
    pub players: Vec<Player>
}

/// The Game module
pub struct Game { 
    players: Vec<Player>
}

impl Game{
 
    /// create a new Game
    pub fn setup(player_count:usize) -> Result<Game, &'static str>{
        let deck = Deck::new();

        let dealt_cards = deck.deal(player_count);

        let players:Vec<Player> = (0..player_count).map(|n: usize| {
            let player = Player::new();
            player.set_hand(dealt_cards[n].clone())
        }).collect();

        Ok(
            Game{
                players: players.clone()
            }
        )
    }

    /// load an existing game from a `GameDefinition`
    pub fn load(game_definition: GameDefinition) -> Result<Game, &'static str>{

        Ok(
            Game{
                players: game_definition.players
            }
        )
    }
  
    /// get a player for querying information
    pub fn get_player(&self, n: usize) -> Option<&Player> {
       self.players.get(n)
    }

}
