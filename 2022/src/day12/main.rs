#![feature(test)]
extern crate test;

use pathfinding::directed::bfs::bfs;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 12");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    let map = input.as_bytes();
    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let peak = input.bytes().position(|b| b == b'E').unwrap();
    let wrap = width + 1;

    let successors = |&p: &usize| {
        let pos = if map[p] == b'E' { b'z' } else { map[p] };
        [p + 1, p + wrap, p.wrapping_sub(1), p.wrapping_sub(wrap)]
            .into_iter()
            .filter(|&coord| coord < map.len())
            .filter(move |&coord| {
                let next = if map[coord] == b'S' { b'a' } else { map[coord] };
                pos != b'\n' && pos <= next + 1
            })
    };
    let success = |p: &usize| map[*p] == b'S';

    (bfs(&peak, successors, success).unwrap().len() - 1).to_string()
}

fn solve_2(input: &str) -> String {
    let map = input.as_bytes();
    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let peak = input.bytes().position(|b| b == b'E').unwrap();
    let wrap = width + 1;

    let successors = |&p: &usize| {
        let pos = if map[p] == b'E' { b'z' } else { map[p] };
        [p + 1, p + wrap, p.wrapping_sub(1), p.wrapping_sub(wrap)]
            .into_iter()
            .filter(|&coord| coord < map.len())
            .filter(move |&coord| {
                let next = if map[coord] == b'S' { b'a' } else { map[coord] };
                pos != b'\n' && pos <= next + 1
            })
    };
    let success = |p: &usize| map[*p] == b'a' || map[*p] == b'S';

    (bfs(&peak, successors, success).unwrap().len() - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_12() {
        assert_eq!(solve_1(EXAMPLE), "31");
        assert_eq!(solve_1(DATA), "440");
        assert_eq!(solve_2(EXAMPLE), "29");
        assert_eq!(solve_2(DATA), "439");
    }

    #[bench]
    fn bench_12_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_12_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
