use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

struct Part1;
fn partse_races(lines: &Vec<String>) -> Vec<Race> {
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let distnces = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    times
        .iter()
        .zip(distnces.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect_vec()
}

fn parse_races_as_one(lines: &Vec<String>) -> Race {
    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .collect_vec()
        .concat()
        .parse()
        .unwrap();
    let distance = lines[1]
        .split_whitespace()
        .skip(1)
        .collect_vec()
        .concat()
        .parse()
        .unwrap();
    Race { time, distance }
}

fn score_race(race: &Race) -> usize {
    (0..race.time + 1)
        .filter(|t| (race.time - t) * t > race.distance)
        .count()
}

impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let races = partse_races(lines);
        races
            .into_iter()
            .map(|race| score_race(&race))
            .product::<usize>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let race = parse_races_as_one(lines);
        score_race(&race).to_string()
    }
}

get_day_fn!(Part1, Part2);
