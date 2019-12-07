use crate::vm::{Word, VM};
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryInto;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> anyhow::Result<Vec<Word>> {
    input
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .context("Unable to parse input.")
}

#[aoc(day2, part1)]
pub fn part1(program: &[Word]) -> Word {
    let mut program = program.to_vec(); // because we need mutability for the current solution; and the aoc generator doesn't support it

    let mut vm = VM::new(&mut program);

    vm.execute().unwrap()
}

// brute force
#[aoc(day2, part2)]
pub fn part2(program: &[Word]) -> Word {
    let expected: Word = 19690720;

    const MAX: Word = 64;

    for noun in 0..=MAX {
        for verb in 0..=MAX {
            let mem = &mut program.to_vec();
            mem[1] = noun;
            mem[2] = verb;

            let mut vm = VM::new(mem);

            if let Ok(v) = vm.execute() {
                if v == expected {
                    dbg!(noun, verb);
                    return 100 * noun + verb;
                }
            }
        }
    }

    panic!("Unable to find compute")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup;

    ide!();

    fn inputs() -> anyhow::Result<Vec<Word>> {
        setup(2, parse_input)
    }

    #[parameterized(
        input = {
            &mut [1,9,10,3,2,3,11,0,99,30,40,50],
            &mut [1,0,0,0,99],
            &mut [2,3,0,3,99],
            &mut [2,4,4,5,99,0],
            &mut [1,1,1,4,99,5,6,0,99]
        },
        expected = {
            3500,
            2,
            2,
            2,
            30,
        },
    )]
    fn part1_aoc_from_start(input: &mut [Word], expected: Word) {
        let mut vm = VM::new(input);
        assert_eq!(vm.execute().unwrap(), expected);
    }

    #[test]
    fn part1_aoc_with_error_state() {
        let mem = &mut inputs().unwrap();
        mem[1] = 12;
        mem[2] = 2;

        let mut vm = VM::new(mem);

        assert_eq!(vm.execute().unwrap(), 3895705);
    }

    #[test]
    fn part2_aoc_brute_force() {
        assert_eq!(part2(&mut inputs().unwrap()), 6417);
    }
}
