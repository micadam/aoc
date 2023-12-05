use std::cmp::max;
use std::cmp::min;

use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct AlmanacEntry {
    source_start: usize,
    source_end: usize,
    target_start: usize,
    len: usize,
}
trait Includes {
    fn includes(&self, seed: usize) -> bool;
}
impl Includes for AlmanacEntry {
    fn includes(&self, seed: usize) -> bool {
        seed >= self.source_start && seed < self.source_end
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SeedRange {
    start: usize,
    len: usize,
}
impl Includes for SeedRange {
    fn includes(&self, seed: usize) -> bool {
        seed >= self.start && seed < self.start + self.len
    }
}

fn parse_seeds(line: &String) -> Vec<usize> {
    line.split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec()
}

fn parse_seed_ranges(line: &String) -> Vec<SeedRange> {
    line.split(": ")
        .last()
        .unwrap()
        .split(" ")
        .chunks(2)
        .into_iter()
        .map(|c| {
            let parts = c.collect_vec();
            SeedRange {
                start: parts[0].parse::<usize>().unwrap(),
                len: parts[1].parse::<usize>().unwrap(),
            }
        })
        .collect_vec()
}

fn parse_almanac(lines: &Vec<String>) -> Vec<Vec<AlmanacEntry>> {
    lines
        .split(|l| l.is_empty())
        .map(|chunk| {
            chunk
                .into_iter()
                .skip(1)
                .map(|l| {
                    let parts = l.split(" ").collect::<Vec<&str>>();
                    AlmanacEntry {
                        source_start: parts[1].parse::<usize>().unwrap(),
                        source_end: parts[1].parse::<usize>().unwrap()
                            + parts[2].parse::<usize>().unwrap(),
                        target_start: parts[0].parse::<usize>().unwrap(),
                        len: parts[2].parse::<usize>().unwrap(),
                    }
                })
                .sorted()
                .collect_vec()
        })
        .collect_vec()
}

trait MergeRanges {
    fn merge_ranges(&self) -> Vec<SeedRange>;
}
impl MergeRanges for Vec<SeedRange> {
    fn merge_ranges(&self) -> Vec<SeedRange> {
        let mut merged = vec![];
        let mut curr = self[0].clone();
        self.iter().for_each(|r| {
            if curr.includes(r.start) {
                curr.len = max(curr.len, r.len + r.start - curr.start);
            } else if r.start == curr.start + curr.len {
                curr.len += r.len;
            } else {
                merged.push(curr.clone());
                curr = r.clone();
            }
        });
        merged.push(curr);
        merged
    }
}

trait AlmanacMapping {
    fn map_seed(&self, seed: &usize) -> usize;
    fn map_seed_range(&self, seed_range: &SeedRange) -> Vec<SeedRange>;
}
impl AlmanacMapping for Vec<AlmanacEntry> {
    fn map_seed(&self, seed: &usize) -> usize {
        match self.binary_search_by(|entry| entry.source_start.cmp(seed)) {
            Ok(idx) => self[idx].target_start,
            Err(idx) => {
                if idx == 0 || self[idx - 1].source_end <= *seed {
                    *seed
                } else {
                    self[idx - 1].target_start + seed - self[idx - 1].source_start
                }
            }
        }
    }

    fn map_seed_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let mut new_ranges = vec![];
        let mut new_start = seed_range.start;
        let mut remaining_len = seed_range.len;
        let mut entry_idx =
            match self.binary_search_by(|entry| entry.source_start.cmp(&seed_range.start)) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
        let before_any_entry = AlmanacEntry {
            source_start: 0,
            source_end: 0,
            target_start: 0,
            len: 0,
        };
        let after_any_entry = AlmanacEntry {
            source_start: usize::MAX,
            source_end: usize::MAX,
            target_start: usize::MAX,
            len: 0,
        };

        while remaining_len > 0 {
            let prev = if entry_idx > 0 {
                &self[entry_idx - 1]
            } else {
                &before_any_entry
            };
            let next = if entry_idx < self.len() {
                &self[entry_idx]
            } else {
                &after_any_entry
            };
            if prev.includes(new_start) {
                new_ranges.push(SeedRange {
                    start: prev.target_start + new_start - prev.source_start,
                    len: min(remaining_len, prev.source_end - new_start),
                });
                remaining_len -= min(remaining_len, prev.source_end - new_start);
                new_start = prev.source_end;
            }

            new_ranges.push(SeedRange {
                start: new_start,
                len: min(remaining_len, next.source_start - new_start),
            });
            remaining_len -= min(remaining_len, next.source_start - new_start);
            new_start = next.source_start;

            entry_idx += 1;
        }
        new_ranges
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let seeds = parse_seeds(&lines[0]);
        let almanac = parse_almanac(&lines[2..].to_vec());
        almanac
            .iter()
            .fold(seeds, |seeds, mapping| {
                seeds
                    .into_iter()
                    .map(|seed| mapping.map_seed(&seed))
                    .collect_vec()
            })
            .into_iter()
            .min()
            .unwrap()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let seed_ranges = parse_seed_ranges(&lines[0]);
        let almanac = parse_almanac(&lines[2..].to_vec());
        almanac
            .iter()
            .fold(seed_ranges, |ranges, mapping| {
                ranges
                    .iter()
                    .flat_map(|seed_range| mapping.map_seed_range(seed_range))
                    .filter(|r| r.len > 0)
                    .sorted()
                    .collect_vec()
                    .merge_ranges()
            })
            .into_iter()
            .min()
            .unwrap()
            .start
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
