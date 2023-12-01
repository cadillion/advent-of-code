#![feature(test)]
#![allow(dead_code)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

const HEIGHT: usize = 173;
const XMIN: usize = 0;
const XMAX: usize = 1000;
const WIDTH: usize = XMAX - XMIN + 1;
const ORIGIN: usize = 500 - XMIN;

fn main() {
    println!("Day 14");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn fill(map: &mut [u8; HEIGHT * WIDTH], x: usize, y: usize) -> Option<usize> {
    if map[ORIGIN] == 1 {
        return None;
    }

    // For each y in the map, check if there's rock or sand in the way
    for h in y..(HEIGHT - 1) {
        // Get the current position at height y
        let pos = h * WIDTH + x;
        // If the current position is a rock or sand check the rules
        match map[pos + WIDTH] {
            1 => {
                return match x {
                    // If we're against the left or right edge, break
                    0 | WIDTH => None,
                    // If the item to the left is empty, move to the left
                    _one => match map[pos + WIDTH - 1] {
                        0 => fill(map, x - 1, h),
                        _one => match map[pos + WIDTH + 1] {
                            0 => fill(map, x + 1, h),
                            _one => {
                                map[pos] = 1;
                                Some(pos)
                            }
                        },
                    },
                };
            }
            _0 => continue,
        }
    }
    // If we touch the bottom of the map, exit
    None
}

fn print_map(map: [u8; HEIGHT * WIDTH]) {
    println!(
        "\n   {}",
        (0..WIDTH)
            .step_by(5)
            .map(|c| format!("{c:>width$}", width = 5))
            .collect::<String>()
    );
    map.chunks(WIDTH).enumerate().for_each(|(i, b)| {
        println!(
            "{i:width$} | {}",
            b.into_iter()
                .map(|c| (*c as u8 + b'0') as char)
                .collect::<String>(),
            width = 4,
        )
    });
}

fn solve_1(input: &str) -> String {
    // draw rocks
    let mut map = input
        .lines()
        .fold([0_u8; HEIGHT * WIDTH], |acc, line: &str| {
            let new = line
                .split(" -> ")
                .map(|coord| coord.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap() - XMIN, y.parse().unwrap()))
                .collect::<Vec<(usize, usize)>>()
                .windows(2)
                .fold(acc, |mut ac, coord| {
                    let &[(x1, y1), (x2, y2)] = coord else {unreachable!()};
                    for x in x1..=x2 {
                        ac[y1 * WIDTH + x] = 1;
                    }
                    for x in x2..=x1 {
                        ac[y1 * WIDTH + x] = 1;
                    }
                    for y in y1..=y2 {
                        ac[y * WIDTH + x1] = 1;
                    }
                    for y in y2..=y1 {
                        ac[y * WIDTH + x1] = 1;
                    }
                    ac
                });
            new
        });

    let mut counter: usize = 0;
    while let Some(_) = fill(&mut map, ORIGIN, 0) {
        counter += 1;
    }
    counter.to_string()
}

fn solve_2(input: &str) -> String {
    let mut map = input
        .lines()
        .fold([0_u8; HEIGHT * WIDTH], |acc, line: &str| {
            let new = line
                .split(" -> ")
                .map(|coord| coord.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap() - XMIN, y.parse().unwrap()))
                .collect::<Vec<(usize, usize)>>()
                .windows(2)
                .fold(acc, |mut ac, coord| {
                    let &[(x1, y1), (x2, y2)] = coord else {unreachable!()};
                    for x in x1..=x2 {
                        ac[y1 * WIDTH + x] = 1;
                    }
                    for x in x2..=x1 {
                        ac[y1 * WIDTH + x] = 1;
                    }
                    for y in y1..=y2 {
                        ac[y * WIDTH + x1] = 1;
                    }
                    for y in y2..=y1 {
                        ac[y * WIDTH + x1] = 1;
                    }
                    ac
                });
            new
        });

    let max_y = (map.len() - map.iter().rev().position(|v| *v == 1).unwrap()) / WIDTH + 2;
    for x in 0..WIDTH {
        map[max_y * WIDTH + x] = 1;
    }

    let mut counter: usize = 0;
    while let Some(_) = fill(&mut map, ORIGIN, 0) {
        counter += 1;
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_14() {
        assert_eq!(solve_1(EXAMPLE), "24");
        assert_eq!(solve_1(DATA), "843");
        assert_eq!(solve_2(EXAMPLE), "93");
        assert_eq!(solve_2(DATA), "");
    }

    #[bench]
    fn bench_14_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_14_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
