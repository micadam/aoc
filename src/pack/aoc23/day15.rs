use core::panic;
use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn score(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        lines[0].split(",").map(score).sum::<usize>().to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut lenses: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
        lines[0].split(",").for_each(|s| {
            let label = s
                .chars()
                .take_while(|c| *c != '-' && *c != '=')
                .collect::<String>();
            let box_idx = score(&label);
            if !lenses.contains_key(&box_idx) {
                lenses.insert(box_idx, vec![]);
            }
            let b = lenses.get_mut(&box_idx).unwrap();
            match s.chars().nth(label.len()) {
                Some('=') => {
                    let focal = s.split('=').nth(1).unwrap().parse::<usize>().unwrap();
                    // Find index of box with label labael, or lebses[box_idx].len() if not found.
                    let idx = b.iter().position(|(l, _)| l == &label);
                    if let Some(idx) = idx {
                        b[idx].1 = focal;
                    } else {
                        b.push((label, focal));
                    }
                }
                Some('-') => {
                    let idx = b.iter().position(|(l, _)| l == &label);
                    if let Some(idx) = idx {
                        b.remove(idx);
                    }
                }
                _ => {
                    panic!("Invalid input {}", s);
                }
            }
        });
        lenses
            .iter()
            .fold(0, |acc, (box_idx, f)| {
                acc + (box_idx + 1)
                    * f.iter()
                        .enumerate()
                        .fold(0, |acc, (i, (_, f))| acc + f * (i + 1))
            })
            .to_string()
    }
}

get_day_fn!(Part1, Part2);
