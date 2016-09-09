use std::fmt;

use cards::types::*;

#[derive(Clone, Debug, PartialEq, Copy)]
/// An individual card
pub struct Card{
    /// The `Rank` of the card
    pub rank: Rank,
    /// The `Suit` of the card
    pub suit: Suit,
    /// This is linked to the `Suit` but
    /// is also explicitly stored here
    pub colour: Colour
}

impl Card {

    /// returns a new `Card`
    pub fn new(rank: Rank, suit: Suit) -> Card {
        let colour = match suit {
            Suit::Diamonds | Suit::Hearts   => Colour::Red,
            _                               => Colour::Black
        };
        Card{suit: suit, rank: rank, colour: colour}
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

