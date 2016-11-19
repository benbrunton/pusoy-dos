use rand;
use rand::Rng;

use cards::types::*;
use cards::card::{Card, PlayerCard};

/// a fresh deck of cards
#[derive(Clone, Debug)]
pub struct Deck(Vec<PlayerCard>);

impl Deck {
    /// create a new deck
    pub fn new() -> Deck {
        let mut cards:Vec<PlayerCard> = Vec::with_capacity(52);
        for suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in &[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, 
                Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
                let c = Card::new(rank.clone(), suit.clone());
                cards.push( PlayerCard::Card(c) );
            }
        }
        cards.push(PlayerCard::Joker(0));
        cards.push(PlayerCard::Joker(1));
        Deck(cards)
    }

    /// combine 2 decks of cards
    pub fn combine(decks: Vec<Deck>) -> Deck{

        let mut new_stack = vec!();

        for deck in decks.iter() {
            new_stack.extend(deck.0.iter().cloned());
        }

        Deck(new_stack)
    }

    /// deal to a number of players
    pub fn deal(&self, players: usize) -> Vec<Vec<PlayerCard>> {
        let mut dealt_stacks = vec!();
        
        while dealt_stacks.len() < players {
            dealt_stacks.push(vec!());
        }

        let mut index = 0;

        let mut deck_stack = self.0.clone();

        while deck_stack.len() > 0 {

           let card = deck_stack.pop(); 
           dealt_stacks[index].push(card.unwrap());

           index = if (index + 1) < players {
               index + 1
           } else {
               0
           }

        }

        dealt_stacks

    }
    
    /// rearrange the cards
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.0)
    }
    
    /// number of cards in the deck
    pub fn count(&self) -> usize {
        self.0.len()
    }
}
