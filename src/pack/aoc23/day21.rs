use core::panic;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn get_chars(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

trait Grid {
    fn find_start(&self) -> (usize, usize);
    fn iter_neighbours(&self, r: isize, c: isize, loops: bool) -> Vec<(isize, isize)>;
    fn get_char_looping(&self, r: isize, c: isize) -> char;
}
impl Grid for Vec<Vec<char>> {
    fn find_start(&self) -> (usize, usize) {
        for (r, row) in self.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if *col == 'S' {
                    return (r, c);
                }
            }
        }
        panic!("No start found");
    }

    fn get_char_looping(&self, r: isize, c: isize) -> char {
        let mut r = r;
        let mut c = c;
        while r < 0 {
            r += self.len() as isize;
        }
        while c < 0 {
            c += self[0].len() as isize;
        }
        let r = r as usize;
        let c = c as usize;
        self[r % self.len()][c % self[0].len()]
    }

    fn iter_neighbours(&self, r: isize, c: isize, loops: bool) -> Vec<(isize, isize)> {
        let mut neighbours = Vec::new();
        for (dr, dc) in DIRS {
            let nr = r + dr;
            let nc = c + dc;
            if !loops
                && (nr < 0 || nr >= self.len() as isize || nc < 0 || nc >= self[0].len() as isize)
                || self.get_char_looping(nr, nc) == '#'
            {
                continue;
            }
            neighbours.push((nr, nc));
        }
        neighbours
    }
}

fn get_num_possible(lines: &Vec<String>, max_steps: usize, loops: bool) -> usize {
    let chars = get_chars(lines);
    let (r, c) = chars.find_start();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((r as isize, c as isize, 0));
    while let Some((r, c, d)) = q.pop_front() {
        if visited.contains(&(r, c, d % 2 == 0)) {
            continue;
        }
        visited.insert((r, c, d % 2 == 0));
        if d >= max_steps {
            continue;
        }
        for (nr, nc) in chars.iter_neighbours(r, c, loops) {
            if !visited.contains(&(nr, nc, (d + 1) % 2 == 0)) {
                q.push_back((nr, nc, d + 1));
            }
        }
    }
    visited
        .into_iter()
        .filter(|(_, _, d)| *d == (max_steps % 2 == 0))
        .count()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        get_num_possible(lines, 64, false).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let num_steps = 26_501_365;
        let chars = get_chars(lines);
        let mid_row = chars.len() / 2;
        let mid_col = chars[0].len() / 2;
        assert!(
            chars[mid_row][mid_col] == 'S',
            "Assumed start would be in the middle but isn't"
        );
        assert!(
            chars.len() == chars[0].len(),
            "Assumed square map but isn't"
        );
        let steps = (mid_col..10_000)
            .step_by(chars.len())
            .take(5)
            .map(|n| get_num_possible(lines, n, true))
            .collect_vec();
        let first_derivative = steps
            .iter()
            .clone()
            .zip(steps.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
        let second_derivative = first_derivative
            .clone()
            .iter()
            .zip(first_derivative.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
        assert!(
            second_derivative.iter().all_equal(),
            "Assumed exact quadratic growth but isn't"
        );
        assert!(
            (num_steps - mid_row) % chars.len() == 0,
            "Assumed number of steps would divide exactly into square length, but doesn't"
        );
        // The series of steps is a quadratic on the form of
        // n^2 (d/2) + n (a - d/2) + c
        // where:
        // * c is the solution for MID_ROW steps,
        // * a is the difference between the solutions for MID_ROW and MID_ROW + NUM_STEPS
        // * d is the "second derivative" in terms of diffs, which should be constant
        // We're looking for the solution for n = (NUM_STEPS - MID_ROW) / NUM_ROWS
        let c = steps[0];
        let a = first_derivative[0];
        let d = first_derivative[0];
        let n = (num_steps - mid_row) / chars.len();
        (n * n * d / 2 + n * (a - d / 2) + c).to_string()
    }
}

get_day_fn!(Part1, Part2);
