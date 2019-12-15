use crate::vm::{ExecutionOption, Word, VM};
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Result<Vec<Word>> {
    input
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .context("Unable to parse input.")
}

fn run_vm_with(program: &[Word], phase_settings: i32, input_signal: i32) -> Result<Word> {
    let mut memory = program.to_vec();
    let mut vm = VM::with_inputs(&mut memory, vec![phase_settings, input_signal]);
    vm.execute(ExecutionOption::OutputByTapeOutput)
        .context("Invalid calculation")
}

#[aoc(day7, part1)]
fn part1(program: &[Word]) -> Result<Word> {
    // hihihi :P
    (0..=4)
        .permutations(5)
        .flat_map(|amplifier_controls| {
            run_vm_with(program, amplifier_controls[0], 0).and_then(|out| {
                run_vm_with(program, amplifier_controls[1], out).and_then(|out| {
                    run_vm_with(program, amplifier_controls[2], out).and_then(|out| {
                        run_vm_with(program, amplifier_controls[3], out)
                            .and_then(|out| run_vm_with(program, amplifier_controls[4], out))
                    })
                })
            })
        })
        .max()
        .context("Unable to compute the maximum amplifier output.")
}
