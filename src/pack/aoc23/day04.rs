use std::collections::HashMap;
use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

fn parse_num_winning(line: &String) -> usize {
    let (_, rest) = line.split_once(": ").unwrap();
    let (winners, actual): (HashSet<i32>, Vec<i32>) = rest
        .split_once(" | ")
        .map(|(w, a)| {
            (
                w.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                a.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .unwrap();
    let num_winning = actual.into_iter().filter(|a| winners.contains(a)).count();
    num_winning
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .into_iter()
            .map(|l| parse_num_winning(l))
            .map(|n| if n > 0 { 2i32.pow(n as u32 - 1) } else { 0 })
            .sum::<i32>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut extra_copies: HashMap<usize, usize> = HashMap::new();

        lines
            .into_iter()
            .map(|l| parse_num_winning(l))
            .enumerate()
            .for_each(|(i, n)| {
                let current_num_copies = extra_copies.get(&i).unwrap_or(&0).clone() + 1;
                for j in 1..n + 1 {
                    extra_copies.insert(
                        i + j,
                        extra_copies.get(&(i + j)).unwrap_or(&0) + current_num_copies,
                    );
                }
            });
        // Extra copies from tickets + 1 original copy each.
        (extra_copies.values().sum::<usize>() + lines.len()).to_string()
    }
}

get_day_fn!(Part1, Part2);
