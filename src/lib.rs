#[cfg(test)]
#[macro_use]
extern crate parameterized;

use aoc_runner_derive::aoc_lib;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

// the intcode vm
mod vm;

aoc_lib! { year = 2019 }

#[cfg(test)]
fn setup<F, V>(day: u8, parse: F) -> anyhow::Result<V>
where
    F: Fn(&str) -> anyhow::Result<V>,
{
    use anyhow::Context;

    let input = std::fs::read_to_string(format!("input/2019/day{}.txt", day))
        .map(|v| v.trim_end().to_string())
        .context("Unable to open & read file.")?;

    parse(&input).context("Unable to parse input")
}
