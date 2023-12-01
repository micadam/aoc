use std::collections::HashMap;

use crate::day::Day;

/// Denotes a single day pack (e.g. a single year)
#[derive(Debug)]
pub struct DayPack {
    pub name: String,
    pub days: HashMap<String, Day>,
}

impl DayPack {
    pub fn read_lines(&self, day_name: &String, test: bool) -> Vec<String> {
        let path = format!(
            "./input/{}/{}{}.txt",
            &self.name,
            day_name,
            if test { ".test" } else { "" }
        );
        println!("Reading from {}", path);
        std::fs::read_to_string(path)
            .unwrap_or_else(|_| {
                println!("File not found, defaulting to empty");
                String::new()
            })
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}

macro_rules! get_pack_fn {
    ($name: expr, $( $days: ident), *) => {
        pub fn get_pack() -> DayPack {
            DayPack { name: $name.to_string(), days: HashMap::from([$( (stringify!($days).to_string(), $days::get_day()) ),*]) }
        }
    };
}

macro_rules! get_day_fn {
    ( $( $parts: ident), *) => {
        pub fn get_day() -> Day {
            Day::new(
                module_path!().split("::").last().unwrap().to_string(),
                vec![$(Box::new($parts)),*],
            )
        }
    }
}

pub mod all_packs;
mod aoc23;
mod euler;
