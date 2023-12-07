#![feature(test)]
#![allow(dead_code)]
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
type Card = u16; // 13 cards

fn sort_five(x: [u16; 5]) -> [u16; 5] {
    let [a, b, c, d, e] = x;

    let [a, b] = if a < b { [a, b] } else { [b, a] };
    let [c, d] = if c < d { [c, d] } else { [d, c] };
    // a<b, c<d

    let [a, b, c, d] = if b < d { [a, b, c, d] } else { [c, d, a, b] };
    // a<b<d, c<d

    #[allow(clippy::collapsible_else_if)]
    let [a, b, d, e] = if e < b {
        if e < a {
            [e, a, b, d]
        } else {
            [a, e, b, d]
        }
    } else {
        if e < d {
            [a, b, e, d]
        } else {
            [a, b, d, e]
        }
    };
    // a<b<d<e, c<e
    #[allow(clippy::collapsible_else_if)]
    let [a, b, c, d] = if c < b {
        if c < a {
            [c, a, b, d]
        } else {
            [a, c, b, d]
        }
    } else {
        if c < d {
            [a, b, c, d]
        } else {
            [a, b, d, c]
        }
    };

    [a, b, c, d, e]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn solve_1(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let mut cards = hand.bytes().map(|b| match b {
                b'A' => 1 << 14,
                b'K' => 1 << 13,
                b'Q' => 1 << 12,
                b'J' => 1 << 11,
                b'T' => 1 << 10,
                b => 1 << (b & 0xf),
            } as Card);

            let mut cards = [
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
            ];

            let pos = (cards[0].trailing_zeros() << 16)
                + (cards[1].trailing_zeros() << 12)
                + (cards[2].trailing_zeros() << 8)
                + (cards[3].trailing_zeros() << 4)
                + cards[4].trailing_zeros();

            cards.sort();
            let [a, b, c, d, e] = cards;

            let trick = match (a | b | c | d | e).count_ones() {
                1 => Hand::FiveOfAKind,
                2 if b == d => Hand::FourOfAKind,
                2 => Hand::FullHouse,
                3 if a == c || b == d || c == e => Hand::ThreeOfAKind,
                3 => Hand::TwoPair,
                4 => Hand::OnePair,
                _ => Hand::HighCard,
            };

            (((trick as u32) << 20) + pos, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by_key(|&(hand, _)| hand);
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

fn solve_2(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let mut cards = hand.bytes().map(|b| match b {
                b'A' => 1 << 14,
                b'K' => 1 << 13,
                b'Q' => 1 << 12,
                b'J' => 1 << 11,
                b'T' => 1 << 10,
                b => 1 << (b & 0xf),
            } as Card);

            let cards = [
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
            ];

            let mut sorted = cards;
            sorted.sort();
            let [a, b, c, d, e] = sorted;

            let trick = match (a | b | c | d | e).count_ones() {
                1 => Hand::FiveOfAKind,
                2 if b == d => Hand::FourOfAKind,
                2 => Hand::FullHouse,
                3 if a == c || b == d || c == e => Hand::ThreeOfAKind,
                3 => Hand::TwoPair,
                4 => Hand::OnePair,
                _ => Hand::HighCard,
            };

            let mut cards = cards.map(|x| if x == 1 << 11 { 1 } else { x });
            let pos = (cards[0].trailing_zeros() << 16)
                + (cards[1].trailing_zeros() << 12)
                + (cards[2].trailing_zeros() << 8)
                + (cards[3].trailing_zeros() << 4)
                + cards[4].trailing_zeros();
            cards.sort();
            let jokers = match cards {
                [_, _, _, _, 1] => 5,
                [_, _, _, 1, _] => 4,
                [_, _, 1, _, _] => 3,
                [_, 1, _, _, _] => 2,
                [1, _, _, _, _] => 1,
                _ => 0,
            };

            let trick = match (jokers, trick) {
                (0, k) => k,
                (4 | 5, _) => Hand::FiveOfAKind,
                (1, Hand::HighCard) => Hand::OnePair,
                (1, Hand::OnePair) => Hand::ThreeOfAKind,
                (1, Hand::TwoPair) => Hand::FullHouse,
                (1, Hand::ThreeOfAKind) => Hand::FourOfAKind,
                (1, Hand::FourOfAKind) => Hand::FiveOfAKind,
                (2, Hand::OnePair) => Hand::ThreeOfAKind,
                (2, Hand::TwoPair) => Hand::FourOfAKind,
                (2, Hand::FullHouse) => Hand::FiveOfAKind,
                (3, Hand::ThreeOfAKind) => Hand::FourOfAKind,
                (3, Hand::FullHouse) => Hand::FiveOfAKind,
                _ => unimplemented!(),
            };

            (((trick as u32) << 20) + pos, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by_key(|&(hand, _)| hand);
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_7() {
        assert_eq!(solve_1(EXAMPLE), "6440");
        assert_eq!(solve_1(DATA), "253603890");
        assert_eq!(solve_2(EXAMPLE), "5905");
        assert_eq!(solve_2(DATA), "253630098");
    }

    #[bench]
    fn bench_07_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_07_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
