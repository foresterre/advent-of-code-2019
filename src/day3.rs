#![allow(dead_code)]

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Result<Vec<Vec<Twist>>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.split_at(1))
                .map(|(dir, amount)| {
                    let amount = amount.parse().with_context(|| "Not a valid number.")?;

                    Ok(Twist::new(dir, amount))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

#[aoc(day3, part1)]
fn part1(wires: &[Vec<Twist>]) -> u32 {
    let mut circuit = HashMap::new();

    draw_wires(&mut circuit, wires);

    circuit
        .into_iter()
        .map(|((x, y), vec)| ((x, y), GridMatches::new(vec)))
        .filter(|(_, matches)| matches.contains(0) && matches.contains(1))
        .map(|((x, y), _)| manhattan(x, y))
        .min()
        .expect("No overlapping lines found...")
}

#[aoc(day3, part2)]
fn part2(wires: &[Vec<Twist>]) -> usize {
    let mut circuit = HashMap::new();

    draw_wires(&mut circuit, wires);

    let map = circuit
        .into_iter()
        .map(|((x, y), vec)| ((x, y), GridMatches::new(vec)))
        .filter_map(|(_, grid_matches)| {
            let line0: Option<CableMatch> = grid_matches.get(0);
            let line1: Option<CableMatch> = grid_matches.get(1);

            match (line0, line1) {
                (Some(l0), Some(l1)) => Some(l0.length.0 + l1.length.0),
                _ => None,
            }
        })
        .min();

    map.expect("No overlapping lines found...")
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Twist {
    direction: Direction,
    distance: u32,
}

impl Twist {
    fn new(direction: &str, distance: u32) -> Self {
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unable to parse inputs"),
        };

        Self {
            direction,
            distance,
        }
    }
}

struct GridMatches {
    indexed_map: HashMap<usize, CableMatch>,
}

impl GridMatches {
    fn new(vec: Vec<CableMatch>) -> Self {
        Self {
            indexed_map: vec.iter().map(|cm| (cm.line, *cm)).collect(),
        }
    }

    fn get(&self, line: usize) -> Option<CableMatch> {
        self.indexed_map.get(&line).cloned()
    }

    fn contains(&self, line: usize) -> bool {
        self.indexed_map.get(&line).is_some()
    }
}

// (which line, length of match)
#[derive(Copy, Clone)]
struct CableMatch {
    line: usize,
    length: Len,
}

impl CableMatch {
    fn new(line: usize, len: Len) -> CableMatch {
        CableMatch { line, length: len }
    }
}

impl PartialEq for CableMatch {
    fn eq(&self, other: &Self) -> bool {
        self.line.eq(&other.line)
    }
}

impl Eq for CableMatch {}

impl PartialOrd for CableMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length.0.partial_cmp(&other.length.0)
    }
}

impl Ord for CableMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.0.cmp(&other.length.0)
    }
}

#[derive(Default, Copy, Clone)]
struct Len(usize);

impl Len {
    fn incr(self) -> Len {
        Len(self.0 + 1)
    }
}

fn draw_wires(circuit: &mut HashMap<(i32, i32), Vec<CableMatch>>, wires: &[Vec<Twist>]) {
    for (n, line) in wires.iter().enumerate() {
        // draw out a cable
        line.iter().fold((0, 0, Len(0)), |(fx, fy, length), twist| {
            // draw out each current cable coordinate
            (0..twist.distance).fold((fx, fy, length), |(x, y, len), _i| {
                let coord = match twist.direction {
                    Direction::Right => (x + 1, y),
                    Direction::Left => (x - 1, y),
                    Direction::Up => (x, y + 1),
                    Direction::Down => (x, y - 1),
                };

                let incr = len.incr();

                // if we find a match
                // push the line number, + the current line length.
                circuit
                    .entry((coord.0, coord.1))
                    .or_insert_with(|| vec![])
                    .push(CableMatch::new(n, incr));

                (coord.0, coord.1, incr)
            })
        });
    }
}

fn manhattan(a: i32, b: i32) -> u32 {
    (a.abs() + b.abs()) as u32
}
