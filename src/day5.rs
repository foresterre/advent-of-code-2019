use crate::vm::{ExecutionOption, Word, VM};
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Result<Vec<Word>> {
    input
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .context("Unable to parse input.")
}

#[aoc(day5, part1)]
fn part1(input: &[Word]) -> Result<Word> {
    let mut program = input.to_vec(); // because we need mutability for the current solution; and the aoc generator doesn't support it

    let mut vm = VM::with_inputs(&mut program, vec![1]);
    vm.execute(ExecutionOption::OutputByTapeOutput)
}

#[aoc(day5, part2)]
fn part2(input: &[Word]) -> Result<Word> {
    let mut program = input.to_vec(); // because we need mutability for the current solution; and the aoc generator doesn't support it

    let mut vm = VM::with_inputs(&mut program, vec![5]);
    vm.execute(ExecutionOption::OutputByTapeOutput)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized as pm;

    ide!();

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::setup;

        ide!();

        fn _inputs() -> anyhow::Result<Vec<Word>> {
            setup(5, parse_input)
        }

        #[pm(
        program = {
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, - 1, 0, 1, 9],
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, - 1, 0, 1, 9],
            &mut [3, 3, 1105, - 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &mut [3, 3, 1105, - 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        },
        input = {
            1,
            0,
            1,
            0,
        },
        expected = {
            1,
            0,
            1,
            0,
        })]
        fn jump_if_true_mirror(program: &mut [Word], input: Word, expected: Word) {
            let mut vm = VM::with_inputs(program, vec![input]);
            assert_eq!(
                vm.execute(ExecutionOption::OutputByTapeOutput).unwrap(),
                expected
            );
        }
    }
}
