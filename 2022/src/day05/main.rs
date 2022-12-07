#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 05");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn build_stack(input: &str) -> (std::str::Lines<'_>, Vec<Vec<u8>>) {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut boxes = crates.lines().enumerate();

    let (_, first) = boxes.next().unwrap();
    let width: usize = first.len() / 3;

    let layout: Vec<Vec<u8>> = std::iter::once((0, first))
        .chain(boxes)
        .fold(vec![Vec::new(); width], |mut stacks, (_, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_uppercase())
                .for_each(|(x, c)| {
                    stacks[x / 4].push(c);
                });

            stacks
        })
        .into_iter()
        .map(|mut vec| {
            vec.reverse();
            vec
        })
        .collect();

    (instructions.lines(), layout)
}

fn extract_moves(line: &[u8]) -> (usize, usize, usize) {
    match line[6] {
        b' ' => ((line[5] - b'0').into(), 12_usize, 17_usize),
        _ => (
            ((line[5] - b'0') * 10 + line[6] - b'0').into(),
            13_usize,
            18_usize,
        ),
    }
}

fn pop_each_stack(layout: Vec<Vec<u8>>) -> String {
    layout
        .into_iter()
        .filter(|v| !v.is_empty())
        .fold(String::new(), |mut acc, mut stack| {
            acc.push(stack.pop().unwrap() as char);
            acc
        })
}

/// Parse the box representation into vectors, then follow the instructions:
/// Pop items n times from the indicated stack and push to the indicated stack
fn solve_1(input: &str) -> String {
    let (instructions, mut layout) = build_stack(input);
    let _movement = instructions.for_each(|l| {
        let line = l.as_bytes();
        let (moves, from_ind, to_ind) = extract_moves(line);

        for _ in 0..moves {
            let moved = layout[(line[from_ind] - b'1') as usize].pop().unwrap();
            layout[(line[to_ind] - b'1') as usize].push(moved);
        }
    });

    pop_each_stack(layout)
}

/// Parse the box representation into vectors, then follow the instructions:
/// Append the last n items from the indicated stack to the indicated stack
fn solve_2(input: &str) -> String {
    let (instructions, mut layout) = build_stack(input);
    let _movement = instructions.for_each(|l| {
        let line = l.as_bytes();
        let (moves, from_ind, to_ind) = extract_moves(line);

        let from: usize = (line[from_ind] - b'1').into();
        let from_size = layout[from].len();
        let moved: Vec<u8> = layout[from].drain((from_size - moves)..).collect();
        layout[(line[to_ind] - b'1') as usize].extend(moved);
    });

    pop_each_stack(layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_5() {
        assert_eq!(solve_1(EXAMPLE), "CMZ");
        assert_eq!(solve_1(DATA), "PTWLTDSJV");
        assert_eq!(solve_2(EXAMPLE), "MCD");
        assert_eq!(solve_2(DATA), "WZMFVGGZP");
    }

    #[bench]
    fn bench_5_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_5_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
