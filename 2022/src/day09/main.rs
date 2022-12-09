#![feature(test)]
extern crate test;

use std::collections::HashSet;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 09");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

// Flip bits for each entry that a tail visits while following the head
fn solve_1(input: &str) -> String {
    let (path, _) = input.lines().fold(
        (HashSet::new(), [(0_isize, 0_isize); 2]),
        |(mut path, mut pos), line| {
            let bytes = line.as_bytes();
            let (step_x, step_y) = match bytes[0] {
                b'R' => (1, 0),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                _u => (0, 1),
            };
            let steps = match bytes.len() {
                3 => bytes[2] - b'0',
                _ => (bytes[2] - b'0') * 10 + bytes[3] - b'0',
            };

            (0..steps).into_iter().for_each(|_| {
                pos[0] = (pos[0].0 + step_x, pos[0].1 + step_y);
                (1..2).into_iter().for_each(|i| {
                    match (pos[i - 1].0 - pos[i].0, pos[i - 1].1 - pos[i].1) {
                        (x, y) if x.abs() > 1 || y.abs() > 1 => {
                            pos[i] = (pos[i].0 + x.signum(), pos[i].1 + y.signum());
                        }
                        _ => {}
                    }
                });

                path.insert(pos[1]);
            });

            (path, pos)
        },
    );

    path.len().to_string()
}

fn solve_2(input: &str) -> String {
    let (path, _) = input.lines().fold(
        (HashSet::new(), [(0_isize, 0_isize); 10]),
        |(mut path, mut pos), line| {
            let bytes = line.as_bytes();
            let (step_x, step_y) = match bytes[0] {
                b'R' => (1, 0),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                _u => (0, 1),
            };
            let steps = match bytes.len() {
                3 => bytes[2] - b'0',
                _ => (bytes[2] - b'0') * 10 + bytes[3] - b'0',
            };

            (0..steps).into_iter().for_each(|_| {
                pos[0] = (pos[0].0 + step_x, pos[0].1 + step_y);
                (1..10).into_iter().for_each(|i| {
                    match (pos[i - 1].0 - pos[i].0, pos[i - 1].1 - pos[i].1) {
                        (x, y) if x.abs() > 1 || y.abs() > 1 => {
                            pos[i] = (pos[i].0 + x.signum(), pos[i].1 + y.signum());
                        }
                        _ => {}
                    }
                });

                path.insert(pos[9]);
            });

            (path, pos)
        },
    );

    path.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_9() {
        assert_eq!(solve_1(EXAMPLE), "88");
        assert_eq!(solve_1(DATA), "6175");
        assert_eq!(solve_2(EXAMPLE), "36");
        assert_eq!(solve_2(DATA), "2578");
    }

    #[bench]
    fn bench_9_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_9_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
