#[cfg(test)]
#[macro_use]
extern crate parameterized;

use aoc_runner_derive::aoc_lib;

// Attribute macro's such as 'parameterized' do not enable the run tests Intent for a module
// marked as cfg(test) (or a #[test] function for that matter) in Intellij.
//
// To enable the intent within a module, we need at least a single test marked with `#[test]`.
// Thus, we will need to create an empty test within every module where we wish to run the tests
// using this intent.
//
// Instead of making such a test every time, we can use the macro below.
// Using the intellij-rust new macro expansion engine, the module will be detected as containing
// tests.
#[cfg(test)]
macro_rules! ide {
    () => {
        #[test]
        fn __intellij_module_test_intent() {}
    };
}

mod day1;
mod day2;
//mod refine; //day3;

mod day4;
mod day5;

// the Intcode vm
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
