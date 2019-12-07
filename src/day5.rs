use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Result<Vec<u32>> {
    input
        .split(',')
        .map(|v| v.parse().with_context(|| "Not a valid number."))
        .collect::<Result<Vec<_>>>()
}

#[aoc(day5, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

//#[aoc(day4, part2)]
//fn part2(input: &[Num]) -> Output {
//    input.iter().fold(0, |acc, x| {
//        if acc == 0 {
//            return *x;
//        } else {
//            acc * x
//        }
//    })
//}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized as pm;

    ide!();

    #[pm(input = {
        &[1, 2, 3],
    }, expected = {
        6,
    })]
    fn part1_test(input: &[u32], expected: u32) {
        assert_eq!(part1(input), expected);
    }

    //    #[pm(input = {
    //        &[1, 2, 4],
    //    }, expected = {
    //        0,
    //    })]
    //    fn part2_test(input: &[Num], expected: Output) {
    //        assert_eq!(part2(input), expected);
    //    }
}
