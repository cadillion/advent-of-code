#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day XX");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    "".to_owned()
}

fn solve_2(input: &str) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_X() {
        assert_eq!(solve_1(EXAMPLE), "");
        assert_eq!(solve_1(DATA), "");
        assert_eq!(solve_2(EXAMPLE), "");
        assert_eq!(solve_2(DATA), "");
    }

    #[bench]
    fn bench_X_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_X_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
