use std::cmp::min;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Fragment {
    row: usize,
    col_start: usize,
    col_end: usize,
}
impl Fragment {
    fn as_number(self, lines: &Vec<String>) -> i32 {
        lines[self.row][self.col_start..self.col_end]
            .parse::<i32>()
            .unwrap()
    }
    fn iterate_around(
        self,
        row_max: usize,
        col_max: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let col_start = if self.col_start == 0 {
            0
        } else {
            self.col_start - 1
        };
        let col_end = min(self.col_end + 1, col_max);
        let row_start = if self.row == 0 { 0 } else { self.row - 1 };
        let row_end = min(self.row + 2, row_max);
        let mut col = col_start;
        let mut row = row_start;
        std::iter::from_fn(move || {
            while row == self.row && col >= self.col_start && col < self.col_end {
                col += 1;
            }
            if col >= col_end {
                col = col_start;
                row += 1;
            }
            if row >= row_end {
                return None;
            }
            let result = (row, col);
            col += 1;
            Some(result)
        })
    }
}

fn extract_fragments(line: &String, row: usize, is_valid: impl Fn(char) -> bool) -> Vec<Fragment> {
    let mut numbers = Vec::new();
    let mut col_start = 0;
    let mut inside_valid = false;
    for (i, c) in line.chars().enumerate() {
        if is_valid(c) {
            if !inside_valid {
                col_start = i;
                inside_valid = true;
            }
        } else {
            if inside_valid {
                inside_valid = false;
                numbers.push(Fragment {
                    row,
                    col_start,
                    col_end: i,
                });
            }
        }
    }
    if inside_valid {
        numbers.push(Fragment {
            row,
            col_start,
            col_end: line.len(),
        });
    }
    numbers
}

trait Symbol {
    fn is_symbol(&self) -> bool;
}
impl Symbol for char {
    fn is_symbol(&self) -> bool {
        !self.is_ascii_digit() && !(*self == '.')
    }
}

fn expand_number(row: usize, col: usize, chars: &Vec<Vec<char>>) -> Fragment {
    let mut col_start = col;
    let mut col_end = col;
    while col_start > 0 && chars[row][col_start - 1].is_ascii_digit() {
        col_start -= 1;
    }
    while col_end < chars[0].len() && chars[row][col_end].is_ascii_digit() {
        col_end += 1;
    }
    Fragment {
        row,
        col_start,
        col_end,
    }
}
struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = lines
            .into_iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let numbers = lines
            .into_iter()
            .enumerate()
            .map(|(i, l)| extract_fragments(l, i, |c| c.is_ascii_digit()))
            .flatten()
            .collect::<Vec<Fragment>>();
        numbers
            .iter()
            .filter(|n| {
                n.iterate_around(lines.len(), lines[0].len())
                    .any(|(r, c)| chars[r][c].is_symbol())
            })
            .map(|n| n.as_number(&lines))
            .sum::<i32>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = lines
            .into_iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let asterisks = lines
            .into_iter()
            .enumerate()
            .map(|(i, l)| extract_fragments(l, i, |c| c == '*'))
            .flatten()
            .collect::<Vec<Fragment>>();
        asterisks
            .into_iter()
            .map(|n| {
                n.iterate_around(lines.len(), lines[0].len())
                    .filter(|(r, c)| chars[*r][*c].is_ascii_digit())
                    .map(|(r, c)| expand_number(r, c, &chars))
                    .unique()
                    .map(|n| n.as_number(&lines))
                    .collect::<Vec<i32>>()
            })
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum::<i32>()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
