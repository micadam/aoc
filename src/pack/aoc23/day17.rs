use crate::day::Day;
use crate::day::Solveable;

fn get_costs(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
// The graph node definition is a bit tricky because of all the limitations.
// In the end I settled on each node being defined by:
// * the position on the grid
// * the number of steps in a straight line so far
// * the current direction
struct GraphNode {
    pos: Position,
    current_steps: usize,
    dir: Direction,
}

const DIRS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_possible_neighbours(
    node: &GraphNode,
    max_row: usize,
    max_col: usize,
    min_line: usize,
    max_line: usize,
) -> Vec<GraphNode> {
    let mut neighbours = Vec::new();
    for dir in DIRS.iter() {
        // Can't turn straight back
        if (node.dir.0 + dir.0, node.dir.1 + dir.1) == (0, 0) {
            continue;
        }
        // Can't take more than max_line steps in a straight line
        if node.dir == *dir && node.current_steps >= max_line {
            continue;
        }
        // Can't take fewer than min_line steps in a straight line
        if node.dir != (0, 0) && node.dir != *dir && node.current_steps < min_line {
            continue;
        }
        let new_pos = (node.pos.0 as isize + dir.0, node.pos.1 as isize + dir.1);
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= max_row as isize
            || new_pos.1 >= max_col as isize
        {
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        neighbours.push(GraphNode {
            pos: new_pos,
            current_steps: if node.dir == *dir {
                node.current_steps + 1
            } else {
                1
            },
            dir: *dir,
        })
    }
    neighbours
}

fn find_min_path(chars: &Vec<Vec<usize>>, min_line: usize, max_line: usize) -> usize {
    let start = (0usize, 0usize);
    let goal = (chars.len() - 1, chars[0].len() - 1);

    let mut pq = std::collections::BinaryHeap::new();
    pq.push((
        0,
        GraphNode {
            pos: start,
            current_steps: 0,
            dir: (0, 0),
        },
    ));
    let mut visited = std::collections::HashSet::new();
    while let Some((cost, node)) = pq.pop() {
        let cost = -cost as usize;
        if node.pos == goal && node.current_steps >= min_line {
            return cost as usize;
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());
        for neighbour in
            get_possible_neighbours(&node, chars.len(), chars[0].len(), min_line, max_line)
        {
            let new_cost = cost + chars[neighbour.pos.0][neighbour.pos.1];
            pq.push((-(new_cost as isize), neighbour));
        }
    }
    panic!("No path found");
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        find_min_path(&get_costs(lines), 1, 3).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        find_min_path(&get_costs(lines), 4, 10).to_string()
    }
}

get_day_fn!(Part1, Part2);
