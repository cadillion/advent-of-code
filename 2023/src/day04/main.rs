#![feature(test)]
#![allow(dead_code)]
extern crate test;
use std::collections::HashMap;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 04");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, l| {
            let (_, body) = l.split_once(": ").unwrap();
            let (win_, has_) = body.split_once(" | ").unwrap();
            let (win, has): (Vec<_>, Vec<_>) = (
                win_.split(' ').filter(|h| !h.trim().is_empty()).collect(),
                has_.split(' ').filter(|h| !h.trim().is_empty()).collect(),
            );
            let mut score = 0;
            for w in win {
                if has.iter().any(|h| *h == w) {
                    score += 1;
                }
            }
            if score > 0 {
                acc + 2_usize.pow(score - 1)
            } else {
                acc
            }
        })
        .to_string()
}

fn solve_2(input: &str) -> String {
    let mut copies: HashMap<_, usize> = HashMap::new();

    input
        .lines()
        .enumerate()
        .fold(0, |acc, (n, l)| {
            let runs = *copies.get(&n).unwrap_or(&0) + 1;

            let (_, body) = l.split_once(": ").unwrap();
            let (win_, has_) = body.split_once(" | ").unwrap();
            let (win, has): (Vec<_>, Vec<_>) = (
                win_.split(' ').filter(|h| !h.trim().is_empty()).collect(),
                has_.split(' ').filter(|h| !h.trim().is_empty()).collect(),
            );
            let mut score = 0;
            for w in win {
                if has.iter().any(|h| *h == w) {
                    score += 1;
                }
            }

            for i in n..n + score {
                copies
                    .entry(i + 1)
                    .and_modify(|j| *j += runs)
                    .or_insert(runs);
            }

            acc + runs
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_4() {
        assert_eq!(solve_1(EXAMPLE), "13");
        assert_eq!(solve_1(DATA), "28750");
        assert_eq!(solve_2(EXAMPLE), "30");
        assert_eq!(solve_2(DATA), "10212704");
    }

    #[bench]
    fn bench_04_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_04_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
