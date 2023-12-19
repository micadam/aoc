use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

use crate::day::Day;
use crate::day::Solveable;

// boring struct declaration starts here
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug)]
enum ConditionType {
    LT,
    GT,
    NONE,
}

#[derive(Clone, Debug)]
struct Condition {
    property: Property,
    type_: ConditionType,
    value: isize,
    outcome: String,
}

#[derive(Clone, Debug)]
struct Rule {
    id: String,
    conditions: Vec<Condition>,
}

#[derive(Debug)]
struct Part {
    values: HashMap<Property, isize>,
}

struct Input {
    rules: HashMap<String, Rule>,
    parts: Vec<Part>,
}
// boring struct declaration ends here

// boring parsing code begins here
impl FromStr for Property {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Property::X),
            "m" => Ok(Property::M),
            "a" => Ok(Property::A),
            "s" => Ok(Property::S),
            _ => panic!("Invalid property {}", s),
        }
    }
}

impl FromStr for Condition {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.find("<").is_none() && value.find(">").is_none() {
            return Ok(Condition {
                property: Property::X,
                type_: ConditionType::NONE,
                value: 0,
                outcome: value.to_string(),
            });
        }
        let (property_str, rest) =
            value.split_at(value.find(|c: char| !c.is_alphabetic()).unwrap());
        let property = property_str.parse::<Property>().unwrap();
        let type_str = rest.chars().next().unwrap().to_string();
        let type_ = match type_str.as_str() {
            "<" => ConditionType::LT,
            ">" => ConditionType::GT,
            _ => panic!("Invalid condition type {}", type_str),
        };
        let (value_str, rest) =
            rest[1..].split_at(rest[1..].find(|c: char| !c.is_numeric()).unwrap());
        let value = value_str.parse::<isize>().unwrap();
        let outcome = rest[1..].to_string();
        Ok(Condition {
            property,
            type_,
            value,
            outcome,
        })
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.split("{").collect::<Vec<&str>>();
        let [id, rest] = binding.as_slice() else {
            panic!("Invalid rule");
        };
        let id = id.to_string();
        let conditions = rest
            .replace("}", "")
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect::<Vec<Condition>>();
        Ok(Rule { id, conditions })
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut values = HashMap::new();
        str.replace("{", "")
            .replace("}", "")
            .split(",")
            .for_each(|p_str| {
                let binding = p_str.split("=").collect::<Vec<&str>>();
                let [property, value] = binding.as_slice() else {
                    panic!("Invalid part");
                };
                let property = property.parse::<Property>().unwrap();
                let value = value.parse::<isize>().unwrap();
                values.insert(property, value);
            });
        Ok(Part { values })
    }
}

impl From<&Vec<String>> for Input {
    fn from(value: &Vec<String>) -> Self {
        let [rules_str, parts_str] = *value
            .split(|l| l.is_empty())
            .collect::<Vec<&[String]>>()
            .as_slice()
        else {
            panic!("Invalid input");
        };
        let rules = rules_str
            .iter()
            .map(|l| l.parse().unwrap())
            .map(|r: Rule| (r.id.clone(), r.clone()))
            .collect::<HashMap<String, Rule>>();
        let parts = parts_str
            .iter()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Part>>();

        Input { rules, parts }
    }
}
// boring parsing code ends here

// part 1 code starts here
impl ConditionType {
    fn test(&self, lhs: &isize, rhs: &isize) -> bool {
        match self {
            ConditionType::LT => lhs < rhs,
            ConditionType::GT => lhs > rhs,
            ConditionType::NONE => true,
        }
    }
}

impl Condition {
    fn apply(&self, part: &Part) -> Option<String> {
        let value = part.values.get(&self.property);
        if value.is_none() {
            panic!("Each part should have all values!");
        }
        let value = value.unwrap();
        if self.type_.test(value, &self.value) {
            return Some(self.outcome.clone());
        } else {
            return None;
        }
    }
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<String> {
        for condition in &self.conditions {
            if let Some(outcome) = condition.apply(part) {
                return Some(outcome);
            }
        }
        panic!("Each rule should match in at least one condition!");
    }
}
// part 1 code ends here

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let input: Input = lines.into();
        input
            .parts
            .into_iter()
            .filter_map(|p| {
                let mut loc = "in".to_string();
                while !["A", "R"].contains(&loc.as_str()) {
                    let rule = input.rules.get(loc.as_str()).unwrap();
                    loc = rule.apply(&p).unwrap().to_string();
                }
                match loc.as_str() {
                    "A" => Some(p),
                    "R" => None,
                    _ => panic!("Invalid location {}", loc),
                }
            })
            .map(|p| p.values.values().sum::<isize>())
            .sum::<isize>()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let max_value = 4000usize;
        let input: Input = lines.into();
        let mut q = VecDeque::new();
        let mut ans = 0usize;
        q.push_back(("in".to_string(), HashMap::from([
            (Property::X, (1..max_value+1)),
            (Property::M, (1..max_value+1)),
            (Property::A, (1..max_value+1)),
            (Property::S, (1..max_value+1)),
        ])));
        while let Some((loc, counts)) = q.pop_front() {
            if counts.values().map(|r| r.len()).product::<usize>() == 0 {
                continue;
            }
            if loc == "A" {
                ans += counts.values().map(|r| r.len()).product::<usize>();
                continue;
            } else if loc == "R" {
                continue;
            }
            let rule = input.rules.get(loc.as_str()).unwrap();
            let mut new_counts = counts.clone();
            for condition in rule.conditions.iter() {
                let value = new_counts.get(&condition.property).unwrap().clone();
                match condition.type_ {
                    ConditionType::LT => {
                        if (condition.value as usize) < value.start {
                            // no match -- skip condition
                            continue;
                        }
                        else if (condition.value as usize) < value.end {
                            // partial match -- split off
                            let mut new_value = value.clone();
                            new_value.end = condition.value as usize;
                            let mut split_counts = new_counts.clone();
                            split_counts.insert(condition.property.clone(), new_value);
                            q.push_back((condition.outcome.clone(), split_counts));
                            let value = new_counts.get_mut(&condition.property).unwrap();
                            value.start = condition.value as usize;
                        } else {
                            // full match -- consume all
                            q.push_back((condition.outcome.clone(), new_counts.clone()));
                            break;
                        }
                    }
                    ConditionType::GT => {
                        if (condition.value as usize) >= value.end {
                            // no match -- skip condition
                            continue;
                        }
                        else if (condition.value as usize) >= value.start {
                            // partial match -- split off
                            let mut new_value = value.clone();
                            new_value.start= condition.value as usize + 1;
                            let mut split_counts = new_counts.clone();
                            split_counts.insert(condition.property.clone(), new_value);
                            q.push_back((condition.outcome.clone(), split_counts));
                            let value = new_counts.get_mut(&condition.property).unwrap();
                            value.end = condition.value as usize + 1;
                        } else {
                            // full match -- consume all
                            q.push_back((condition.outcome.clone(), new_counts.clone()));
                            break;
                        }
                    }
                    ConditionType::NONE => {
                        // accept all
                        q.push_back((condition.outcome.clone(), new_counts.clone()));
                    }
                }
            }
        }
        ans.to_string()
    }
}

get_day_fn!(Part1, Part2);
