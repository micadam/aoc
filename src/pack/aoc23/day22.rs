use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

use crate::day::{Day, Solveable};

#[derive(Debug, Clone)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once("~").unwrap();
        let start: (usize, usize, usize) = start_str
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let end: (usize, usize, usize) = end_str
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        assert!(
            start.0 <= end.0 && start.1 <= end.1 && start.2 <= end.2,
            "Assumed brick would be sorted"
        );
        Ok(Brick { start, end })
    }
}

fn stabilize(bricks: &[Brick]) -> Vec<Brick> {
    let mut bricks = bricks.to_vec();
    let mut z_plane_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        for z in brick.start.2..=brick.end.2 {
            let plane = z_plane_map.entry(z).or_insert_with(HashSet::new);
            plane.insert(i);
        }
    }

    let mut dropped = true;
    while dropped {
        dropped = false;
        let mut to_update = vec![];

        for (i, brick) in bricks.iter().enumerate() {
            let xy_tiles = (brick.start.0..=brick.end.0)
                .cartesian_product(brick.start.1..=brick.end.1)
                .collect::<HashSet<(usize, usize)>>();

            if brick.start.2 == 0 {
                continue; // Already at the bottom.
            }

            let z = brick.start.2 - 1;
            let bricks_here = z_plane_map.get(&z).unwrap_or(&HashSet::new()).clone();
            if bricks_here.iter().any(|j| {
                let supporting_brick = bricks.get(*j).unwrap();
                let xy_tiles_here = (supporting_brick.start.0..=supporting_brick.end.0)
                    .cartesian_product(supporting_brick.start.1..=supporting_brick.end.1)
                    .collect::<HashSet<(usize, usize)>>();

                xy_tiles.iter().any(|t| xy_tiles_here.contains(t))
            }) {
                continue;
            }

            dropped = true;
            to_update.push(i);
        }

        to_update.iter().for_each(|i| {
            let brick = bricks.get_mut(*i).unwrap();
            z_plane_map.entry(brick.end.2).and_modify(|e| {
                e.remove(i);
            });

            z_plane_map
                .entry(brick.start.2 - 1)
                .and_modify(|e| {
                    e.insert(*i);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(*i);
                    set
                });

            brick.start.2 -= 1;
            brick.end.2 -= 1;
        });
    }

    bricks
}

fn get_supported_by(bricks: &[Brick]) -> HashMap<usize, HashSet<usize>> {
    let mut z_plane_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        for z in brick.start.2..=brick.end.2 {
            let plane = z_plane_map.entry(z).or_insert_with(HashSet::new);
            plane.insert(i);
        }
    }

    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, supported_brick) in bricks.iter().enumerate() {
        let brick1_tiles = (supported_brick.start.0..=supported_brick.end.0)
            .cartesian_product(supported_brick.start.1..=supported_brick.end.1)
            .collect_vec();

        for z in (0..supported_brick.start.2).rev() {
            if !z_plane_map.contains_key(&z) {
                continue; // no bricks here
            }

            if supported_by.contains_key(&i) {
                break; // already supported, nothing to do
            }

            let plane = z_plane_map.get(&z).unwrap().clone();

            for j in plane.iter() {
                let supporting_brick = bricks.get(*j).unwrap();
                let brick2_tiles = (supporting_brick.start.0..=supporting_brick.end.0)
                    .cartesian_product(supporting_brick.start.1..=supporting_brick.end.1)
                    .collect_vec();

                if brick1_tiles
                    .iter()
                    .any(|t1| brick2_tiles.iter().any(|t2| *t1 == *t2))
                {
                    let sup = supported_by.entry(i).or_insert_with(HashSet::new);
                    sup.insert(*j);
                }
            }
        }
    }

    supported_by
}

struct Part1;

impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let bricks = lines
            .iter()
            .map(|l| l.parse::<Brick>().unwrap())
            .collect::<Vec<Brick>>();

        let bricks = stabilize(&bricks);
        let supported_by = get_supported_by(&bricks);

        (bricks.len()
            - supported_by
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .flat_map(|(_, v)| v.iter())
                .map(|v| *v)
                .collect::<HashSet<usize>>()
                .len())
        .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let bricks = lines
            .iter()
            .map(|l| l.parse::<Brick>().unwrap())
            .collect::<Vec<Brick>>();

        let bricks = stabilize(&bricks);
        let supported_by = get_supported_by(&bricks);

        let supporting = supported_by
            .iter()
            .flat_map(|(i, v)| v.iter().map(|j| (*j, *i)))
            .sorted()
            .group_by(|(i, _)| *i)
            .into_iter()
            .map(|(i, v)| (i, v.map(|(_, j)| j).collect()))
            .collect::<HashMap<usize, HashSet<usize>>>();

        // Topological sort to avoid digging too deep too early.
        let origin_nodes = (0..bricks.len())
            .filter(|i| !supported_by.contains_key(i))
            .collect::<Vec<usize>>();

        let mut removed_edges = HashSet::new();
        let mut topo_sorted = vec![];
        let mut q = VecDeque::new();
        q.extend(origin_nodes);

        while let Some(i) = q.pop_front() {
            topo_sorted.push(i);
            supporting.get(&i).map(|s| {
                s.iter().for_each(|j| {
                    removed_edges.insert((i, *j));

                    if supported_by
                        .get(j)
                        .map(|s| s.iter().all(|k| removed_edges.contains(&(*k, *j))))
                        .unwrap_or(false)
                    {
                        q.push_back(*j);
                    }
                })
            });
        }

        let topo_map = topo_sorted
            .iter()
            .enumerate()
            .map(|(i, j)| (*j, i))
            .collect::<HashMap<usize, usize>>();
        // end topo sort

        let mut ans = 0;
        // Starting nodes are any nodes that are the sole supporting node for another.
        // Removing any other node doesn't cause anything to fall.
        let starting_nodes = supported_by
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(_, v)| *v.iter().next().unwrap())
            .unique()
            .collect::<Vec<usize>>();

        starting_nodes.iter().for_each(|i| {
            let mut destroyed = HashSet::new();
            destroyed.insert(*i);

            topo_sorted[topo_map.get(i).unwrap() + 1..]
                .iter()
                .for_each(|i| {
                    if supported_by
                        .get(i)
                        .map(|s| s.iter().all(|j| destroyed.contains(j)))
                        .unwrap_or(false)
                    {
                        ans += 1;
                        destroyed.insert(*i);
                    }
                });
        });

        ans.to_string()
    }
}

get_day_fn!(Part1, Part2);
