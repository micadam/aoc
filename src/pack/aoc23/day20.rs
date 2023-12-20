use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

use std::hash::Hash;

use itertools::Itertools;
use itertools::PeekingNext;
use num::integer::lcm;

use crate::day::Day;
use crate::day::Solveable;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum ModuleType {
    NORMAL,
    FLIPFLOP,
    CONJUNCTION,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Signal {
    LOW,
    HIGH,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Module {
    id: String,
    type_: ModuleType,
    outputs: Vec<String>,
    inputs: HashMap<String, Signal>,
    on: bool,
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split(" -> ").collect_tuple().unwrap();
        let mut iter = id.chars().peekable();
        let mut type_: ModuleType = ModuleType::NORMAL;
        iter.peeking_next(|c| match c {
            '%' => {
                type_ = ModuleType::FLIPFLOP;
                true
            }
            '&' => {
                type_ = ModuleType::CONJUNCTION;
                true
            }
            _ => false,
        });
        let id = iter.collect::<String>();
        let outputs = rest.split(", ").map(|s| s.to_string()).collect();
        Ok(Module {
            id,
            type_,
            outputs,
            inputs: HashMap::new(),
            on: false,
        })
    }
}

type Emission = (String, Signal, String);

impl Module {
    fn emit(&self, signal: &Signal) -> Vec<Emission> {
        self.outputs
            .iter()
            .map(|o| (self.id.clone(), signal.clone(), o.clone()))
            .collect()
    }
    fn receive(&mut self, signal: &Signal, from: String) -> Vec<Emission> {
        match self.type_ {
            ModuleType::NORMAL => self.emit(signal),
            ModuleType::FLIPFLOP => {
                if *signal == Signal::HIGH {
                    return vec![];
                }
                let new_signal = if self.on { Signal::LOW } else { Signal::HIGH };
                self.on = !self.on;
                self.emit(&new_signal)
            }
            ModuleType::CONJUNCTION => {
                self.inputs.insert(from, signal.clone());
                if self.inputs.values().all(|v| *v == Signal::HIGH) {
                    self.emit(&Signal::LOW)
                } else {
                    self.emit(&Signal::HIGH)
                }
            }
        }
    }
}

fn get_map(lines: &Vec<String>) -> HashMap<String, Module> {
    let mut modules = lines
        .iter()
        .map(|s| s.parse::<Module>().unwrap())
        .collect_vec();
    let mut id_to_module = modules
        .iter()
        .map(|m| (m.id.clone(), m.clone()))
        .collect::<HashMap<String, Module>>();
    for module in modules.iter_mut() {
        for out_module in module.outputs.iter_mut() {
            if let Some(out_module) = id_to_module.get_mut(out_module) {
                out_module.inputs.insert(module.id.clone(), Signal::LOW);
            };
        }
    }
    id_to_module
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut id_to_module = get_map(lines);
        let mut q = VecDeque::new();
        let mut counts = HashMap::from([(Signal::LOW, 0usize), (Signal::HIGH, 0usize)]);
        for _ in 1..1001 {
            q.push_back(("button".to_string(), Signal::LOW, "broadcaster".to_string()));
            while let Some((from, signal, to)) = q.pop_front() {
                counts.entry(signal.clone()).and_modify(|c| *c += 1);
                if let Some(module) = id_to_module.get_mut(&to) {
                    let emissions = module.receive(&signal, from.to_string());
                    for (from, signal, to) in emissions {
                        q.push_back((from.to_string(), signal, to.to_string()));
                    }
                }
            }
        }
        counts.values().product::<usize>().to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut id_to_module = get_map(lines);
        let mut q = VecDeque::new();
        let rx_inputs = id_to_module
            .iter()
            .filter(|(_, m)| m.outputs.contains(&"rx".to_string()))
            .map(|(k, _)| k)
            .collect_vec();
        if rx_inputs.len() > 1 {
            panic!("Assumed only one input to rx, but found {:?}.", rx_inputs);
        }
        let mut rx_input_inputs: HashMap<String, Vec<usize>> = HashMap::new();
        for inp in id_to_module.get(rx_inputs[0]).unwrap().inputs.keys() {
            rx_input_inputs.insert(inp.clone(), vec![]);
        }
        let mut i = 0;
        loop {
            i += 1;
            q.push_back(("button".to_string(), Signal::LOW, "broadcaster".to_string()));
            while let Some((from, signal, to)) = q.pop_front() {
                if let Some(module) = id_to_module.get_mut(&to) {
                    let emissions = module.receive(&signal, from.to_string());
                    for (from, signal, to) in emissions {
                        q.push_back((from.to_string(), signal, to.to_string()));
                    }
                    if to == "hp" && signal == Signal::HIGH {
                        rx_input_inputs.entry(from).and_modify(|v| v.push(i));
                    }
                }
            }
            // Give some time for the loops to stabilize.
            if rx_input_inputs.values().all(|v| v.len() >= 5) {
                for (input, times) in rx_input_inputs.iter() {
                    if !times.iter().tuple_windows().map(|(a, b)| b - a).all_equal() {
                        panic!(
                            "Assumed all inputs would loop in a fixed interval, but {} did not.",
                            input
                        );
                    }
                    if times[1] != 2 * times[0] {
                        panic!("Assumed all loops would being at 0, but {} did not.", input);
                    }
                }
                return rx_input_inputs
                    .values()
                    .map(|v| v[0])
                    .fold(1, |a, b| lcm(a, b))
                    .to_string();
            }
        }
    }
}

get_day_fn!(Part1, Part2);
