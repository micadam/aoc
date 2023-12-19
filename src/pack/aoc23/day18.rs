use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

type Instruction = (char, isize);
type Vertex = (isize, isize);

fn parse_line(line: &String) -> Instruction {
    let parts = line.split_whitespace().collect_vec();
    let dir = parts[0].chars().next().unwrap();
    let steps = parts[1].parse().unwrap();

    (dir, steps)
}

fn parse_line_p2(line: &String) -> Instruction {
    let parts = line.split_whitespace().collect_vec();
    let color = parts[2]
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    let dir = match color.chars().nth(5) {
        Some('0') => 'R',
        Some('1') => 'D',
        Some('2') => 'L',
        Some('3') => 'U',
        c => panic!("Invalid color {:?}", c),
    };
    // steps is a hexadecimal string
    let steps = isize::from_str_radix(&color[..5], 16).unwrap();

    (dir, steps)
}

fn get_vertices(instructions: &Vec<Instruction>) -> Vec<Vertex> {
    let mut vertices = vec![];

    let mut pos = (0, 0);
    vertices.push(pos);

    let mut last_dir = 'X';

    for (dir, steps) in instructions {
        if *dir == last_dir {
            panic!("Repeat detected");
        }
        pos = match dir {
            'R' => (pos.0, pos.1 + steps),
            'L' => (pos.0, pos.1 - steps),
            'U' => (pos.0 + steps, pos.1),
            'D' => (pos.0 - steps, pos.1),
            c => panic!("Invalid direction {:?}", c),
        };
        last_dir = *dir;
        vertices.push(pos);
    }
    vertices
}

fn get_area(vertices: &Vec<Vertex>) -> isize {
    let area = vertices.windows(2).fold(0, |acc, p| match p {
        // the last part is to account for the half-pixels missed
        &[(a, b), (c, d)] => acc + a * d - b * c + (c - a).abs() + (d - b).abs() - 1,
        _ => panic!("Invalid vertices"),
    }) / 2;

    // For some reason the area missed from the corners equal to ceil(vertices.len() / 2)
    // (both on the test input and my input)
    // TODO figure it out
    area + (vertices.len() as isize + 1) / 2
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let instructions = lines.iter().map(parse_line).collect_vec();
        let vertices = get_vertices(&instructions);
        let area = get_area(&vertices);
        area.to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let instructions = lines.iter().map(parse_line_p2).collect_vec();
        let vertices = get_vertices(&instructions);
        let area = get_area(&vertices);
        area.to_string()
    }
}

get_day_fn!(Part1, Part2);
