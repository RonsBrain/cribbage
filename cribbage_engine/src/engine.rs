use crate::card::{Card, Rank, Suit};
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Player {
    First,
    Second,
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

fn get_random_cards(num_cards: usize) -> Vec<Card> {
    use Rank::*;
    use Suit::*;
    let mut cards = Vec::new();
    for rank in [
        Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
    ] {
        for suit in [Spades, Hearts, Diamonds, Clubs] {
            cards.push(Card::new(rank, suit));
        }
    }

    cards.shuffle(&mut rng());
    cards.drain(0..num_cards).collect()
}

pub struct Start {
    cards: [Card; 2],
}

impl Start {
    fn new() -> Self {
        let mut cards;
        loop {
            cards = get_random_cards(2);

            // If the cut cards are a tie, repeat the cut so we don't need to deal with ties.
            if cards[0].rank == cards[1].rank {
                continue;
            }
            break;
        }
        Self {
            cards: [cards[0], cards[1]],
        }
    }

    pub fn choose_dealer(&self) -> EngineState {
        let cut_cards = HashMap::from([
            (Player::First, self.cards[0]),
            (Player::Second, self.cards[1]),
        ]);

        let dealer = if self.cards[0].rank < self.cards[1].rank {
            Player::First
        } else {
            Player::Second
        };

        let game_data = GameData::new(dealer);
        let result = CutResult {
            cut_cards,
            game_data,
        };
        EngineState::DealerChosen(result)
    }
}

pub struct CutResult {
    pub cut_cards: HashMap<Player, Card>,
    pub game_data: GameData,
}

pub struct GameData {
    pub dealer: Player,
    pub current_player: Player,
    pub scores: HashMap<Player, u8>,
}

impl GameData {
    fn new(dealer: Player) -> Self {
        Self {
            dealer,
            current_player: dealer.next(),
            scores: HashMap::from([(Player::First, 0), (Player::Second, 0)]),
        }
    }
}

pub struct DealResult {
    pub dealer: Player,
    pub cards: HashMap<Player, [Card; 6]>,
}

pub enum EngineState {
    NewGame(Start),
    DealerChosen(CutResult),
}

pub fn new_engine() -> EngineState {
    EngineState::NewGame(Start::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chooses_correct_dealer() {
        for (cards, dealer, current_player) in [
            (
                [Card::try_from("as").unwrap(), Card::try_from("ks").unwrap()],
                Player::First,
                Player::Second,
            ),
            (
                [Card::try_from("ks").unwrap(), Card::try_from("as").unwrap()],
                Player::Second,
                Player::First,
            ),
        ] {
            let engine = new_engine();
            if let EngineState::NewGame(mut start) = engine {
                start.cards = cards;
                if let EngineState::DealerChosen(result) = start.choose_dealer() {
                    assert_eq!(result.cut_cards[&Player::First], cards[0]);
                    assert_eq!(result.cut_cards[&Player::Second], cards[1]);
                    assert_eq!(result.game_data.dealer, dealer);
                    assert_eq!(result.game_data.current_player, current_player);
                    assert_eq!(result.game_data.scores[&Player::First], 0);
                    assert_eq!(result.game_data.scores[&Player::Second], 0);
                } else {
                    panic!("Game moved to incorrect state!");
                }
            }
        }
    }
}
