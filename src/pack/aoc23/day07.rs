use std::cmp::Ordering;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    Pair,
    HighCard,
}
fn get_groups(chars: &Vec<char>) -> Vec<usize> {
    chars
        .into_iter()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_, g)| g.count())
        .sorted()
        .rev()
        .collect_vec()
}
impl HandType {
    fn from_groups(groups: &Vec<usize>) -> HandType {
        match &groups[..] {
            [5] => HandType::Five,
            [4, 1] => HandType::Four,
            [3, 2] => HandType::Full,
            [3, ..] => HandType::Three,
            [2, 2, 1] => HandType::TwoPair,
            [2, ..] => HandType::Pair,
            _ => HandType::HighCard,
        }
    }

    fn new(cards: &Vec<char>) -> HandType {
        let groups = get_groups(cards);

        HandType::from_groups(&groups)
    }

    fn new_joker(cards: &Vec<char>) -> HandType {
        let num_jokers = cards.iter().filter(|c| **c == 'J').count();
        let cards_no_jokers = cards
            .into_iter()
            .filter(|c| **c != 'J')
            .map(|c| *c)
            .collect_vec();
        let mut groups = get_groups(&cards_no_jokers);

        if groups.is_empty() {
            groups.push(0);
        }
        groups[0] += num_jokers;
        HandType::from_groups(&groups)
    }

    fn value(&self) -> i32 {
        match self {
            HandType::Five => 6,
            HandType::Four => 5,
            HandType::Full => 4,
            HandType::Three => 3,
            HandType::TwoPair => 2,
            HandType::Pair => 1,
            HandType::HighCard => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bet: usize,
    joker: bool,
}
impl Hand {
    fn new(line: &String, joker: bool) -> Hand {
        let parts = line.split_once(" ").unwrap();
        let cards = parts.0.chars().collect_vec();
        let bet = parts.1.parse::<usize>().unwrap();
        let hand_type = if !joker {
            HandType::new(&cards)
        } else {
            HandType::new_joker(&cards)
        };
        Hand {
            cards,
            hand_type,
            bet,
            joker,
        }
    }
}

fn char_value(c: &char, joker: bool) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if !joker {
                11
            } else {
                1
            }
        }
        'T' => 10,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.value().cmp(&other.hand_type.value()) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| char_value(a, self.joker).cmp(&char_value(b, self.joker)))
                .find(|o| *o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            o => o,
        }
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .into_iter()
            .map(|l| Hand::new(l, false))
            .sorted()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bet)
            .sum::<usize>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .into_iter()
            .map(|l| Hand::new(l, true))
            .sorted()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bet)
            .sum::<usize>()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
