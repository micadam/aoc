use std::collections::HashSet;
use std::collections::VecDeque;

use crate::day::Day;
use crate::day::Solveable;

fn get_chars(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

trait CheckAndPush {
    fn check_and_push(
        &mut self,
        row: isize,
        col: isize,
        row_dir: isize,
        col_dir: isize,
        max_row: usize,
        max_col: usize,
    );
}
impl CheckAndPush for VecDeque<(usize, usize, isize, isize)> {
    fn check_and_push(
        &mut self,
        row: isize,
        col: isize,
        row_dir: isize,
        col_dir: isize,
        max_row: usize,
        max_col: usize,
    ) {
        if row < 0 || col < 0 || row as usize >= max_row || col as usize >= max_col {
            return;
        }
        self.push_back((row as usize, col as usize, row_dir, col_dir));
    }
}

fn get_num_lit(
    chars: &Vec<Vec<char>>,
    row_start: usize,
    col_start: usize,
    row_dir: isize,
    col_dir: isize,
) -> usize {
    let mut lit: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, isize, isize)> = VecDeque::new();
    queue.push_back((row_start, col_start, row_dir, col_dir));
    while let Some((row, col, row_dir, col_dir)) = queue.pop_front() {
        if visited.contains(&(row, col, row_dir, col_dir)) {
            continue;
        }
        lit.insert((row, col));
        visited.insert((row, col, row_dir, col_dir));
        match (row_dir, col_dir, chars[row][col]) {
            (_, _, '.') | (0, _, '-') | (_, 0, '|') => {
                queue.check_and_push(
                    row as isize + row_dir,
                    col as isize + col_dir,
                    row_dir,
                    col_dir,
                    chars.len(),
                    chars[0].len(),
                );
            }
            (_, _, '-') => {
                queue.check_and_push(
                    row as isize,
                    col as isize - 1,
                    0,
                    -1,
                    chars.len(),
                    chars[0].len(),
                );
                queue.check_and_push(
                    row as isize,
                    col as isize + 1,
                    0,
                    1,
                    chars.len(),
                    chars[0].len(),
                );
            }
            (_, _, '|') => {
                queue.check_and_push(
                    row as isize - 1,
                    col as isize,
                    -1,
                    0,
                    chars.len(),
                    chars[0].len(),
                );
                queue.check_and_push(
                    row as isize + 1,
                    col as isize,
                    1,
                    0,
                    chars.len(),
                    chars[0].len(),
                );
            }
            (_, _, '\\') => {
                let new_row_dir = col_dir;
                let new_col_dir = row_dir;
                queue.check_and_push(
                    row as isize + new_row_dir,
                    col as isize + new_col_dir,
                    new_row_dir,
                    new_col_dir,
                    chars.len(),
                    chars[0].len(),
                );
            }
            (_, _, '/') => {
                let new_row_dir = -col_dir;
                let new_col_dir = -row_dir;
                queue.check_and_push(
                    row as isize + new_row_dir,
                    col as isize + new_col_dir,
                    new_row_dir,
                    new_col_dir,
                    chars.len(),
                    chars[0].len(),
                );
            }
            (_, _, _) => {
                panic!("Unknown char {} at {},{}", chars[row][col], row, col);
            }
        }
    }
    lit.len()
}
struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = get_chars(lines);

        get_num_lit(&chars, 0, 0, 0, 1).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = get_chars(lines);

        (0..chars.len())
            .flat_map(|row| [(row, 0, 0, 1), (row, chars[0].len() - 1, 0, -1)])
            .chain(
                (0..chars[0].len()).flat_map(|col| [(0, col, 1, 0), (chars.len() - 1, col, -1, 0)]),
            )
            .map(|(row, col, row_dir, col_dir)| get_num_lit(&chars, row, col, row_dir, col_dir))
            .max()
            .unwrap()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
