#![feature(test)]
#![allow(dead_code)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 09");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |mut acc, l| {
            let nums = l
                .split(' ')
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let mut next = vec![nums];
            while !next[next.len() - 1].iter().all(|n| *n == 0) {
                next.push(
                    next[next.len() - 1]
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<_>>(),
                );
            }
            for n in next {
                acc += n[n.len() - 1];
            }
            acc
        })
        .to_string()
}

fn solve_2(input: &str) -> String {
    input
        .lines()
        .fold(0, |mut acc, l| {
            let nums = l
                .split(' ')
                .rev()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let mut next = vec![nums];
            while !next[next.len() - 1].iter().all(|n| *n == 0) {
                next.push(
                    next[next.len() - 1]
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<_>>(),
                );
            }
            for n in next {
                acc += n[n.len() - 1];
            }
            acc
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_09() {
        assert_eq!(solve_1(EXAMPLE), "114");
        assert_eq!(solve_1(DATA), "2005352194");
        assert_eq!(solve_2(EXAMPLE), "2");
        assert_eq!(solve_2(DATA), "1077");
    }

    #[bench]
    fn bench_09_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_09_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
