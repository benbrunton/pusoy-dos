use cards::card::Card;
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

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd, RustcDecodable, RustcEncodable)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, RustcDecodable, RustcEncodable)]
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

impl PartialOrd for Trick {

    fn partial_cmp(&self, other: &Trick) -> Option<Ordering> {

        match (self.trick_type, other.trick_type) {
            (TrickType::FullHouse, TrickType::FullHouse) => compare_full_house(self, other),
            _ if self.trick_type > other.trick_type   =>  Some(Ordering::Greater),
			_ if self.trick_type < other.trick_type   =>  Some(Ordering::Less), 
			_  						=>  Some(Ordering::Equal) 
        }

    }
}

// todo - this is super simple
fn compare_full_house(self_trick:&Trick, other:&Trick) -> Option<Ordering> {

	let counts = get_counts(self_trick.cards.to_vec());
	let mut self_rank = Rank::Three;
	for (rank, count) in &counts {
		if *count == 3 {
			self_rank = *rank;
		}	
	}		

	let counts = get_counts(other.cards.to_vec());
	let mut other_rank = Rank::Three;
	for (rank, count) in &counts {
		if *count == 3 {
			other_rank = *rank;
		}	
	}

	Some(self_rank.cmp(&other_rank))
}


/// builds a move from a Vec of cards
pub fn build_move(cards: Vec<Card>) -> Option<Move> {

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
