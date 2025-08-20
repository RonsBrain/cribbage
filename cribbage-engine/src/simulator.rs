#[derive(PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl TryFrom<char> for Suit {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            's' => Ok(Self::Spades),
            'h' => Ok(Self::Hearts),
            'c' => Ok(Self::Clubs),
            'd' => Ok(Self::Diamonds),
            _ => Err("Unknown suit"),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Rank {
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
    type Error = &'static str;
    fn try_from(rank: char) -> Result<Self, Self::Error> {
        match rank {
            'a' => Ok(Self::Ace),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            't' => Ok(Self::Ten),
            'j' => Ok(Self::Jack),
            'q' => Ok(Self::Queen),
            'k' => Ok(Self::King),
            _ => Err("Unknown rank"),
        }
    }
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err("Specifier must be two characters");
        }

        let mut chars = value.chars();

        let rank = Rank::try_from(chars.next().unwrap())?;
        let suit = Suit::try_from(chars.next().unwrap())?;

        Ok(Card { rank, suit })
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        Self { cards: vec![] }
    }
    
    fn stack(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }

    fn draw(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.drain(0..num_cards).collect()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    First,
    Second,
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

struct GameData {
    dealer: Player,
    current_player: Player,
}

impl GameData {
    fn new() -> Self {
        Self {
            dealer: Player::First,
            current_player: Player::Second,
        }
    }
}

pub struct Simulator {
    game_data: GameData,
    deck: Deck,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            game_data: GameData::new(),
            deck: Deck::new(),
        }
    }

    pub fn choose_dealer(&mut self) -> ReadyToDeal {
        let mut cards;
        loop {
            cards = self.deck.draw(2);
            if cards[0].rank != cards[1].rank {
                break;
            }
        }

        let dealer = match cards[0].rank < cards[1].rank {
            true => Player::First,
            false => Player::Second,
        };

        let current_player = dealer.next();

        ReadyToDeal {
            game_data: GameData {
                dealer,
                current_player,
            },
        }
    }
}

pub struct ReadyToDeal {
    game_data: GameData,
}

impl ReadyToDeal {
    pub fn dealer(&self) -> &Player {
        &self.game_data.dealer
    }

    pub fn current_player(&self) -> &Player {
        &self.game_data.current_player
    }
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn chooses_a_dealer_from_cut() {
        let parameters = [
            (
                vec![Card::try_from("as").unwrap(), Card::try_from("2s").unwrap()],
                Player::First,
                Player::Second,
            ),
            (
                vec![Card::try_from("2s").unwrap(), Card::try_from("as").unwrap()],
                Player::Second,
                Player::First,
            ),
        ];
        for (cards, expected_dealer, expected_current) in parameters {
            let mut simulator = Simulator::new();
            simulator.deck.stack(cards);
            let next = simulator.choose_dealer();
            assert!(*next.dealer() == expected_dealer);
            assert!(*next.current_player() == expected_current);
        }
    }
}
