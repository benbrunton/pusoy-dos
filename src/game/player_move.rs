use cards::card::Card;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Copy)]
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

    if cards.len() != 2 {
        return None
    }
    
    let card1 = cards[0];
    let card2 = cards[1];
    
    if card1.rank == card2.rank {
        Some(Move::Pair(card1, card2))
    } else {
        None
    }
}

fn check_valid_prial(cards: Vec<Card>) -> Option<Move> {

    if cards.len() == 3 && 
        cards[0].rank == cards[1].rank && 
        cards[1].rank == cards[2].rank{  
        Some(Move::Prial(cards[0], cards[1], cards[2]))
    } else {
        None
    }
}

fn check_valid_fct(cards: Vec<Card>) -> Option<Move> {

    if cards[0].rank == cards[1].rank
        && cards[1].rank == cards[2].rank
        && cards[2].rank == cards[3].rank
        && cards[3].rank == cards[4].rank {
        return build_five_card_trick(cards);
    }

    let mut hm = HashMap::new();
    for card in &cards {
        hm.insert(card.rank, 0);
    }
    
    let count_types = cards.iter().fold(hm, |acc, &card|{
        let mut output = HashMap::new();
        for (rank, count) in &acc {
            let c = if *rank == card.rank {
                *count + 1
            } else {
                *count
            };
            output.insert(*rank, c);
        }
        output
    });

   for (_, count) in &count_types {
       match *count{
          4 => {
              return build_five_card_trick(cards);
          },
          _ => ()
       }
   }

   if cards[0].suit == cards[1].suit
        && cards[1].suit == cards[2].suit
        && cards[2].suit == cards[3].suit
        && cards[3].suit == cards[4].suit {
            return build_five_card_trick(cards);
    }

    if cards[0].next_rank().unwrap() == cards[1].rank
        && cards[1].next_rank().unwrap() == cards[2].rank
        && cards[2].next_rank().unwrap() == cards[3].rank
        && cards[3].next_rank().unwrap() == cards[4].rank {
            return build_five_card_trick(cards);
    }

    None
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

