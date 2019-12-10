use anyhow::anyhow as anyhowdy; // intellij-rust doesn't like anyhow::anyhow ....
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

// day4, if only we were doing prolog ...

// dayum, hacky
#[derive(Debug)]
struct AdjNum {
    start: usize, //index
    end: usize,   //index, assume that start < end
}

impl AdjNum {
    fn just_two(&self) -> bool {
        assert!(self.start < self.end);

        self.start + 1 == self.end
    }
}

#[derive(Debug)]
struct Code {
    tokens: [char; 6],
}

impl Code {
    fn try_from_str(input: &str) -> Result<Self> {
        let mut chars = input.chars();

        const NO_SUCH_ELEMENT: &str = "no such element";

        Ok(Self {
            tokens: [
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
                chars.next().ok_or_else(|| anyhowdy!(NO_SUCH_ELEMENT))?,
            ],
        })
    }

    // with allocation:
    // `Self::try_from_str(&number.to_string())`
    fn from_number(number: u32) -> Result<Self> {
        let mut buffer = [0u8; 6];
        let n = itoa::write(&mut buffer[..], number).context("failed to parse u32")?;

        let text = unsafe { std::str::from_utf8_unchecked(&buffer[..n]) };

        Self::try_from_str(text)
    }

    fn good_code(number: u32) -> Option<Self> {
        Self::from_number(number)
            .ok()
            .filter(|v| v.adjacency_criteria() && v.increasing_criteria())
    }

    fn adjacency_criteria(&self) -> bool {
        self.tokens[0] == self.tokens[1]
            || self.tokens[1] == self.tokens[2]
            || self.tokens[2] == self.tokens[3]
            || self.tokens[3] == self.tokens[4]
            || self.tokens[4] == self.tokens[5]
    }

    fn increasing_criteria(&self) -> bool {
        self.tokens[0] <= self.tokens[1]
            && self.tokens[1] <= self.tokens[2]
            && self.tokens[2] <= self.tokens[3]
            && self.tokens[3] <= self.tokens[4]
            && self.tokens[4] <= self.tokens[5]
    }

    // part 2
    fn good_code_for_forgetful_elf(number: u32) -> Option<Self> {
        Self::from_number(number)
            .ok()
            .filter(|v| v.increasing_criteria())
            .filter(|v| {
                let vec = v.adjacent_characters();

                vec.iter().filter(|twos| twos.just_two()).count() > 0
            })
    }

    // these are the characters for which there are adj. numbers
    fn adjacent_characters(&self) -> Vec<AdjNum> {
        (b'0'..=b'9')
            .map(char::from)
            .filter_map(|chr| self.adj(chr).map(|(start, end)| AdjNum { start, end }))
            .collect::<Vec<_>>()
    }

    // -> Some(index, len)
    fn adj(&self, num: char) -> Option<(usize, usize)> {
        // we want to know the starting index
        for i in 0..=4 {
            // there need to be two adjacent of the number
            if self.tokens[i] == num && self.tokens[i + 1] == num {
                // we want to know the ending index
                let mut end = i;
                for same_num in &self.tokens[i + 1..=5] {
                    if *same_num != num {
                        break;
                    }
                    end += 1;
                }

                return Some((i, end)); //Some(i);
            }
        }

        None
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Result<(u32, u32)> {
    let mut iter = input.split('-').flat_map(|v| v.parse());

    let x = iter.next().context("No value 1")?;
    let y = iter.next().context("No value 2")?;

    Ok((x, y))
}

/// There are six constraints:
/// - six digit number
/// - value in range of puzzle input
/// - two adjacent numbers are the same
/// - going left to right, digits never decrease
///
/// We stay within puzzle input range, so we'll never break the first or second rules.
/// Then we find the numbers within this range which have adjacent numbers; this leaves us
/// with a minimal amount of options.
///
/// For these options, we'll brute force check what numbers meet this requirement.
///
/// If our computing speed does become an issue we'll look into skipping numbers which can never
/// be valid (after a previous validation).
///
#[aoc(day4, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)] // this is how we receive the input from cargo-aoc
fn part1(input: &(u32, u32)) -> usize {
    (input.0..=input.1).filter_map(Code::good_code).count()
}

#[aoc(day4, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)] // this is how we receive the input from cargo-aoc
fn part2(input: &(u32, u32)) -> usize {
    (input.0..=input.1)
        .filter_map(Code::good_code_for_forgetful_elf)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1 (extra)
    // ==============

    #[test]
    fn adj() {
        let code = Code {
            tokens: ['1', '1', '1', '1', '1', '1'],
        };

        assert!(code.adjacency_criteria())
    }

    #[test]
    fn adj_none() {
        let code = Code {
            tokens: ['1', '2', '3', '4', '5', '1'],
        };

        assert!(!code.adjacency_criteria())
    }

    #[test]
    fn incr() {
        let code = Code {
            tokens: ['1', '1', '1', '1', '1', '1'],
        };

        assert!(code.increasing_criteria())
    }

    #[test]
    fn incr_fail() {
        let code = Code {
            tokens: ['1', '1', '1', '1', '1', '0'],
        };

        assert!(!code.increasing_criteria())
    }
}
