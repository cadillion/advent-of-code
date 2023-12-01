#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 13");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(left), Packet::Int(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => left.iter().cmp(right),
            (Packet::Int(left), right) => Packet::List(vec![Packet::Int(*left)]).cmp(right),
            (left, Packet::Int(right)) => left.cmp(&Packet::List(vec![Packet::Int(*right)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.bytes().next() {
            Some(b'[') => Ok(Packet::List(
                list(s)
                    .map(|b| Packet::from_str(std::str::from_utf8(&b[..]).unwrap()).unwrap())
                    .collect(),
            )),
            Some(_) => Ok(Packet::Int(s.parse().unwrap())),
            None => Err("Attempted to parse empty string".into()),
        }
    }
}

fn list(string: &str) -> impl Iterator<Item = Vec<u8>> {
    string
        .bytes()
        .fold(
            (Vec::new(), Vec::new(), 0),
            |(mut nums, mut buff, depth), byte| match (depth, byte) {
                (0, b'[') => (nums, buff, depth + 1),
                (1, b']') => {
                    if buff.len() != 0 {
                        nums.push(buff.drain(..).collect());
                    }
                    (nums, buff, depth)
                }
                (1, b',') => {
                    nums.push(buff.drain(..).collect());
                    (nums, buff, depth)
                }
                (_, b) => {
                    buff.push(b);
                    let new_depth = match b {
                        b'[' => depth + 1,
                        b']' => depth - 1,
                        _ => depth,
                    };
                    (nums, buff, new_depth)
                }
            },
        )
        .0
        .into_iter()
}

fn solve_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|pair| {
            let (l, r) = pair.split_once('\n').unwrap();
            (l.parse::<Packet>().unwrap(), r.parse::<Packet>().unwrap())
        })
        .enumerate()
        .filter_map(|(index, (left, right))| match left < right {
            true => Some(index + 1),
            false => None,
        })
        .sum::<usize>()
        .to_string()
}

fn solve_2(input: &str) -> String {
    let first = "[[2]]".parse::<Packet>().unwrap();
    let second = "[[6]]".parse::<Packet>().unwrap();
    let decoder = BinaryHeap::from([first.clone(), second.clone()]);

    let sorted = input
        .split("\n\n")
        .map(|pair| {
            let (l, r) = pair.split_once('\n').unwrap();
            (l.parse::<Packet>().unwrap(), r.parse::<Packet>().unwrap())
        })
        .fold(decoder, |mut acc, (left, right)| {
            acc.push(left);
            acc.push(right);
            acc
        })
        .into_sorted_vec();

    let d1 = sorted.binary_search(&first).unwrap() + 1;
    let d2 = sorted.binary_search(&second).unwrap() + 1;
    (d1 * d2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_13() {
        assert_eq!(solve_1(EXAMPLE), "13");
        assert_eq!(solve_1(DATA), "5806");
        assert_eq!(solve_2(EXAMPLE), "140");
        assert_eq!(solve_2(DATA), "23600");
    }

    #[bench]
    fn bench_13_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_13_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
