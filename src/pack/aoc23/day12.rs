use itertools::Itertools;
use memoize::memoize;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Hash)]
struct Problem {
    schematic: Vec<char>,
    groups: Vec<usize>,
}

fn parse_line(line: &String, repetitions: usize) -> Problem {
    let (schematic_str, groups_str) = line.split_whitespace().collect_tuple().unwrap();
    let schematic = schematic_str.chars().collect_vec();
    let groups = groups_str
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let repeated_schematic = (0..repetitions)
        .into_iter()
        .flat_map(|i| {
            schematic
                .clone()
                .into_iter()
                .chain(if i < repetitions - 1 { Some('?') } else { None })
        })
        .collect_vec();
    let repeated_groups = groups.repeat(repetitions);
    Problem {
        schematic: repeated_schematic,
        groups: repeated_groups,
    }
}

#[memoize]
fn get_possible_combinations(schematic: Vec<char>, groups: Vec<usize>) -> usize {
    if groups.len() == 0 {
        if schematic.len() == 0 {
            1
        } else {
            if schematic.iter().all(|c| *c == '.' || *c == '?') {
                1
            } else {
                0
            }
        }
    } else if schematic.len() == 0 {
        0 // We already know groups are not empty
    } else if schematic[0] == '.' {
        // Case 1: schematic starts with dots -- skip the dots
        let new_schematic = schematic
            .iter()
            .skip_while(|c| **c == '.')
            .map(|c| *c)
            .collect_vec();
        get_possible_combinations(new_schematic, groups)
    } else if schematic[0] == '#' {
        // Case 2: schematic starts with a hash -- try to match it with a group
        let group_size = groups[0];
        if schematic.len() >= group_size
            && schematic[..group_size]
                .iter()
                .all(|c| *c == '#' || *c == '?')
        {
            let new_schematic = schematic[group_size..].to_vec();
            let new_groups = groups[1..].to_vec();
            if new_schematic.len() > 0 {
                if new_schematic[0] == '.' || new_schematic[0] == '?' {
                    get_possible_combinations(new_schematic[1..].to_vec(), new_groups)
                } else {
                    0
                }
            } else {
                get_possible_combinations(new_schematic, new_groups)
            }
        } else {
            0
        }
    } else if schematic[0] == '?' {
        // Case 3: schematic starts with a question mark -- either match it or don't
        let mut acc = 0;
        acc += get_possible_combinations(schematic[1..].to_vec(), groups.clone());
        let group_size = groups[0];
        if schematic.len() >= group_size
            && schematic[..group_size]
                .iter()
                .all(|c| *c == '#' || *c == '?')
        {
            let new_schematic = schematic[group_size..].to_vec();
            let new_groups = groups[1..].to_vec();
            if new_schematic.len() > 0 {
                if new_schematic[0] == '.' || new_schematic[0] == '?' {
                    acc += get_possible_combinations(new_schematic[1..].to_vec(), new_groups);
                }
            } else {
                acc += get_possible_combinations(new_schematic, new_groups);
            }
        }
        acc
    } else {
        panic!("Invalid schematic: {}", schematic.iter().join(", "));
    }
}

fn solve(lines: &Vec<String>, repetitions: usize) -> String {
    lines
        .into_iter()
        .map(|l| parse_line(l, repetitions))
        .map(|p| get_possible_combinations(p.schematic, p.groups))
        .sum::<usize>()
        .to_string()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        solve(lines, 1)
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        solve(lines, 5)
    }
}

get_day_fn!(Part1, Part2);
