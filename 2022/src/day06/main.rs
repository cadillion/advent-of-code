#![feature(test)]
extern crate test;

use std::ops::ControlFlow;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    println!("Day 06");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

// Check that all the items in the window are unique with a bit shift
// (eg, 1 << 3 = 1000; 1 << 2 = 100; (1 << 3) & (1 << 1) = 1010)
// Break out of the iterator on the first duplicate, otherwise flip any bits we've already seen and continue
// (ie, 0101 & 0110 = 0100 shares the third bit, and is greater than 0, so we break out of the iterator)
fn all_unique(code: &[u8]) -> ControlFlow<usize, usize> {
    code.into_iter().try_fold(0, |bin, cha| {
        let shift = cha - b'a';
        match bin & (1 << shift) {
            0 => ControlFlow::Continue(bin | (1 << shift)),
            b => ControlFlow::Break(b),
        }
    })
}

// Break the input into windows and iterate through each of the windows searching for a unique set
fn window(input: &[u8], size: usize) -> usize {
    match input
        .windows(size)
        .try_fold(size, |acc, cur| match all_unique(cur) {
            ControlFlow::Continue(_) => ControlFlow::Break(acc),
            _ => ControlFlow::Continue(acc + 1),
        }) {
        ControlFlow::Break(index) => index,
        _ => 0,
    }
}

// Find a 4 character start code with all unique characters
fn solve_1(input: &[u8]) -> String {
    window(input, 4).to_string()
}

// Find a 14 character start code with all unique characters
fn solve_2(input: &[u8]) -> String {
    window(input, 14).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_6() {
        assert_eq!(solve_1(EXAMPLE), "7");
        assert_eq!(solve_1(DATA), "1093");
        assert_eq!(solve_2(EXAMPLE), "19");
        assert_eq!(solve_2(DATA), "3534");
    }

    #[bench]
    fn bench_6_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_6_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
