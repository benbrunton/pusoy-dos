use rand;
use rand::Rng;

use cards::types::*;
use cards::card::Card;

/// a fresh deck of cards
pub struct Deck(Vec<Card>);

impl Deck {
    /// create a new deck
    pub fn new() -> Deck {
        let mut cards:Vec<Card> = Vec::with_capacity(52);
        for suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in &[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, 
                Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
                cards.push( Card::new(rank.clone(), suit.clone()) );
            }
        }
        Deck(cards)
    }
    
    /// take a card from the top of the deck
    pub fn deal(&mut self) -> Option<Card> {
        self.0.pop()
    }
 
    /// rearrange the cards
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.0)
    }
    
    /// remove n cards from the deck
    /// and return the new Vec of cards
    pub fn take(&mut self, n: usize) -> Vec<Card>{
    
        let mut temp_stack:Vec<Card> = Vec::new();
        while temp_stack.len() < n {
            if let Some(card) = self.deal(){
                temp_stack.push(card);
            }
        }
        
        temp_stack
    }
    
    /// number of cards in the deck
    pub fn count(&self) -> usize {
        self.0.len()
    }
    
    /// put a `Card` at the top of this `Deck`
    pub fn add_to_top(&mut self, cards: Vec<Card>){
        for card in cards {
            self.0.push(card.clone());
        }
    }
}
