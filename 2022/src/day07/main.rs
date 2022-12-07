#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 07");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Find each directory with less than 100,000 bytes and sum them, even if they are nested in each other
fn solve_1(input: &str) -> String {
    let (acc, _stack) = input
        .lines()
        .fold((0, Vec::new()), |(acc, mut stack), line| {
            match line.as_bytes() {
                &[.., b'.', b'.'] => match stack.pop().expect("Popped empty array") {
                    x if x <= 100_000 => (acc + x, stack),
                    _ => (acc, stack),
                },
                &[b'$', b' ', b'c', ..] => {
                    stack.push(0);
                    (acc, stack)
                }
                &[b'0'..=b'9', ..] => {
                    let (s, _) = line.split_once(' ').unwrap();
                    let num: usize = s.parse().unwrap();
                    stack.iter_mut().for_each(|x| *x += num);
                    (acc, stack)
                }
                _ => (acc, stack),
            }
        });
    acc.to_string()
}

/// Find the smallest folder you can delete if the available disk space on the hard drive is 70000000 bytes
/// and the update requires 30000000 bytes free
fn solve_2(input: &str) -> String {
    const SPACE: usize = 70000000;
    const REQUIRED: usize = 30000000;

    let (mut all, stack) = input.lines().fold(
        (Vec::new(), Vec::new()),
        |(mut all, mut stack), line| match line.as_bytes() {
            &[.., b'.', b'.'] => {
                let size = stack.pop().expect("Popped empty array");
                all.push(size);
                (all, stack)
            }
            &[b'$', b' ', b'c', ..] => {
                stack.push(0);
                (all, stack)
            }
            &[b'0'..=b'9', ..] => {
                let (s, _) = line.split_once(' ').unwrap();
                let num: usize = s.parse().unwrap();
                stack.iter_mut().for_each(|x| *x += num);
                (all, stack)
            }
            _ => (all, stack),
        },
    );
    all.extend(stack);
    all.sort_unstable();
    let needed = REQUIRED - (SPACE - all.last().unwrap());
    let smallest = all.iter().find(|x| **x >= needed).unwrap();
    smallest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_7() {
        assert_eq!(solve_1(EXAMPLE), "95437");
        assert_eq!(solve_1(DATA), "1501149");
        assert_eq!(solve_2(EXAMPLE), "24933642");
        assert_eq!(solve_2(DATA), "10096985");
    }

    #[bench]
    fn bench_7_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_7_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
