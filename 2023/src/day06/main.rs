#![feature(test)]
#![allow(dead_code)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 06");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn find(t: f64, d: f64) -> usize {
    let r = (t.powi(2) - d * 4.).sqrt();
    (((t + r) / 2.).ceil() - ((t - r) / 2.).floor() - 1.) as usize
}

/// Discover what combinations of n * (m - n) are greater than distance d
fn solve_1(input: &str) -> String {
    let (t, d) = input.split_once('\n').unwrap();
    t.split_ascii_whitespace()
        .skip(1)
        .zip(d.split_ascii_whitespace().skip(1))
        .map(|(t, d)| find(t.parse().unwrap(), d.parse().unwrap()))
        .product::<usize>()
        .to_string()
}

fn solve_2(input: &str) -> String {
    let mut td = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<f64>()
            .unwrap()
    });

    find(td.next().unwrap(), td.next().unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_6() {
        assert_eq!(solve_1(EXAMPLE), "288");
        assert_eq!(solve_1(DATA), "440000");
        assert_eq!(solve_2(EXAMPLE), "71503");
        assert_eq!(solve_2(DATA), "26187338");
    }

    #[bench]
    fn bench_06_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_06_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
