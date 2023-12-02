use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn get_sum_from_lines(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|l| [l.chars().next().unwrap(), l.chars().last().unwrap()])
        .map(|l| l.iter().collect::<String>())
        .map(|l| l.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn contract_digits(line: &String) -> String {
    let mut line = line.clone();
    let spelled_out = HashMap::from([
        ("zero", "zero0zero"),
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);
    spelled_out
        .iter()
        .for_each(|(k, v)| line = line.replace(k, v));
    line
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        get_sum_from_lines(lines).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let lines = lines
            .into_iter()
            .map(|l| contract_digits(&l))
            .collect::<Vec<String>>();
        get_sum_from_lines(&lines).to_string()
    }
}

get_day_fn!(Part1, Part2);
