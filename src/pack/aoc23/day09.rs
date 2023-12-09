use std::str::FromStr;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

fn parse(line: &String) -> Vec<isize> {
    line.split(" ")
        .map(|s| isize::from_str(s).unwrap())
        .collect_vec()
}

fn extrapolate(sequence: Vec<isize>, extrapolate_fn: fn(isize, Vec<isize>) -> isize) -> isize {
    let mut diff_sequences = vec![sequence];
    while diff_sequences.last().unwrap().iter().any(|i| *i != 0) {
        let new_sequence = diff_sequences
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        diff_sequences.push(new_sequence);
    }

    diff_sequences
        .into_iter()
        .rev()
        .skip(1)
        .fold(0, extrapolate_fn)
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let sequences = lines.iter().map(parse).collect_vec();
        sequences
            .into_iter()
            .map(|s| extrapolate(s, |a, s| a + s.last().unwrap()))
            .sum::<isize>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let sequences = lines.iter().map(parse).collect_vec();
        sequences
            .into_iter()
            .map(|s| extrapolate(s, |a, s| s.first().unwrap() - a))
            .sum::<isize>()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
