#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 04");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Find the pairs that have one pair which contains the other, inclusive
fn solve_1(input: &str) -> String {
    let nested_pairs = input
        .lines()
        .map(|line| {
            let mut bound = line.split(&['-', ',']).map(|i| i.parse::<u16>().unwrap());
            [
                bound.next().unwrap(),
                bound.next().unwrap(),
                bound.next().unwrap(),
                bound.next().unwrap(),
            ]
        })
        .filter(|[start_1, end_1, start_2, end_2]| {
            (start_1 <= start_2 && end_2 <= end_1) || (start_2 <= start_1 && end_1 <= end_2)
        })
        .count();

    format!("{}", nested_pairs)
}

/// Find the pairs where the ranges share any members, inclusive
fn solve_2(input: &str) -> String {
    let overlapping_pairs = input
        .lines()
        .map(|line| {
            let mut bound = line.split(&['-', ',']).map(|i| i.parse::<u16>().unwrap());
            [
                bound.next().unwrap(),
                bound.next().unwrap(),
                bound.next().unwrap(),
                bound.next().unwrap(),
            ]
        })
        .filter(|[start_1, end_1, start_2, end_2]| start_1 <= end_2 && start_2 <= end_1)
        .count();

    format!("{}", overlapping_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_4() {
        assert_eq!(solve_1(EXAMPLE), "2");
        assert_eq!(solve_1(DATA), "453");
        assert_eq!(solve_2(EXAMPLE), "4");
        assert_eq!(solve_2(DATA), "919");
    }

    #[bench]
    fn bench_4_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_4_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
