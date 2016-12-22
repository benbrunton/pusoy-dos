use cards::card::PlayerCard;

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

    pub fn reverse_hand(&self) -> Player {
        let reversed_hand = self.hand.iter().map(|&c|{c.reverse()}).collect::<Vec<PlayerCard>>();
        Player{
            hand: reversed_hand,
            id: self.id
        }
    }

    /// take some cards from a player
    pub fn remove(&self, cards:&Vec<PlayerCard>) -> Player {

        let hand = self.remove_jokers(cards);

        let new_hand = hand.iter().filter(|&card| {
           !cards.contains(card) 
        }).map(|&card|{
            card.clone()  
        }).collect();

        Player {
            id: self.id,
            hand: new_hand
        }
    }

    pub fn remove_jokers(&self, cards:&Vec<PlayerCard>)-> Vec<PlayerCard> {
        let mut new_hand = vec!();
        let mut jokers = 0;

        for card in cards.iter() {
            match *card {
                PlayerCard::Wildcard(_) => jokers += 1,
                _ => ()
            }
        }

        for card in self.hand.iter() {
            match *card {
                PlayerCard::Joker(n) => {
                    if jokers < 1 { 
                        new_hand.push(PlayerCard::Joker(n));
                    }else {
                        jokers -= 1;
                    }
                },
                c => new_hand.push(c.to_owned())
            }
        }

        new_hand

    }

}
