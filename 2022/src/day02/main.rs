#![feature(test)]
extern crate test;

use std::cmp::Ordering;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 2");
    let r1 = solve_1(DATA);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Find your total points if playing as XYZ: Rock (X) is 1 point, Paper (Y) is 2 points, Scissors (Z) is 3 points,
/// if a Draw (A X, B Y, C Z) give an extra 3 points,
/// and a Win (C X, A Y, B Z) give an extra 6 points
pub fn solve_1(input: &str) -> String {
    let points = input.lines().fold(0, |acc, line| {
        // Extract the string directly as bytes so we don't have to serialize them into characters
        let l = line.as_bytes();
        // Convert the first and third character into the Hand type based on the byte presented
        let (opponent, own): (Hand, Hand) = (l[0].into(), l[2].into());

        // Compare the two Hands and convert the resulting Ordering into a score
        let result = score(own.cmp(&opponent));
        // Cast the enum types to a usize and add them to our running total
        acc + own as usize + result
    });

    format!("{}", points)
}

/// Find the total points if Rock (A) is 1 point, Paper (B) is 2 points, Scissors (C) is 3 points,
/// and Lose (X) is 0 points, Draw (Y) is 3 points, and Win (Z) is 6 points
pub fn solve_2(input: &str) -> String {
    let points = input.lines().fold(0, |acc, l| {
        // Extract the string directly as bytes so we don't have to serialize them into characters
        let l = l.as_bytes();
        // Convert the first and third character into a Hand type and a Ordering type based on the byte presented
        let (opponent, strategy): (Hand, Ordering) = (
            l[0].into(),
            match l[2] {
                b'X' => Ordering::Less,
                b'Y' => Ordering::Equal,
                _z => Ordering::Greater,
            },
        );
        // Get the hand you should play based on the required strategy and whatever hand your opponent played
        let own = match strategy {
            Ordering::Equal => opponent,
            Ordering::Greater => !opponent,
            Ordering::Less => !!opponent,
        };

        // Cast the enum types to a usize and add them to our running total
        acc + own as usize + score(strategy)
    });

    format!("{}", points)
}

pub fn score(ord: Ordering) -> usize {
    (ord as i8 + 1) as usize * 3
}

/// Establishes the score for playing a given hand
#[derive(Eq, PartialEq, PartialOrd)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// Provides a conversion function from a byte character to a Hand
impl From<u8> for Hand {
    fn from(character: u8) -> Self {
        match character {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

/// Provides an explicit ordering function instead of deriving so that the enum ordering can be cyclical
/// Add one so hands can be less than while unsized, add 3 to prevent negative values (eg Rock - Scissors),
/// modulo so Scissors wraps properly to Rock
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        ((1 + 3 + *self as usize - *other as usize) % 3).cmp(&1_usize)
    }
}

/// Overrides standard behavior of ! operator to provide the winning hand against the current hand
impl std::ops::Not for Hand {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_2() {
        assert_eq!(solve_1(EXAMPLE), "15");
        assert_eq!(solve_1(DATA), "12276");
        assert_eq!(solve_2(EXAMPLE), "12");
        assert_eq!(solve_2(DATA), "9975");
    }

    #[bench]
    fn bench_2_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_2_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
