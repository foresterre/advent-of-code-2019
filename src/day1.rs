use aoc_runner_derive::{aoc, aoc_generator};
use std::iter;

type Mass = i32;
type Fuel = i32;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<Mass>, std::num::ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn calc_fuel(mass: Mass) -> Fuel {
    mass / 3 - 2
}

#[aoc(day1, part1)]
fn part1(mass: &[Mass]) -> Fuel {
    mass.iter().map(|v| calc_fuel(*v)).sum()
}

fn pfff(mut mass: Mass) -> Fuel {
    let mut fuel = 0;

    while mass > 0 {
        let grossed = calc_fuel(mass);
        if grossed > 0 {
            fuel += grossed;
            mass = grossed;
        } else {
            break;
        }
    }

    fuel
}

#[aoc(day1, part2)]
fn part2(mass: &[Mass]) -> Fuel {
    mass.iter().map(|v| pfff(*v)).sum()
}

trait PositiveMass {
    #[inline]
    fn ensure_mass_positive(self) -> Option<Mass>
    where
        Self: Into<Mass>,
    {
        let mass = self.into();

        if mass <= 0 {
            None
        } else {
            Some(mass)
        }
    }
}

impl PositiveMass for Mass {}

fn iterator(mass: Mass) -> impl Iterator<Item = Mass> {
    iter::successors(Some(mass), |next| calc_fuel(*next).ensure_mass_positive()).skip(1)
}

#[aoc(day1, part2, Iterator)]
fn part2_iterator(mass: &[Mass]) -> Fuel {
    let grossed_fuel: fn(i32) -> i32 = |mass: Mass| iterator(mass).sum();
    mass.iter().map(|v| grossed_fuel(*v)).sum()
}

fn recursive(mass: Mass) -> Fuel {
    let fuel = calc_fuel(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + recursive(fuel)
    }
}

#[aoc(day1, part2, Recursive)]
fn part2_recursive(mass: &[Mass]) -> Fuel {
    mass.iter().map(|v| recursive(*v)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    ide!();

    fn setup_answer() -> Vec<Mass> {
        let mut input = String::new();
        std::fs::File::open("input/2019/day1.txt")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();

        parse_input_day1(&input).unwrap()
    }

    #[parameterized(
        mass = {
            &setup_answer(), // my answer input
            &[14],
            &[1969],
            &[100756]
        },
        expected = {
            5139037,        // my answer
            2,
            966,
            50346
        },
    )]
    fn part2_aoc(mass: &[Mass], expected: Fuel) {
        assert_eq!(part2(mass), expected);
        assert_eq!(part2_iterator(&mass), expected);
        assert_eq!(part2_recursive(mass), expected);
    }
}
