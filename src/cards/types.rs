use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
/// Card suit
pub enum Suit{
    /// ♦
    Diamonds,
    /// ♣
    Clubs,
    /// ♥
    Hearts,
    /// ♠
    Spades
}

impl fmt::Display for Suit{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self {
            &Suit::Diamonds    => "♦",
            &Suit::Clubs       => "♣",
            &Suit::Hearts      => "♥",
            &Suit::Spades      => "♠"
        };
        write!(f, "{}", icon)
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
/// Card colour
pub enum Colour{
    /// red
    Red,
    /// black
    Black
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
/// Rank of a card
pub enum Rank{
    /// 3
    Three, 
    /// 4
    Four, 
    /// 5
    Five, 
    /// 6
    Six, 
    /// 7
    Seven, 
    /// 8
    Eight, 
    /// 9
    Nine, 
    /// 10
    Ten, 
    /// Jack
    Jack, 
    /// Queen
    Queen, 
    /// King
    King,
    /// Ace
    Ace, 
    /// 2 
    Two
}

impl fmt::Display for Rank{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            Rank::Ace      => "A",
            Rank::Two      => "2",
            Rank::Three    => "3",
            Rank::Four     => "4",
            Rank::Five     => "5",
            Rank::Six      => "6",
            Rank::Seven    => "7",
            Rank::Eight    => "8",
            Rank::Nine     => "9",
            Rank::Ten      => "10",
            Rank::Jack     => "J",
            Rank::Queen    => "Q",
            Rank::King     => "K",
        };
        write!(f, "{}", display)
    }
}



