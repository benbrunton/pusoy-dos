use std::fmt;
use std::cmp::Ordering;

use cards::types::*;

#[macro_export]
macro_rules! card {
    ($rank:ident, $suit:ident) => (PlayerCard::Card(Card::new(Rank::$rank, Suit::$suit, false)));
    ($rank:ident, $suit:ident, $reverse:expr) => (PlayerCard::Card(Card::new(Rank::$rank, Suit::$suit, $reverse)));
}

#[macro_export]
macro_rules! wildcard {
    ($rank:ident, $suit:ident) => (PlayerCard::Wildcard(Card::new(Rank::$rank, Suit::$suit, false)));
    ($rank:ident, $suit:ident, $reverse:expr) => (PlayerCard::Wildcard(Card::new(Rank::$rank, Suit::$suit, $reverse)));
}



#[derive(Clone, Debug, PartialEq, Copy, PartialOrd, RustcDecodable, RustcEncodable, Eq, Ord)]
/// A Wrapper type that holds Real Cards and Jokers
pub enum PlayerCard {
   Card(Card),
   Wildcard(Card),
   Joker(u64)
}

impl PlayerCard {
    
    pub fn to_card(&self) -> Card {
        match *self {
            PlayerCard::Card(c)|PlayerCard::Wildcard(c) => c,
            PlayerCard::Joker(_) => panic!("Joker must be specified as Wildcard(Card)!")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, RustcDecodable, RustcEncodable, Eq, Ord)]
/// An individual card
pub struct Card{
    /// The `Rank` of the card
    pub rank: Rank,
    /// The `Suit` of the card
    pub suit: Suit,
    /// This is linked to the `Suit` but
    /// is also explicitly stored here
    pub colour: Colour,
    reversed: bool
}

impl Card {

    /// returns a new `Card`
    pub fn new(rank: Rank, suit: Suit, reversed: bool) -> Card {
        let colour = match suit {
            Suit::Diamonds | Suit::Hearts   => Colour::Red,
            _                               => Colour::Black
        };
        Card{suit: suit, rank: rank, colour: colour, reversed: reversed}
    }

    /// returns previous `Rank` of card or `None`
    pub fn previous_rank(&self) -> Option<Rank>{
        previous_rank(&self.rank)
    }

    /// returns next `Rank` of this card or `None`
    pub fn next_rank(&self) -> Option<Rank>{
        next_rank(&self.rank)
    }

    /// returns the alternate `Colour` to this card
    pub fn alternate_colour(&self) -> Colour{
        if self.colour == Colour::Red {
            Colour::Black
        } else {
            Colour::Red
        }
    }
}

impl PartialOrd for Card {

    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {

        if self.reversed {
            match other.rank.partial_cmp(&self.rank) {
                Some(Ordering::Equal) => other.suit.partial_cmp(&self.suit),
                x                     => x
            }
        } else {
            match self.rank.partial_cmp(&other.rank) {
                Some(Ordering::Equal) => self.suit.partial_cmp(&other.suit),
                x                     => x
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card = format!("{}{}", self.suit, self.rank);
        write!(f, "{}", card)
    }
}




fn previous_rank(rank:&Rank) -> Option<Rank> {
    match *rank {
        Rank::Three    => None,
        Rank::Four     => Some(Rank::Three),
        Rank::Five     => Some(Rank::Four),
        Rank::Six      => Some(Rank::Five),
        Rank::Seven    => Some(Rank::Six),
        Rank::Eight    => Some(Rank::Seven),
        Rank::Nine     => Some(Rank::Eight),
        Rank::Ten      => Some(Rank::Nine),
        Rank::Jack     => Some(Rank::Ten),
        Rank::Queen    => Some(Rank::Jack),
        Rank::King     => Some(Rank::Queen),
        Rank::Ace      => Some(Rank::King),
        Rank::Two      => Some(Rank::Ace),
    }
}

fn next_rank(rank:&Rank) -> Option<Rank> {
    match *rank {
        Rank::Three    => Some(Rank::Four),
        Rank::Four     => Some(Rank::Five),
        Rank::Five     => Some(Rank::Six),
        Rank::Six      => Some(Rank::Seven),
        Rank::Seven    => Some(Rank::Eight),
        Rank::Eight    => Some(Rank::Nine),
        Rank::Nine     => Some(Rank::Ten),
        Rank::Ten      => Some(Rank::Jack),
        Rank::Jack     => Some(Rank::Queen),
        Rank::Queen    => Some(Rank::King),
        Rank::King     => Some(Rank::Ace),
        Rank::Ace      => Some(Rank::Two),
        Rank::Two      => None,
    }
}

