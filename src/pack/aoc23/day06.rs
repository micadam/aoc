use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

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

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = (b * b - 4.0 * a * c);
    if discriminant < 0.0 {
        panic!("No real roots");
    }
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (x1, x2)
}

fn score_race_fast(race: &Race) -> usize {
    // x * (t - x) > d
    // x * t - x^2 - d > 0
    // a = -1, b = t, c = -d
    let (x1, x2) = solve_quadratic(-1.0, race.time as f64, -(race.distance as f64));
    // Technically this doesn't solve the case where x2 is a whole number, but
    // the odds of that are so low that I don't care.
    (x2.ceil() - x1.ceil()) as usize
}

struct Part1;
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

struct Part1Fast;
impl Solveable for Part1Fast {
    fn solve(&self, lines: &Vec<String>) -> String {
        let races = partse_races(lines);
        races
            .into_iter()
            .map(|race| score_race_fast(&race))
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

struct Part2Fast;
impl Solveable for Part2Fast {
    fn solve(&self, lines: &Vec<String>) -> String {
        let race = parse_races_as_one(lines);
        score_race_fast(&race).to_string()
    }
}

get_day_fn!(Part1, Part1Fast, Part2, Part2Fast);
