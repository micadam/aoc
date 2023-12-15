use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn score_column(chars: &Vec<Vec<char>>, col: usize) -> usize {
    (0..chars.len())
        .into_iter()
        .filter(|&r| chars[r][col] == 'O')
        .map(|r| chars.len() - r)
        .sum()
}


fn transpose(chars: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_chars = chars.clone();
    for i in 0..new_chars.len() {
        for j in i + 1..new_chars.len() {
            let tmp = new_chars[i][j];
            new_chars[i][j] = new_chars[j][i];
            new_chars[j][i] = tmp;
        }
    }
    new_chars
}

fn turn_upside_down(chars: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    chars.iter().rev().map(|row| row.clone()).collect()
}

fn tilt(row_dir: isize, col_dir: isize, chars: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // let's rotate the board so we're tilting north, and then rotate back
    let mut chars = chars.clone();
    let mut transposed = false;
    let mut upside_down = false;
    let mut col_dir = col_dir;
    if col_dir == 0 {
        chars = transpose(&chars);
        transposed = true;
        col_dir = row_dir;
    }
    if col_dir == 1 {
        chars = turn_upside_down(&chars);
        upside_down = true;
    }

    let mut new_chars = vec![vec!['.'; chars[0].len()]; chars.len()];
    // copy over hashtags
    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            if chars[row][col] == '#' {
                new_chars[row][col] = '#';
            }
        }
    }

    for col in 0..chars[0].len() {
        let mut row = 0;
        while row < chars.len() {
            let end_idx = (row..chars.len())
                .find(|&r| chars[r][col] == '#')
                .unwrap_or(chars.len());
            let num_rocks = (row..end_idx).filter(|&r| chars[r][col] == 'O').count();
            for i in (row..row + num_rocks).rev() {
                new_chars[i][col] = 'O';
            }
            row = end_idx + 1;
        }
    }

    if upside_down {
        new_chars = turn_upside_down(&new_chars);
    }
    if transposed {
        new_chars = transpose(&new_chars);
    }
    new_chars
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = lines
            .into_iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let chars = tilt(0, -1, &chars);
        (0..lines[0].len())
            .map(|col| score_column(&chars, col))
            .sum::<usize>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut chars = lines
            .into_iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let total_tilts = 4 * 1_000_000_000;
        let mut seen: HashMap<(Vec<Vec<char>>, usize), usize> = HashMap::new();
        let mut archive: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
        let dirs = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
        let mut current_step = 0;
        while current_step < total_tilts {
            let chars_tmp = chars.clone();
            let dir_idx = current_step % dirs.len();
            if let Some(j) = seen.get(&(chars_tmp, dir_idx)) {
                let cycle_len = current_step - j;
                let remaining_cycles = (total_tilts - current_step) % cycle_len;
                chars = archive[&(j + remaining_cycles)].clone();
                break;
            }

            seen.insert((chars.clone(), dir_idx), current_step);
            archive.insert(current_step, chars.clone());
            let (row_dir, col_dir) = dirs[dir_idx];
            chars = tilt(row_dir, col_dir, &chars);
            current_step += 1;
        }
        (0..chars[0].len())
            .map(|col| score_column(&chars, col))
            .sum::<usize>()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
