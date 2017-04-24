use cards::card::{ Card, PlayerCard };
use cards::types::*;
use std::collections::HashMap;

use std::cmp::Ordering;

macro_rules! build_fct {
    ($trick:ident, $cards:ident) => (Some(Move::FiveCardTrick(
						Trick{
							trick_type:TrickType::$trick,
							cards:[$cards[0], $cards[1], $cards[2], $cards[3], $cards[4]]
						})));
}

#[derive(Clone, Debug, PartialEq, Copy, RustcDecodable, RustcEncodable)]
/// Type of hand that can be played
pub enum Move{
    /// No cards
    Pass,
    /// One card
    Single(Card),
    /// A pair of matching cards
    Pair(Card, Card),
    /// 3 of a kind
    Prial(Card, Card, Card),
    /// 5 card trick
    FiveCardTrick(Trick)
}

impl Move {
    pub fn reverse(&self) -> Move {
       match *self {
          Move::Pass => Move::Pass,
          Move::Single(c) => Move::Single(c.reverse()),
          Move::Pair(a, b) => Move::Pair(a.reverse(), b.reverse()),
          Move::Prial(a, b, c) => Move::Prial(a.reverse(), b.reverse(), c.reverse()),
          Move::FiveCardTrick(t) => Move::FiveCardTrick(t.reverse())
       }
    }

    fn is_pass(&self) -> bool {
        Move::Pass == *self
    }

    fn is_single(&self) -> bool {
        match *self {
            Move::Single(_) => true,
            _         => false
        }
    }

    fn is_pair(&self) -> bool {
        match *self {
            Move::Pair(_, _) => true,
            _          => false
        }
    }

    fn is_prial(&self) -> bool {
        match *self {
            Move::Prial(_, _, _) => true,
            _              => false
        }
    }

    fn is_five_card_trick(&self) -> bool {
        match *self {
            Move::FiveCardTrick(_) => true,
            _                => false
        }
    }

    fn get_five_card_trick(&self) -> Option<Trick> {
        match *self {
            Move::FiveCardTrick(t) => Some(t),
            _                      => None
        }
    }

    fn get_top_card(&self) -> Card {
        match *self {
            Move::Pass => panic!("pass has no top card!"),
            Move::Single(a) => get_max_card(vec!(a)),
            Move::Pair(a, b) => get_max_card(vec!(a, b)),
            Move::Prial(a, b, c) => get_max_card(vec!(a, b, c)),
            Move::FiveCardTrick(t) => get_max_card(t.cards.to_vec())
        }
    }

    fn types_match(&self, other: &Move) -> bool {
        match *self {
            Move::Pass => other.is_pass(),
            Move::Single(_) => other.is_single(),
            Move::Pair(_,_) => other.is_pair(),
            Move::Prial(_, _, _) => other.is_prial(),
            Move::FiveCardTrick(_) => other.is_five_card_trick()
        }
    }

}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {

        if !self.types_match(other) {
            return None;
        }

        match *self {
            Move::Pass =>  Some(Ordering::Equal),
            Move::FiveCardTrick(t) => t.partial_cmp(&other.get_five_card_trick().unwrap()),
            _ => self.get_top_card().partial_cmp(&other.get_top_card())
        }
        
    }

}


#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, RustcDecodable, RustcEncodable)]
/// Type of 5 card trick
pub enum TrickType{
    /// sequence
    Straight,
    /// same suit
    Flush,
    /// 3 over 2
    FullHouse,
    /// 4 of same, 1 different
    FourOfAKind,
    /// sequence of same suit
    StraightFlush,
    /// 5 of same
    FiveOfAKind,
}

#[derive(Clone, Debug, PartialEq, Copy, RustcDecodable, RustcEncodable)]
pub struct Trick{
	pub trick_type: TrickType,
	pub cards: [Card;5]	
}

impl Trick {
    pub fn reverse(&self) -> Trick {
        let cards = [
            self.cards[0].reverse(),
            self.cards[1].reverse(),
            self.cards[2].reverse(),
            self.cards[3].reverse(),
            self.cards[4].reverse()
        ];

        Trick {
            trick_type: self.trick_type,
            cards: cards
        }
    }
}

impl PartialOrd for Trick {

    fn partial_cmp(&self, other: &Trick) -> Option<Ordering> {

        if self.trick_type == other.trick_type {
            match self.trick_type {
                TrickType::Flush 
                    | TrickType::FiveOfAKind
                    | TrickType::Straight => compare_top_card(self, other),
                TrickType::FullHouse => compare_main_set(self, other, 3),
                TrickType::FourOfAKind => compare_main_set(self, other, 4),
                _ => self.cards.partial_cmp(&other.cards)
            }
        } else {
            self.trick_type.partial_cmp(&other.trick_type)
        }

    }
}

fn compare_main_set(this:&Trick, other:&Trick, n:usize) -> Option<Ordering> {
    let this_top_card = get_top_of_n(this.cards.to_vec(), n);
    let other_top_card = get_top_of_n(other.cards.to_vec(), n);
	this_top_card.partial_cmp(&other_top_card)
}

fn compare_top_card(this:&Trick, other:&Trick) -> Option<Ordering> {
    let top_this = get_max_card(this.cards.to_vec());
    let top_other = get_max_card(other.cards.to_vec());

    top_this.partial_cmp(&top_other)
}

fn get_max_card(cards:Vec<Card>) -> Card{
    let mut c = sort_cards(cards);

    // For some reason, this needs removing in the beta and nightly
    // builds of rust. My *guess* is that the Vec.sort method is not
    // respecting the custom Card order which means we have to make this check
    // for the reversed case. There are tests that will cover this.
    if !c.get(0).unwrap().reversed {
        c.reverse();
    }

    c.first().unwrap().to_owned()
}

fn get_top_of_n(cards: Vec<Card>, n:usize) -> Card{

    let counts = get_counts(cards.clone());
    let mut top_rank = Rank::Three;
    
	for (rank, count) in &counts {
		if *count == n {
            top_rank = *rank;
		}
	}


    let valid_cards:Vec<Card> = cards.iter()
                .filter(|&c|{ c.rank == top_rank })
                .map(|&c|{ c.clone() }).collect();

    get_max_card(valid_cards)
}


/// builds a move from a Vec of cards
pub fn build_move(player_cards: Vec<PlayerCard>) -> Option<Move> {

    let cards:Vec<Card> = player_cards.iter().map(| ref card | { card.to_card() }).collect();

    match cards.len() {
        0 => Some(Move::Pass),
        1 => Some(Move::Single(cards[0])),
        2 => check_valid_pair(cards),
        3 => check_valid_prial(cards),
        5 => check_valid_fct(cards),
        _ => None
    }
}

fn check_valid_pair(cards: Vec<Card>) -> Option<Move> {

    if get_counts(cards.clone()).len() == 1 {
        Some(Move::Pair(cards[0], cards[1]))
    } else {
        None
    }
}

fn check_valid_prial(cards: Vec<Card>) -> Option<Move> {

    if get_counts(cards.clone()).len() == 1 {
        Some(Move::Prial(cards[0], cards[1], cards[2]))
    } else {
        None
    }
}

fn check_valid_fct(c: Vec<Card>) -> Option<Move> {

    let cards = sort_cards(c);
    let rank_count = get_counts(cards.clone());
    match rank_count.len() {
        1 => build_fct!(FiveOfAKind, cards),
        2 => {
           match *rank_count.values().last().unwrap() {
                3 | 2   => build_fct!(FullHouse, cards),
                4 | 1   => build_fct!(FourOfAKind, cards),
                _       => None

           }
        },
        5 => {
            //flush or straight or straight flush
            let straight = cards.iter().enumerate().all(|(i, &card)| i == 0 || card.previous_rank().is_some() && cards[i-1].rank == card.previous_rank().unwrap());
            let flush = cards.iter().all(|&card| card.suit == cards[0].suit);
            match (straight, flush) {
                (true, true)    => build_fct!(StraightFlush, cards),
                (true, _)       => build_fct!(Straight, cards),
                (_, true)       => build_fct!(Flush, cards),
                _               => None
            }
        },
        _ => None
    }
}

fn sort_cards(cards: Vec<Card>) -> Vec<Card> {
    let mut c = cards.clone();
    c.sort();
    c
}

fn get_counts(cards: Vec<Card>) -> HashMap<Rank, usize> {
    cards.iter().fold(HashMap::new(), |mut acc, &card| {
        *acc.entry(card.rank).or_insert(0) += 1;
        acc
    })
}
