#![feature(test)]
#![allow(dead_code)]
extern crate test;
use std::cmp::max;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 02");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Get the sum of all game IDs where the sum of each color is less than 12 red, 13 green, 14 blue
fn solve_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|l| {
            let (id, draws) = l.split_at(5).1.split_once(": ").unwrap();
            match draws.split("; ").all(|draw| {
                let (r, g, b) = draw.split(", ").fold((0, 0, 0), |(r, g, b), c| {
                    let (count, color) = c.split_once(' ').unwrap();
                    let count = count.parse::<usize>().unwrap();
                    match color {
                        "red" => (r + count, g, b),
                        "green" => (r, g + count, b),
                        "blue" => (r, g, b + count),
                        _ => unreachable!(),
                    }
                });
                r < 13 && g < 14 && b < 15
            }) {
                true => id.parse::<usize>().ok(),
                false => None,
            }
        })
        .sum::<usize>()
        .to_string()
}

/// Get the products of the minimum number of cubes required for each game and sum them
fn solve_2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let (r, g, b) =
                l.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .fold((0, 0, 0), |(r1, g1, b1), draw| {
                        let (r2, g2, b2) = draw.split(", ").fold((0, 0, 0), |(r2, g2, b2), c| {
                            let (count, color) = c.split_once(' ').unwrap();
                            let count = count.parse::<usize>().unwrap();
                            match color {
                                "red" => (r2 + count, g2, b2),
                                "green" => (r2, g2 + count, b2),
                                "blue" => (r2, g2, b2 + count),
                                _ => unreachable!(),
                            }
                        });
                        (max(r1, r2), max(g1, g2), max(b1, b2))
                    });
            r * g * b
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_2() {
        assert_eq!(solve_1(EXAMPLE), "8");
        assert_eq!(solve_1(DATA), "2617");
        assert_eq!(solve_2(EXAMPLE), "2286");
        assert_eq!(solve_2(DATA), "59795");
    }

    #[bench]
    fn bench_02_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_02_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
