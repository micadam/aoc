use std::cmp::max;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

struct Subset {
    red: i32,
    blue: i32,
    green: i32,
}
impl Subset {
    fn is_possible(&self, max: &Subset) -> bool {
        self.red <= max.red && self.blue <= max.blue && self.green <= max.green
    }

    fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

struct Game {
    id: i32,
    subsets: Vec<Subset>,
}

fn parse_game(line: &String) -> Game {
    let id_and_rest = line.split(": ").collect::<Vec<&str>>();
    let id = id_and_rest[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let subset_parts = id_and_rest[1].split("; ");
    let subsets = subset_parts
        .map(|s| {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            s.replace(",", "")
                .split(" ")
                .chunks(2)
                .into_iter()
                .map(|p| p.collect::<Vec<&str>>())
                .for_each(|p| match p[1] {
                    "red" => red = p[0].parse::<i32>().unwrap(),
                    "blue" => blue = p[0].parse::<i32>().unwrap(),
                    "green" => green = p[0].parse::<i32>().unwrap(),
                    a => panic!("Invalid color {}", a),
                });
            Subset { red, blue, green }
        })
        .collect::<Vec<Subset>>();
    Game { id, subsets }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let games = lines
            .into_iter()
            .map(|l| parse_game(l))
            .collect::<Vec<Game>>();
        let max_subset = Subset {
            red: 12,
            green: 13,
            blue: 14,
        };
        games
            .into_iter()
            .filter(|g| g.subsets.iter().all(|s| s.is_possible(&max_subset)))
            .map(|g| g.id)
            .sum::<i32>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let games = lines
            .into_iter()
            .map(|l| parse_game(l))
            .collect::<Vec<Game>>();
        games
            .iter()
            .map(|g| {
                g.subsets.iter().fold(
                    Subset {
                        red: 0,
                        blue: 0,
                        green: 0,
                    },
                    |acc, s| Subset {
                        red: max(acc.red, s.red),
                        blue: max(acc.blue, s.blue),
                        green: max(acc.green, s.green),
                    },
                )
            })
            .map(|s| s.power())
            .sum::<i32>()
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
