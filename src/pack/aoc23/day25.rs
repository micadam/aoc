use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::AddAssign;

use crate::day::Day;
use crate::day::Solveable;

type UnweightedGraph = HashMap<String, HashSet<String>>;
type WeightedGraph = HashMap<String, HashMap<String, usize>>;

fn get_connections(lines: &Vec<String>) -> UnweightedGraph {
    let mut connections = HashMap::new();
    for line in lines {
        let (from, to) = line.split_once(": ").unwrap();
        to.split_whitespace().for_each(|s| {
            connections
                .entry(s.to_string())
                .or_insert(HashSet::new())
                .insert(from.to_string());
            connections
                .entry(from.to_string())
                .or_insert(HashSet::new())
                .insert(s.to_string());
        });
    }
    connections
}

fn minimum_cut_phase(graph: &WeightedGraph, magnitude: &HashMap<String, usize>, vertex: String) -> (WeightedGraph, HashMap<String, usize>, usize, usize) {
    let mut added = HashSet::new();
    let mut magnitude = magnitude.clone();
    added.insert(vertex.clone());
    let mut mega_vertex = graph.get(&vertex).unwrap().clone();
    let mut last_added_nodes = vec![vertex.clone()];
    let mut cut_of_the_phase = usize::MAX;
    let mut magnitude_of_the_phase = usize::MAX;
    while added.len() < graph.len() {
        let max_vertex = mega_vertex
            .iter()
            .max_by_key(|(_, v)| *v)
            .unwrap()
            .0
            .clone();
        added.insert(max_vertex.clone());
        last_added_nodes.push(max_vertex.clone());
        for edge in graph.get(&max_vertex).unwrap() {
            if added.contains(edge.0) {
                continue;
            }
            mega_vertex
                .entry(edge.0.clone())
                .and_modify(|v| *v += edge.1)
                .or_insert(0)
                .add_assign(edge.1)
        }
        mega_vertex.remove(&max_vertex);
        if added.len() == graph.len() - 1 {
            assert!(mega_vertex.len() == 1);
            cut_of_the_phase = mega_vertex.iter().next().unwrap().1.clone();
            magnitude_of_the_phase = magnitude.get(mega_vertex.iter().next().unwrap().0).unwrap().clone();
        }
        
    }
    let mut last_added_nodes = (
        last_added_nodes.pop().unwrap(),
        last_added_nodes.pop().unwrap(),
    );
    if vertex == last_added_nodes.1 {
        // swap them
        last_added_nodes = (last_added_nodes.1, last_added_nodes.0);
    }
    let mut new_graph = graph.clone();
    // last_added_nodes.1 gets merged into last_added_nodes.0
    for edge in new_graph.get(&last_added_nodes.1).unwrap().clone() {
        if *edge.0 == last_added_nodes.0 {
            new_graph
                .get_mut(&last_added_nodes.0)
                .unwrap()
                .remove(&last_added_nodes.1);
            continue;
        }
        // modify the edge from edge.0 to last_added_nodes.1 so that it points to last_added_nodes.0
        new_graph.entry(edge.0.clone()).and_modify(|s| {
            let w = s.get(&last_added_nodes.1).unwrap().clone();
            s.remove(&last_added_nodes.1);
            s.entry(last_added_nodes.0.clone())
                .or_insert(0)
                .add_assign(w);
        });
        new_graph
            .get_mut(&last_added_nodes.0)
            .unwrap()
            .entry(edge.0.clone())
            .or_insert(0)
            .add_assign(edge.1);
    }
    let old_magnitude = magnitude.get(&last_added_nodes.1).unwrap().clone();
    magnitude
        .entry(last_added_nodes.0.clone())
        .or_insert(1)
        .add_assign(old_magnitude);
    magnitude.remove(&last_added_nodes.1);
    new_graph.remove(&last_added_nodes.1);
    (new_graph, magnitude, cut_of_the_phase, magnitude_of_the_phase)
}

fn minimum_cut(graph: &UnweightedGraph) -> usize {
    let mut graph: WeightedGraph = graph
        .iter()
        .map(|(k, v)| (k.clone(), v.iter().map(|s| (s.clone(), 1)).collect()))
        .collect();
    let vertex_count = graph.len();
    let vertex = graph.keys().next().unwrap().clone();
    let mut magnitude = graph.keys().map(|s| (s.clone(), 1)).collect();
    let mut min_cut_of_the_phase = usize::MAX;
    let mut magnitude_of_min_phase = usize::MAX;
    while graph.len() > 1 {
        let cut_of_the_phase;
        let magnitude_of_the_phase;
        (graph, magnitude, cut_of_the_phase, magnitude_of_the_phase) = minimum_cut_phase(&graph, &magnitude, vertex.clone());
        if cut_of_the_phase < min_cut_of_the_phase {
            min_cut_of_the_phase = cut_of_the_phase;
            magnitude_of_min_phase = magnitude_of_the_phase;
        }
    }
    magnitude_of_min_phase * (vertex_count - magnitude_of_min_phase)
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let connections = get_connections(lines);
        minimum_cut(&connections).to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        "Merry Christmas".to_string()
    }
}

get_day_fn!(Part1, Part2);
