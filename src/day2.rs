use anyhow::{bail, Context};
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::ops::{Add, Mul};

type Opcode = usize;
type Index = usize;
type Value = usize;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> anyhow::Result<Vec<Opcode>> {
    input
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<Opcode>, ParseIntError>>()
        .context("Unable to parse input.")
}

const OPCODE_ADD: Opcode = 1;
const OPCODE_MUL: Opcode = 2;
const OPCODE_RET: Opcode = 99;

fn compute(tape: &mut [Opcode], mut pc: Index, noun: Index, verb: Index) -> anyhow::Result<Opcode> {
    tape[1] = noun;
    tape[2] = verb;

    while tape[pc] != OPCODE_RET {
        pc = match tape[pc] {
            OPCODE_ADD => binop(Value::add)(tape, pc),
            OPCODE_MUL => binop(Value::mul)(tape, pc),
            _ => bail!("Unable to compute. Rejected program."),
        }
    }

    Ok(tape[0])
}

fn binop<F>(binop: F) -> impl Fn(&mut [Opcode], Index) -> Index
where
    F: Fn(Value, Value) -> Value,
{
    move |tape: &mut [Opcode], pc: Index| {
        let pc1 = tape[pc + 1];
        let pc2 = tape[pc + 2];
        let pc3 = tape[pc + 3];

        tape[pc3] = binop(tape[pc1], tape[pc2]);

        pc + 4
    }
}

#[aoc(day2, part1)]
pub fn part1(program: &[Opcode]) -> Value {
    let mut program = program.to_vec(); // because we need mutability for the current solution; and the aoc generator doesn't support it
    compute(&mut program, 0, 12, 2).unwrap()
}

// brute force
#[aoc(day2, part2)]
pub fn part2(program: &[Opcode]) -> Value {
    let expected: usize = 19690720;

    const MAX: usize = 64;

    for noun in 0..=MAX {
        for verb in 0..=MAX {
            if let Ok(v) = compute(&mut program.to_vec(), 0, noun, verb) {
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

    fn inputs() -> anyhow::Result<Vec<Opcode>> {
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
    fn part1_aoc_from_start(input: &mut [Opcode], expected: Opcode) {
        assert_eq!(compute(input, 0, input[1], input[2]).unwrap(), expected);
    }

    #[test]
    fn part1_aoc_with_error_state() {
        assert_eq!(compute(&mut inputs().unwrap(), 0, 12, 2).unwrap(), 3895705);
    }

    #[test]
    fn part2_aoc_brute_force() {
        assert_eq!(part2(&mut inputs().unwrap()), 6417);
    }
}
