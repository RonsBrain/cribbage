use crate::card::{Card, Rank, Suit};
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Player {
    First,
    Second,
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

pub struct Start;

pub struct DealerResult {
    pub cut_cards: HashMap<Player, Card>,
    pub dealer: Player,
}

impl Start {
    fn new() -> Self {
        Self {}
    }

    pub fn choose_dealer(&self) -> EngineState {
        let mut cards;
        loop {
            cards = get_random_cards(2);

            // If the cut cards are a tie, repeat the cut so we don't need to deal with ties.
            if cards[0].rank == cards[1].rank {
                continue;
            }
            break;
        }

        let mut cut_cards = HashMap::new();
        cut_cards.insert(Player::First, cards[0]);
        cut_cards.insert(Player::Second, cards[1]);

        let dealer = if cards[0].rank < cards[1].rank {
            Player::First
        } else {
            Player::Second
        };

        let result = DealerResult { cut_cards, dealer };
        EngineState::DealerChosen(result)
    }
}

pub enum EngineState {
    NewGame(Start),
    DealerChosen(DealerResult),
}

pub fn new_engine() -> EngineState {
    EngineState::NewGame(Start::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chooses_correct_dealer() {
        for _times in 0..100 {
            let engine = new_engine();
            if let EngineState::NewGame(start) = engine {
                if let EngineState::DealerChosen(data) = start.choose_dealer() {
                    if data.cut_cards[&Player::First].rank < data.cut_cards[&Player::Second].rank {
                        assert_eq!(data.dealer, Player::First);
                    } else {
                        assert_eq!(data.dealer, Player::Second);
                    }
                }
            }
        }
    }
}
