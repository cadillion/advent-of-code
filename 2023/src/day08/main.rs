#![feature(test)]
#![allow(dead_code)]
extern crate test;
use std::collections::HashMap;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const EXAMPLE_2: &str = include_str!("example_2.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 08");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    let (lr, tail) = input.split_once("\n\n").unwrap();
    let mut cur = b"AAA";

    let map: HashMap<&[u8; 3], (&[u8; 3], &[u8; 3])> = tail
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            let (pos, l, r) = (&b[0..3], &b[7..10], &b[12..15]);
            (
                pos.try_into().unwrap(),
                (l.try_into().unwrap(), r.try_into().unwrap()),
            )
        })
        .collect();

    for (steps, b) in lr.bytes().cycle().enumerate() {
        if cur == b"ZZZ" {
            return steps.to_string();
        };
        cur = match b {
            b'L' => map[cur].0,
            b'R' => map[cur].1,
            _ => unreachable!(),
        };
    }
    unreachable!()
}

fn solve_2(input: &str) -> String {
    let (lr, tail) = input.split_once("\n\n").unwrap();
    let map: HashMap<&[u8; 3], (&[u8; 3], &[u8; 3])> = tail
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            let (pos, l, r) = (&b[0..3], &b[7..10], &b[12..15]);
            (
                pos.try_into().unwrap(),
                (l.try_into().unwrap(), r.try_into().unwrap()),
            )
        })
        .collect();

    map.keys()
        .copied()
        .filter(|p| p[2] == b'A')
        .map(|start| {
            let mut cur = start;
            let mut path = HashMap::<&[u8; 3], usize>::new();
            for (steps, b) in lr.bytes().cycle().enumerate() {
                if path.contains_key(cur) {
                    return steps - path[cur];
                }
                path.insert(cur, steps);

                cur = match b {
                    b'L' => map[cur].0,
                    b'R' => map[cur].1,
                    _ => unreachable!(),
                };
            }
            unreachable!()
        })
        .fold(lr.len(), |a, p| lcm(p, a))
        .to_string()
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let n = b;
        b = a % b;
        a = n;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_08() {
        assert_eq!(solve_1(EXAMPLE), "6");
        assert_eq!(solve_1(DATA), "13207");
        assert_eq!(solve_2(EXAMPLE_2), "6");
        assert_eq!(solve_2(DATA), "12324145107121");
    }

    #[bench]
    fn bench_08_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_08_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
