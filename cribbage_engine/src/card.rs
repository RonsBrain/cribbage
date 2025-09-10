#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl TryFrom<char> for Rank {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(Rank::Ace),
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            't' | 'T' => Ok(Rank::Ten),
            'j' | 'J' => Ok(Rank::Jack),
            'q' | 'Q' => Ok(Rank::Queen),
            'k' | 'K' => Ok(Rank::King),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl TryFrom<char> for Suit {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            's' | 'S' => Ok(Suit::Spades),
            'h' | 'H' => Ok(Suit::Hearts),
            'c' | 'C' => Ok(Suit::Clubs),
            'd' | 'D' => Ok(Suit::Diamonds),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl TryFrom<&str> for Card {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let (Ok(rank), Ok(suit)) = (
            Rank::try_from(value.chars().next().unwrap()),
            Suit::try_from(value.chars().nth(1).unwrap()),
        ) {
            return Ok(Self::new(rank, suit));
        }
        Err(())
    }
}
