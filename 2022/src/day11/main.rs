#![feature(test)]
extern crate test;

use std::collections::VecDeque;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 11");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

pub type Items = VecDeque<usize>;
pub type Operation = (String, Result<usize, std::num::ParseIntError>);
pub type Dividend = usize;
pub type Test = (usize, usize);
pub type Monkey = (Items, Operation, Dividend, Test);

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            monkey.lines().enumerate().skip(1).fold(
                (VecDeque::new(), (String::new(), Ok(0)), 1, (0, 0)),
                |(items, op, div, test), (j, line)| match j {
                    1 => {
                        let new_items = line
                            .split_once(": ")
                            .unwrap()
                            .1
                            .split(", ")
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect();
                        (new_items, op, div, test)
                    }
                    2 => {
                        let (sym, num) =
                            line.split_once("old ").unwrap().1.split_once(' ').unwrap();
                        (items, (sym.to_string(), num.parse::<usize>()), div, test)
                    }
                    3 => {
                        let new_div = line.split_once("by ").unwrap().1.parse::<usize>().unwrap();
                        (items, op, new_div, test)
                    }
                    4 => {
                        let new_test = line
                            .split_once("monkey ")
                            .unwrap()
                            .1
                            .parse::<usize>()
                            .unwrap();
                        (items, op, div, (new_test, test.1))
                    }
                    _5 => {
                        let new_test = line
                            .split_once("monkey ")
                            .unwrap()
                            .1
                            .parse::<usize>()
                            .unwrap();
                        (items, op, div, (test.0, new_test))
                    }
                },
            )
        })
        .collect()
}

fn solve_1(input: &str) -> String {
    // Get the list of rules
    let mut monkeys = parse_input(input);
    let inspect = |op, div, test: Test, mut monkeys: Vec<Monkey>| match (op / 3) % div {
        0 => {
            monkeys[test.0].0.push_back(op / 3);
            monkeys
        }
        _ => {
            monkeys[test.1].0.push_back(op / 3);
            monkeys
        }
    };

    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            // For each monkey, iterate through each item they are holding
            while let Some(item) = monkeys[index].0.pop_front() {
                // Count the number of times we visited an item for each monkey
                inspections[index] += 1;
                // Apply the listed operation in line 2 then divide by 3
                // Apply the test listed in line 3 with values in line 4 (true) and 5 (false)
                let val = monkeys[index].1 .1.as_ref().unwrap_or(&item);
                monkeys = match monkeys[index].1 .0.as_str() {
                    "+" => inspect(item + val, monkeys[index].2, monkeys[index].3, monkeys),
                    "*" => inspect(item * val, monkeys[index].2, monkeys[index].3, monkeys),
                    _ => unreachable!(),
                }
            }
        }
    }

    // Get the two monkeys with max inspections and multiply the numbers together
    let (ult, pen) = inspections.into_iter().fold((0, 0), |(ult, pen), cur| {
        (ult.max(cur), pen.max(cur.min(ult)))
    });
    (ult * pen).to_string()
}

fn solve_2(input: &str) -> String {
    // Get the list of rules
    let mut monkeys = parse_input(input);
    let modulus = monkeys.iter().map(|m| m.2).product::<usize>();

    let inspect = |op, div, test: Test, mut monkeys: Vec<Monkey>| match op % div {
        0 => {
            monkeys[test.0].0.push_back(op % modulus);
            monkeys
        }
        _ => {
            monkeys[test.1].0.push_back(op % modulus);
            monkeys
        }
    };

    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            while let Some(item) = monkeys[index].0.pop_front() {
                inspections[index] += 1;
                let val = monkeys[index].1 .1.as_ref().unwrap_or(&item);
                monkeys = match monkeys[index].1 .0.as_str() {
                    "+" => inspect(item + val, monkeys[index].2, monkeys[index].3, monkeys),
                    "*" => inspect(item * val, monkeys[index].2, monkeys[index].3, monkeys),
                    _ => unreachable!(),
                }
            }
        }
    }

    // Get the two monkeys with max inspections and multiply the numbers together
    let (ult, pen) = inspections.into_iter().fold((0, 0), |(ult, pen), cur| {
        (ult.max(cur), pen.max(cur.min(ult)))
    });
    println!("{ult} {pen}");
    (ult * pen).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_11() {
        assert_eq!(solve_1(EXAMPLE), "10605");
        assert_eq!(solve_1(DATA), "99852");
        assert_eq!(solve_2(EXAMPLE), "2713310158");
        assert_eq!(solve_2(DATA), "25935263541");
    }

    #[bench]
    fn bench_11_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_11_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
