use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

fn next_position(
    pos: &(usize, usize),
    dpos: &(i32, i32),
    maxes: &(usize, usize),
) -> Option<(usize, usize)> {
    let nrow = pos.0 as i32 + dpos.0;
    let ncol = pos.1 as i32 + dpos.1;
    if nrow < 0 || nrow >= maxes.0 as i32 || ncol < 0 || ncol >= maxes.1 as i32 {
        None
    } else {
        Some((nrow as usize, ncol as usize))
    }
}

fn get_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
}

fn find_start(chars: &Vec<Vec<char>>) -> (usize, usize) {
    chars
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|pos| (i, pos)))
        .unwrap()
}

fn get_exits(from: &char) -> Vec<(i32, i32)> {
    match from {
        '-' => vec![(0, 1), (0, -1)],
        '|' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, -1), (1, 0)],
        _ => vec![(0, 1), (1, 0), (-1, 0), (0, -1)],
    }
}

fn is_entrance_allowed(from: (i32, i32), to: &char) -> bool {
    match to {
        'S' => true,
        '-' => from.0 == 0,
        '|' => from.1 == 0,
        'L' => from == (0, -1) || from == (1, 0),
        'F' => from == (0, -1) || from == (-1, 0),
        'J' => from == (0, 1) || from == (1, 0),
        '7' => from == (0, 1) || from == (-1, 0),
        _ => false,
    }
}

fn find_furthest(lines: &Vec<String>) -> usize {
    let chars = get_grid(lines);
    let s_pos = find_start(&chars);

    let mut dist_map = HashMap::new();
    let mut deque = VecDeque::new();
    deque.push_back((0, s_pos));
    while let Some((dist, pos)) = deque.pop_front() {
        if dist_map.contains_key(&pos) {
            continue;
        }
        dist_map.insert(pos, dist);
        for dpos in get_exits(&chars[pos.0][pos.1]) {
            if let Some(npos) = next_position(&pos, &dpos, &(chars.len(), chars[0].len())) {
                let nchar = chars[npos.0][npos.1];
                if !is_entrance_allowed(dpos, &nchar) {
                    continue;
                }
                let ndist = (dist + 1) as i32;
                deque.push_back((ndist, npos));
            }
        }
    }
    dist_map.values().max().unwrap().clone() as usize
}

// Assuming I'm moving in a `dpos` direction, find my left hand side, and right hand side directions.
fn get_left_right(dpos: &(i32, i32)) -> [(i32, i32); 2] {
    match dpos {
        (0, 1) => [(-1, 0), (1, 0)],
        (0, -1) => [(1, 0), (-1, 0)],
        (1, 0) => [(0, 1), (0, -1)],
        (-1, 0) => [(0, -1), (0, 1)],
        _ => panic!("Invalid dpos: {:?}", dpos),
    }
}

#[derive(Debug)]
struct Blob {
    tiles: HashSet<(usize, usize)>,
    inside: bool,
}
impl Blob {
    fn new(
        origin: &(usize, usize),
        boundary: &HashSet<(usize, usize)>,
        inside: &bool,
        max_row: &usize,
        max_col: &usize,
    ) -> Blob {
        let mut tiles = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back(*origin);
        while let Some(pos) = deque.pop_front() {
            assert!(!boundary.contains(&pos));
            if tiles.contains(&pos) {
                continue;
            }
            tiles.insert(pos);
            for dpos in get_exits(&'*') {
                if let Some(npos) = next_position(&pos, &dpos, &(*max_row, *max_col)) {
                    if boundary.contains(&npos) || tiles.contains(&npos) {
                        continue;
                    }
                    deque.push_back(npos);
                }
            }
        }
        Blob {
            tiles,
            inside: *inside,
        }
    }
}

fn find_enclosed(lines: &Vec<String>) -> usize {
    let chars = get_grid(lines);
    let s_pos = find_start(&chars);

    let mut path = vec![s_pos];
    let mut is_path = HashSet::new();
    is_path.insert(s_pos);
    let mut curr = Some(s_pos);
    // Each pipeline piece is guaranteed to be connected to exactly two other pieces,
    // so we can just follow the path until we get back to the start.
    while curr.is_some() {
        let last: &(usize, usize) = path.last().unwrap();
        let next = get_exits(&chars[last.0][last.1]).iter().find_map(|&dpos| {
            let nrow = last.0 as i32 + dpos.0;
            let ncol = last.1 as i32 + dpos.1;
            let npos = (nrow as usize, ncol as usize);
            let nchar = chars[npos.0][npos.1];
            if is_entrance_allowed(dpos, &nchar) && !is_path.contains(&npos) {
                Some(npos)
            } else {
                None
            }
        });
        if let Some(n) = next {
            path.push(n);
            is_path.insert(n);
        }
        curr = next;
    }
    let mut blobs = vec![];
    let mut tile_to_blob = HashMap::new();
    let is_loop_clockwise = path
        .iter()
        .enumerate()
        .map(|(i, (row, col))| {
            let next = path[(i + 1) % path.len()];
            (next.0 as i32 - *row as i32) * (next.1 as i32 + *col as i32)
        })
        .sum::<i32>()
        > 0;
    let inside_lr = [!is_loop_clockwise, is_loop_clockwise];
    path.iter().enumerate().for_each(|(i, pos)| {
        let next = path[(i + 1) % path.len()];
        let dpos = (next.0 as i32 - pos.0 as i32, next.1 as i32 - pos.1 as i32);
        for (tile, (dpos2, inside)) in [*pos, next]
            .iter()
            .cartesian_product(get_left_right(&dpos).iter().zip(inside_lr.iter()))
        {
            if let Some(npos) = next_position(&tile, &dpos2, &(chars.len(), chars[0].len())) {
                if tile_to_blob.contains_key(&npos) || is_path.contains(&npos) {
                    continue;
                }
                let blob = Blob::new(&npos, &is_path, &inside, &lines.len(), &lines[0].len());
                blob.tiles.iter().for_each(|t| {
                    tile_to_blob.insert(*t, blobs.len());
                });
                blobs.push(blob);
            }
        }
    });
    assert!(
        tile_to_blob.len() + is_path.len() == chars.len() * chars[0].len(),
        "{} + {} != {}",
        tile_to_blob.len(),
        is_path.len(),
        chars.len() * chars[0].len()
    );
    blobs
        .into_iter()
        .filter(|b| b.inside)
        .map(|b| b.tiles.len())
        .sum()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        find_furthest(lines).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        find_enclosed(lines).to_string()
    }
}

get_day_fn!(Part1, Part2);
