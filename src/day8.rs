use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use image::{GrayAlphaImage, LumaA};
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::path::PathBuf;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;
const LAYER_PIXELS: usize = (WIDTH * HEIGHT) as usize;

#[aoc_generator(day8)] // ugh
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part1)] // Can't accept custom lifetime parameter ...
fn part1(input: &str) -> Result<u32> {
    input
        .as_bytes()
        .chunks(LAYER_PIXELS)
        .map(|v| v.iter().fold(HashMap::new(), count_chars))
        .min_by_key(|left| *left.get(&b'0').unwrap_or(&0))
        .map(|map| {
            let ones = *map.get(&b'1').unwrap_or(&0);
            let twos = *map.get(&b'2').unwrap_or(&0);

            ones * twos
        })
        .context("Unable to decode Special Image Format message.")
}

#[allow(clippy::trivially_copy_pass_by_ref)] //
fn count_chars(mut map: HashMap<u8, u32>, key: &u8) -> HashMap<u8, u32> {
    *map.entry(*key).or_insert(0) += 1;
    map
}

#[aoc(day8, part2)]
fn part2(input: &str) -> Result<PostOutputLocation<PathBuf>> {
    let canvas = input
        .as_bytes()
        .chunks(LAYER_PIXELS)
        .map(|layer| layer.iter().map(|px| Px::new(*px)).collect())
        .rfold(vec![Px::Transparent; LAYER_PIXELS], |acc, x| {
            paint_on_top(&acc, x)
        });

    let painting = canvas.iter().enumerate().fold(
        GrayAlphaImage::new(WIDTH, HEIGHT),
        |mut buffer, (n, pixel)| {
            let (width, height) = nth(n as u32);
            buffer.put_pixel(width, height, pixel.color());
            buffer
        },
    );

    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("day8b.png");

    painting
        .save(&folder)
        .with_context(|| "Unable to save image")?;

    Ok(PostOutputLocation::new(folder))
}

struct PostOutputLocation<P: AsRef<std::path::Path>>(P);
impl<P: AsRef<std::path::Path>> Display for PostOutputLocation<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&format!("View result at: {}", self.0.as_ref().display()))
    }
}

impl<P: AsRef<std::path::Path>> PostOutputLocation<P> {
    fn new(path: P) -> Self {
        Self(path)
    }
}

#[derive(Debug, Copy, Clone)]
enum Px {
    Black,
    White,
    Transparent,
}

impl Px {
    fn new(c: u8) -> Self {
        match c {
            b'0' => Self::Black,
            b'1' => Self::White,
            b'2' => Self::Transparent,
            _ => panic!("Digital Sending Network: received malformed input."),
        }
    }

    fn color(self) -> LumaA<u8> {
        match self {
            Self::Black => LumaA([0, 255]),
            Self::White => LumaA([255, 255]),
            Self::Transparent => LumaA([0, 0]),
        }
    }

    // If painting a transparent pixel on top of a coloured pixel, the pixel stays the original color.
    fn paint_visible(self, px: Px) -> Self {
        match (self, px) {
            (_, Px::Transparent) => self,
            _ => px,
        }
    }
}

fn paint_on_top(canvas: &[Px], paint: Vec<Px>) -> Vec<Px> {
    canvas
        .iter()
        .zip(paint)
        .map(|(b, t)| b.paint_visible(t))
        .collect()
}

fn nth(n: u32) -> (u32, u32) {
    let row = n / WIDTH;
    let col = n - (WIDTH * row);

    (col, row)
}
