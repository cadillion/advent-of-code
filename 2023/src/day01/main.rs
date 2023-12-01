#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const EXAMPLE_2: &str = include_str!("example_2.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 1");
    let r1 = solve_1(DATA);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Find the sum of the two digit numbers comprising the first and last digit in each row
fn solve_1(input: &str) -> String {
    let get_digits = |l: &str| -> usize {
        let ten = l.bytes().find(|b| b.is_ascii_digit()).unwrap() - b'0';
        let one = l.bytes().rfind(|b| b.is_ascii_digit()).unwrap() - b'0';
        ten as usize * 10 + one as usize
    };

    input.lines().map(get_digits).sum::<usize>().to_string()
}

/// Find the sum of the two digit numbers comprising the first and last digit in each row, even if
/// spelled out in english, ie "one"
fn solve_2(input: &str) -> String {
    const NUMS: [&[u8]; 10] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    let get_digits = |sum, l: &str| -> usize {
        let l = l.as_bytes();
        let mut ten = 0;
        let mut one = 0;

        'fwd: for i in 0..l.len() {
            for (n, num) in NUMS.iter().enumerate() {
                if l[i].is_ascii_digit() {
                    ten = (l[i] - b'0') as usize;
                    break 'fwd;
                }
                if l.get(i..i + num.len()) == Some(num) {
                    ten = n;
                    break 'fwd;
                }
            }
        }
        'rev: for i in (0..l.len()).rev() {
            for (n, num) in NUMS.iter().enumerate() {
                if l[i].is_ascii_digit() {
                    one = (l[i] - b'0') as usize;
                    break 'rev;
                }
                if l.get(i - num.len()..i) == Some(num) {
                    one = n;
                    break 'rev;
                }
            }
        }

        sum + ten * 10 + one
    };

    input.lines().fold(0, get_digits).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_1() {
        assert_eq!(solve_1(EXAMPLE), "142");
        assert_eq!(solve_1(DATA), "54605");
        assert_eq!(solve_2(EXAMPLE_2), "281");
        assert_eq!(solve_2(DATA), "55429");
    }

    #[bench]
    fn bench_1_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_1_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
