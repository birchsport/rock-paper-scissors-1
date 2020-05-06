extern crate rand;

use rand::prelude::*;
use strum::IntoEnumIterator;

use self::Hand::*;
use self::HandResult::*;

#[derive(ToString, Eq, Debug, PartialEq)]
pub enum HandResult {
    Win,
    Lose,
    Draw,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, ToString)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

lazy_static! {
    pub static ref HANDS: Vec<Hand> = Hand::iter().collect();
    pub static ref HANDS_NAMES: Vec<String> = Hand::iter().map(|hand| hand.to_string()).collect();
}

pub trait Beats {
    fn beats(&self) -> Vec<&Self>;
}

impl Beats for Hand {
    fn beats(&self) -> Vec<&Self> {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Rock => vec![&Lizard, &Scissors],
            Paper => vec![&Spock, &Rock],
            Scissors => vec![&Paper, &Lizard],
            Lizard => vec![&Spock, &Paper],
            Spock => vec![&Rock, &Scissors]
        }
    }
}

pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (&own_hand.beats(), &other_hand.beats());

    match (own_beats, other_beats) {
        _ if own_beats.contains(&&other_hand) => Win,
        _ if other_beats.contains(&&own_hand) => Lose,
        _ => Draw,
    }
}

pub fn random_hand(rng: &mut ThreadRng) -> Hand {
    return *HANDS.choose(rng).unwrap();
}

#[cfg(test)]
mod tests {
    use super::{play_hand, HANDS, HANDS_NAMES};
    use super::{Hand::*, HandResult::*};
    use std::collections::HashSet;

    #[test]
    fn test_unique_names() {
        assert_eq!(HANDS_NAMES.len(), HANDS.len());
        assert_eq!(
            HANDS_NAMES.iter().collect::<HashSet<_>>().len(),
            HANDS.len()
        );
    }

    #[test]
    fn test_play_hand() {
        assert_eq!(play_hand(Rock, Scissors), Win);
        assert_eq!(play_hand(Rock, Paper), Lose);
        assert_eq!(play_hand(Rock, Rock), Draw);
        assert_eq!(play_hand(Rock, Lizard), Win);
        assert_eq!(play_hand(Rock, Spock), Lose);

        assert_eq!(play_hand(Paper, Rock), Win);
        assert_eq!(play_hand(Paper, Scissors), Lose);
        assert_eq!(play_hand(Paper, Paper), Draw);
        assert_eq!(play_hand(Paper, Lizard), Lose);
        assert_eq!(play_hand(Paper, Spock), Win);

        assert_eq!(play_hand(Scissors, Paper), Win);
        assert_eq!(play_hand(Scissors, Rock), Lose);
        assert_eq!(play_hand(Scissors, Scissors), Draw);
        assert_eq!(play_hand(Scissors, Lizard), Win);
        assert_eq!(play_hand(Scissors, Spock), Lose);

        assert_eq!(play_hand(Spock, Paper), Lose);
        assert_eq!(play_hand(Spock, Rock), Win);
        assert_eq!(play_hand(Spock, Lizard), Lose);
        assert_eq!(play_hand(Spock, Spock), Draw);
        assert_eq!(play_hand(Spock, Scissors), Win);

        assert_eq!(play_hand(Lizard, Paper), Win);
        assert_eq!(play_hand(Lizard, Spock), Win);
        assert_eq!(play_hand(Lizard, Rock), Lose);
        assert_eq!(play_hand(Lizard, Scissors), Lose);
        assert_eq!(play_hand(Lizard, Lizard), Draw);
    }
}
