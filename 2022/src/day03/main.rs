#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 3");
    let r1 = solve_1(DATA);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Each line has one character repeated in the first and second half,
/// Find the sum of the values if lowercase are 1-26 and uppercase are 27-52
fn solve_1(input: &str) -> String {
    let priority = input.lines().fold(0, |sum, line| {
        let bytes = line.as_bytes();
        // Split the slice of bytes into two slices
        let (first, second) = bytes.split_at(bytes.len() / 2);
        // Find the first item in the first list that also exists in the second list
        let item = first.iter().find(|b| second.contains(b)).unwrap();
        let priority = match item {
            byte @ b'a'..=b'z' => byte - b'a' + 1, // If lowercase letter, assign priority between 1 and 26
            byte @ b'A'..=b'Z' => byte - b'A' + 27, // If capital letter, assign priority between 27 and 52
            _ => 0,
        };

        sum + priority as usize
    });

    format!("{}", priority)
}

/// Every 3 lines shares one character, find the sum of values as indicated in part one
fn solve_2(input: &str) -> String {
    let (priority, _) = input
        .lines()
        .enumerate()
        // Initialize a state machine to aggregate lines before comparing them
        .fold((0, ["", ""]), |(sum, mut group), (index, line)| {
            match index % 3 {
                i @ 0..=1 => {
                    group[i] = line;
                    (sum, group)
                }
                _ => {
                    // Turn the three rucksacks into bytes
                    let (first, second) = (group[0].as_bytes(), group[1].as_bytes());
                    // Find the first item in the line that also exists in the second and third line
                    let item = line
                        .bytes()
                        .find(|b| first.contains(b) && second.contains(b))
                        .unwrap();
                    let priority = match item {
                        byte @ b'a'..=b'z' => byte - b'a' + 1, // If lowercase letter, assign priority between 1 and 26
                        byte @ b'A'..=b'Z' => byte - b'A' + 27, // If capital letter, assign priority between 27 and 32
                        _ => 0,
                    };

                    (sum + priority as usize, group)
                }
            }
        });

    format!("{}", priority)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_3() {
        assert_eq!(solve_1(EXAMPLE), "157");
        assert_eq!(solve_1(DATA), "7701");
        assert_eq!(solve_2(EXAMPLE), "70");
        assert_eq!(solve_2(DATA), "2644");
    }

    #[bench]
    fn bench_3_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_3_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
