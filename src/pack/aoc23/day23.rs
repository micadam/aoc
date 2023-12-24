use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::day::Day;
use crate::day::Solveable;

fn get_chars(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Edge {
    start: (usize, usize),
    end: (usize, usize),
    end_node: (usize, usize),
    length: usize,
}

trait Grid {
    fn find_only_dot_in_row(&self, row: usize) -> (usize, usize);
    fn iter_neighbours(&self, r: usize, c: usize, slopes: bool) -> Vec<(usize, usize)>;
    fn preprocess_graph(&self, slopes: bool) -> HashMap<(usize, usize), HashSet<Edge>>;
}

impl Grid for Vec<Vec<char>> {
    fn find_only_dot_in_row(&self, row: usize) -> (usize, usize) {
        self[row]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .map(|(i, _)| (row, i))
            .unwrap()
    }

    fn iter_neighbours(&self, r: usize, c: usize, slopes: bool) -> Vec<(usize, usize)> {
        let dirs = if slopes {
            match self[r][c] {
                '^' => vec![(-1, 0)],
                'v' => vec![(1, 0)],
                '<' => vec![(0, -1)],
                '>' => vec![(0, 1)],
                _ => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            }
        } else {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        };
        let mut neighbours = Vec::new();
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < self.len() as isize && nc >= 0 && nc < self[0].len() as isize {
                match self[nr as usize][nc as usize] {
                    '#' => continue,
                    '.' | '^' | '>' | 'v' | '<' => neighbours.push((nr as usize, nc as usize)),
                    _ => panic!("Unknown char: {}", self[r as usize][c as usize]),
                }
            }
        }
        neighbours
    }
    fn preprocess_graph(&self, slopes: bool) -> HashMap<(usize, usize), HashSet<Edge>> {
        // graph vertices = any places with more than 2 neighbours, and the start and end.
        // graph edges = stretches of '.' with only 2 neighbours
        let mut graph = HashMap::new();
        let start = self.find_only_dot_in_row(0);
        let end = self.find_only_dot_in_row(self.len() - 1);
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut edges = HashMap::new();
        while let Some(pos) = queue.pop_front() {
            if graph.contains_key(&pos) {
                continue;
            }
            let neighbours = self.iter_neighbours(pos.0, pos.1, slopes);
            assert!(pos == start || pos == end || neighbours.len() > 2,);
            for neigh in neighbours {
                if !edges.contains_key(&neigh) {
                    let mut neigh = neigh;
                    let edge_start = neigh;
                    let mut edge_end = neigh;
                    let mut edge_len = 0;
                    let mut predecessor = pos;
                    let mut end_node = pos;
                    loop {
                        let neighs = self.iter_neighbours(neigh.0, neigh.1, slopes);
                        if neigh == end || neighs.len() > 2 {
                            queue.push_back(neigh);
                            end_node = neigh;
                            break;
                        }
                        edge_end = neigh;
                        let old_neigh = neigh;
                        let neigh_opt = neighs.into_iter().find(|n| *n != predecessor);
                        if neigh_opt.is_none() {
                            break; // dead end
                        }
                        neigh = neigh_opt.unwrap();
                        predecessor = old_neigh;
                        edge_len += 1;
                    }
                    let edge = Edge {
                        start: edge_start,
                        end: edge_end,
                        length: edge_len,
                        end_node: end_node,
                    };
                    if edge_len == 0 {
                        continue; // Can't move in this directoin
                    }
                    edges.insert(edge.start, edge.clone());
                    if !slopes
                        || self[edge.start.0][edge.start.1] == '.'
                            && self[edge.end.0][edge.end.1] == '.'
                    {
                        let rev_edge = Edge {
                            start: edge.end,
                            end: edge.start,
                            length: edge.length,
                            end_node: pos,
                        };
                        edges.insert(rev_edge.start, rev_edge);
                    }
                }
                let edge = edges.get(&neigh).unwrap().clone();
                let mut set = HashSet::new();
                graph
                    .entry(pos)
                    .and_modify(|s: &mut HashSet<Edge>| {
                        s.insert(edge.clone());
                    })
                    .or_insert_with(|| {
                        set.insert(edge);
                        set
                    });
            }
        }
        graph
    }
}

fn get_longest_hike(chars: &Vec<Vec<char>>, slopes: bool) -> usize {
    let start = chars.find_only_dot_in_row(0);
    let end = chars.find_only_dot_in_row(chars.len() - 1);

    let graph = chars.preprocess_graph(slopes);
    let mut longest_path = 0;
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    q.push_back((start, visited, 0));
    while let Some((pos, visited, cur_len)) = q.pop_back() {
        if pos == end {
            longest_path = longest_path.max(cur_len);
            continue;
        }
        let candidates = graph.get(&pos).unwrap();
        if let Some(edge) = candidates.iter().find(|e| e.end_node == end) {
            longest_path = longest_path.max(cur_len + edge.length + 1);
            continue; // if we can reach the end, we can't take any other path as it would block the end.
        }
        for new_pos in graph.get(&pos).unwrap().iter() {
            
            if !visited.contains(&new_pos.end_node) {
                let mut new_visited = visited.clone();
                new_visited.insert(new_pos.end_node);
                q.push_back((new_pos.end_node, new_visited, cur_len + new_pos.length + 1));
            }
        }
    }
    longest_path
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = get_chars(lines);
        get_longest_hike(&chars, true).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let chars = get_chars(lines);
        get_longest_hike(&chars, false).to_string()
    }
}

get_day_fn!(Part1, Part2);
