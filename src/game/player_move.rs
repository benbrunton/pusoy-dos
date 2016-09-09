use cards::card::Card;
use cards::types::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd)]
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
    FiveCardTrick(Card, Card, Card, Card, Card)
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

    if cards.len() == 2 && get_counts(cards.clone()).len() == 1 {
        Some(Move::Pair(cards[0], cards[1]))
    } else {
        None
    }
}

fn check_valid_prial(cards: Vec<Card>) -> Option<Move> {

    if cards.len() == 3 && get_counts(cards.clone()).len() == 1 {
        Some(Move::Prial(cards[0], cards[1], cards[2]))
    } else {
        None
    }
}

fn check_valid_fct(cards: Vec<Card>) -> Option<Move> {

    let rank_count = get_counts(cards.clone());
    match rank_count.len() {
        1 | 2 => build_five_card_trick(cards), //five of a kind, full house or four of a kind
        5 => {
            //flush or straight or straight flush
           if cards[0].suit == cards[1].suit
                && cards[1].suit == cards[2].suit
                && cards[2].suit == cards[3].suit
                && cards[3].suit == cards[4].suit {
                    build_five_card_trick(cards)
            } else if cards[0].next_rank().unwrap() == cards[1].rank
                && cards[1].next_rank().unwrap() == cards[2].rank
                && cards[2].next_rank().unwrap() == cards[3].rank
                && cards[3].next_rank().unwrap() == cards[4].rank {
                    build_five_card_trick(cards)
            }else {
                None
            }
        },
        _ => None
    }
}

fn get_counts(cards: Vec<Card>) -> HashMap<Rank, usize> {
    cards.iter().fold(HashMap::new(), |mut acc, &card| {
        *acc.entry(card.rank).or_insert(0) += 1;
        acc
    })
}


fn build_five_card_trick(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4])),
        _ => None
    }
}

