#![feature(test)]
#![allow(dead_code)]
extern crate test;
use std::collections::HashMap;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 3");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    let schema = input.as_bytes();
    let mut sum = 0;
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let width = first.len() + 1;
    let height = schema.len() / first.len();

    for (i, l) in std::iter::once(first).chain(lines).enumerate() {
        let mut x = 0;
        let y = i * width;
        let (y1, y2) = match i {
            0 => (y, y + width),
            i if i == height - 1 => (y - width, y),
            _i => (y - width, y + width),
        };

        while x < width - 1 {
            let j = match l.bytes().skip(x).position(|b| b.is_ascii_digit()) {
                Some(j) => x + j,
                None => break,
            };
            let x1 = if j == 0 { 0 } else { j - 1 };
            let (k, x2) = match l.bytes().skip(j).position(|b| !b.is_ascii_digit()) {
                Some(k) => (j + k, j + k + 1),
                None => (width - 1, width - 1),
            };

            if !schema[y1 + x1..y1 + x2]
                .iter()
                .chain(std::iter::once(&schema[y + x1]))
                .chain(std::iter::once(&schema[y + x2 - 1]))
                .chain(schema[y2 + x1..y2 + x2].iter())
                .all(|c| c.is_ascii_digit() || *c == b'.')
            {
                sum += String::from_utf8(schema[y + j..y + k].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            };

            x = x2;
        }
    }

    sum.to_string()
}

fn solve_2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let (w, h) = (lines[0].len(), lines.len());

    let mut gears: HashMap<_, Vec<_>> = HashMap::new();
    for (y, l) in lines.iter().enumerate() {
        let mut x = 0;
        while x < w {
            let end = l[x..].find(|c: char| !c.is_ascii_digit()).unwrap_or(w - x);
            if end > 0 {
                let n = l[x..x + end].parse::<i64>().unwrap();

                let mut _valid = false;
                for ny in y as i64 - 1..=y as i64 + 1 {
                    for nx in x as i64 - 1..=(x + end) as i64 {
                        if 0 <= ny && ny < h as i64 && 0 <= nx && nx < w as i64 {
                            let b = lines[ny as usize].as_bytes()[nx as usize];
                            if b == b'*' {
                                gears.entry((nx, ny)).or_default().push(n);
                            }
                            _valid |= b != b'.' && !b.is_ascii_digit();
                        }
                    }
                }
            }

            x += end + 1;
        }
    }

    let mut part2 = 0;
    for val in gears.values() {
        if let &[a, b] = val.as_slice() {
            part2 += a * b;
        }
    }
    part2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_3() {
        assert_eq!(solve_1(EXAMPLE), "4361");
        assert_eq!(solve_1(DATA), "537832");
        assert_eq!(solve_2(EXAMPLE), "467835");
        assert_eq!(solve_2(DATA), "81939900");
    }

    #[bench]
    fn bench_03_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_03_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
