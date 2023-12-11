use std::char;
use std::cmp::max;
use std::cmp::min;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

fn get_chars(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_empty_rows_cols(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = universe
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if !row.iter().any(|&c| c == '#') {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec();
    let empty_cols = (0..universe[0].len())
        .filter_map(|col| {
            if !universe.iter().any(|row| row[col] == '#') {
                Some(col)
            } else {
                None
            }
        })
        .collect_vec();
    (empty_rows, empty_cols)
}

fn manhattan_distance(x: (usize, usize), y: (usize, usize)) -> usize {
    (x.0 as isize - y.0 as isize).abs() as usize + (x.1 as isize - y.1 as isize).abs() as usize
}

fn get_extra_distance(a: usize, b: usize, expanded: &Vec<usize>, expansion_times: usize) -> usize {
    let l = min(a, b);
    let r = max(a, b);
    // `a` will never be in expanded because it's a galaxy coord
    // so we will always get err
    // Also we assume sorted rows/cols
    let mut expanded_idx = expanded.binary_search(&l).unwrap_err();
    let mut extra_distance = 0;
    while expanded_idx < expanded.len() && expanded[expanded_idx] < r {
        // -1 for the original row/column which will be included in the manhattan distance
        extra_distance += expansion_times - 1;
        expanded_idx += 1;
    }
    extra_distance
}

fn get_galaxy_coords(chars: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    chars
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &c)| if c == '#' { Some((i, j)) } else { None })
                .collect_vec()
        })
        .collect_vec()
}

fn get_expanded_pairwise_distance(lines: &Vec<String>, times: usize) -> usize {
    let chars = get_chars(lines);
    let (rows_to_expand, cols_to_expand) = get_empty_rows_cols(&chars);
    let galaxy_coords = get_galaxy_coords(&chars);
    galaxy_coords
        .into_iter()
        .permutations(2)
        .filter(|p| p[0] < p[1])
        .map(|p| {
            manhattan_distance(p[0], p[1])
                + get_extra_distance(p[0].0, p[1].0, &rows_to_expand, times)
                + get_extra_distance(p[0].1, p[1].1, &cols_to_expand, times)
        })
        .sum::<usize>()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        get_expanded_pairwise_distance(lines, 2).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        get_expanded_pairwise_distance(lines, 1_000_000).to_string()
    }
}

get_day_fn!(Part1, Part2);
