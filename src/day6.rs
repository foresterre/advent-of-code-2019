use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

// The id of an object in space.
type Id = String;
type Weight = u32;

// A single orbiter around a center as parsed from an orbit map transmission.
#[derive(Debug, Default, Hash)]
struct OrbitRelation {
    center: String, // pfffff cargo-aoc and lifetimes
    orbiter: String,
}

impl<T> FromIterator<T> for OrbitRelation
where
    T: AsRef<str>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OrbitRelation {
        let mut iterator = iter.into_iter();

        let center = iterator.next().expect("Orbit relation requires a center");
        let orbiter = iterator.next().expect("Orbit relation requires an orbiter");

        OrbitRelation {
            center: center.as_ref().to_string(),
            orbiter: orbiter.as_ref().to_string(),
        }
    }
}

// orbital graph, actual name to index
// values, i.e. names should be in graph too somewhere but this is a whole lot easier.
#[derive(Debug)]
struct OrbitalMap(Graph<Id, Weight>, HashMap<String, NodeIndex>);
type DGraph = Graph<Id, Weight>;
type UGraph = Graph<Id, Weight, petgraph::Undirected>;

fn add_node(
    graph: &mut Graph<Id, Weight>,
    map: &mut HashMap<String, NodeIndex>,
    parent: String,
) -> NodeIndex {
    if map.contains_key(&parent) {
        *map.get(parent.as_str()).unwrap()
    } else {
        let id = graph.add_node(parent.clone());
        map.insert(parent, id);
        id
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> OrbitalMap {
    let vec = input
        .lines()
        .map(|line| line.split(')').collect::<OrbitRelation>())
        .collect::<Vec<OrbitRelation>>();

    let mut graph = DGraph::new();

    // Stores the indexes of parents in the graph we already know about.
    // I am not sure how to do this cleanly with petgraph right now...
    let mut map: HashMap<String, NodeIndex> = HashMap::new();

    for relation in vec {
        let parent = relation.center;
        let child = relation.orbiter;

        let center = add_node(&mut graph, &mut map, parent);
        let orbiter = add_node(&mut graph, &mut map, child);

        // directed edge: orbiter -> object
        //        graph.add_edge(center, orbiter, 1);
        graph.add_edge(orbiter, center, 1);
    }

    OrbitalMap(graph, map)
}

#[aoc(day6, part1)]
fn part1(graph: &OrbitalMap) -> Weight {
    let com = *graph.1.get("COM").unwrap();
    let graph = graph.0.clone();

    graph
        .node_indices()
        .map(|vertex| {
            let (cost, _) =
                astar(&graph, vertex, |node| node == com, |e| *e.weight(), |_| 0).unwrap();

            cost
        })
        .sum()
}

// we'll take a shortcut:
// - 1. calc shortest path between YOU and SAN
// - 2. subtract 2 (this only works if path between you and santa >= 2,
//   but we'll assume it is as it makes our life easier =D).
#[aoc(day6, part2)]
fn part2(graph: &OrbitalMap) -> Weight {
    let you = *graph.1.get("YOU").unwrap();
    let santa = *graph.1.get("SAN").unwrap();

    let graph = graph.0.clone();
    let undirected = UGraph::from_edges(
        graph
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), e.weight)),
    );

    let (cost, _) = astar(
        &undirected,
        you,
        |node| node == santa,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    if cost >= 2 {
        cost - 2
    } else {
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup;

    #[test]
    fn total_orbital_lengths() {
        let input: String = ["COM)B", "B)D", "B)C"].join("\n");
        let orbits = parse_input(&input);
        let sum = part1(&orbits);

        assert_eq!(sum, 5);
    }

    #[test]
    fn part2_shortest_path_to_santa_0_jumps() {
        let input: String = ["COM)B", "B)YOU", "B)SAN"].join("\n");
        let orbits = parse_input(&input);
        let sum = part2(&orbits);

        assert_eq!(sum, 0);
    }

    #[test]
    fn part2_shortest_path_to_santa_1_jump() {
        let input: String = ["COM)B", "B)YOU", "B)C", "C)SAN"].join("\n");
        let orbits = parse_input(&input);
        let sum = part2(&orbits);

        assert_eq!(sum, 1);
    }

    #[test]
    fn part2_shortest_path_to_santa_5_jumps() {
        let input: String = [
            "COM)B", "B)YOU", "B)C", "C)D", "D)E", "E)F", "F)G", "F)H", "H)SAN",
        ]
        .join("\n");
        let orbits = parse_input(&input);
        let sum = part2(&orbits);

        assert_eq!(sum, 5);
    }

    fn problem_input() -> OrbitalMap {
        fn wrapped(input: &str) -> anyhow::Result<OrbitalMap> {
            Ok(parse_input(input))
        }
        setup(6, wrapped).unwrap()
    }

    #[test]
    fn part1_answer() {
        assert_eq!(part1(&problem_input()), 194721);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(part2(&problem_input()), 316);
    }
}
