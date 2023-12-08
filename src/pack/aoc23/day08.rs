use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Debug)]
struct Map {
    path: Vec<char>,
    turns: HashMap<String, (String, String)>,
}

struct Cycle {
    start: usize,
    len: usize,
    end_idx: usize,
}

fn parse(lines: &Vec<String>) -> Map {
    let path = lines[0].chars().collect_vec();
    let turns = lines[2..]
        .into_iter()
        .map(|l| {
            let (from, to) = l.split_once(" = ").unwrap();
            let (l, r) = to
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();
            (from.to_string(), (l.to_string(), r.to_string()))
        })
        .collect::<HashMap<String, (String, String)>>();
    Map { path, turns }
}

fn step(map: &Map, loc: &String, steps_taken: usize) -> String {
    let (l, r) = map.turns.get(loc).unwrap();
    match map.path[steps_taken % map.path.len()] {
        'L' => l.clone(),
        'R' => r.clone(),
        _ => panic!(
            "Invalid path char {}",
            map.path[steps_taken % map.path.len()]
        ),
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let map = parse(&lines);
        let mut loc = "AAA".to_string();
        let mut steps_taken = 0;
        while loc != "ZZZ" {
            loc = step(&map, &loc, steps_taken);
            steps_taken += 1
        }
        steps_taken.to_string()
    }
}

fn find_cycle(map: &Map, start_loc: &String) -> Cycle {
    let mut loc = start_loc.clone();
    let mut visited = HashMap::new();
    // key = loc, path idx, value = steps taken
    visited.insert((loc.clone(), 0), 0);
    let mut steps_taken = 0;
    let mut end = 0;
    loop {
        let map_idx = steps_taken % map.path.len();
        loc = step(&map, &loc, steps_taken);
        steps_taken += 1;
        if loc.ends_with("Z") {
            if end > 0 {
                panic!("Assumed there would only be one end in a cycle but there's two! Find another algorithm bozo.");
            }
            end = steps_taken;
        }
        if !visited.contains_key(&(loc.clone(), map_idx)) {
            visited.insert((loc.clone(), map_idx), steps_taken);
        } else {
            let cycle_start = *visited.get(&(loc.clone(), map_idx)).unwrap();
            return Cycle {
                start: cycle_start,
                len: steps_taken - cycle_start,
                end_idx: end,
            };
        }
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let map = parse(&lines);
        let cycle_info = map
            .turns
            .iter()
            .filter(|(l, _)| l.ends_with("A"))
            .map(|(l, _)| l.clone())
            .map(|l| find_cycle(&map, &l))
            .collect_vec();

        let mut current_steps_taken = cycle_info[0].end_idx;
        let mut current_delta = cycle_info[0].len;
        cycle_info.iter().skip(1).for_each(|cycle| {
            while (current_steps_taken - cycle.start) % cycle.len != cycle.end_idx - cycle.start {
                current_steps_taken += current_delta;
            }
            current_delta = current_delta.lcm(&cycle.len);
        });
        current_steps_taken.to_string()
    }
}

get_day_fn!(Part1, Part2);
