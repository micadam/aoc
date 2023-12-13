use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let mut transposed = vec![String::new(); lines[0].len()];

    for line in lines.iter() {
        for (j, c) in line.chars().enumerate() {
            transposed[j].push(c);
        }
    }

    transposed
}

fn score_candidate(lines: &Vec<String>, idx: usize, mult: usize, req_mistakes: usize) -> usize {
    let mut i = idx as isize;
    let mut di = 1;
    let mut rem_mistakes = req_mistakes;
    while i >= 0 && i as usize + di < lines.len() {
        let num_mistakes = lines[i as usize]
            .chars()
            .zip(lines[i as usize + di].chars())
            .filter(|(c1, c2)| c1 != c2)
            .count();
        if num_mistakes > rem_mistakes {
            return 0;
        }
        i -= 1;
        di += 2;
        rem_mistakes -= num_mistakes;
    }
    if rem_mistakes > 0 {
        return 0;
    }
    return mult * (idx + 1);
}

fn score_rows(lines: &Vec<String>, mult: usize, req_mistakes: usize) -> usize {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter_map(|((i, l1), (_, l2))| {
            let mistakes_found = l1
                .chars()
                .zip(l2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            Some(i).filter(|_| mistakes_found <= req_mistakes)
        })
        .map(|idx| score_candidate(lines, idx, mult, req_mistakes))
        .sum()
}

fn solve_pattern(lines: &Vec<String>, num_mistakes: usize) -> usize {
    score_rows(lines, 100, num_mistakes) + score_rows(&transpose(lines), 1, num_mistakes)
}

fn solve(lines: &Vec<String>, num_mistakes: usize) -> String {
    lines
        .split(|l| l.is_empty())
        .map(|s| s.into_iter().map(|s| s.clone()).collect_vec())
        .map(|lines| solve_pattern(&lines, num_mistakes))
        .sum::<usize>()
        .to_string()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        solve(&lines, 0)
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        solve(&lines, 1)
    }
}

get_day_fn!(Part1, Part2);
