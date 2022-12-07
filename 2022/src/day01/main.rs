#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 1");
    let r1 = solve_1(DATA);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// Find the grouped block of integers with the largest sum
fn solve_1(input: &str) -> String {
    // Split the input at every double line break and track the index of the string slice we're observing
    let (_max_elf, max_calories) = input // &'static str
        .split("\n\n")
        .enumerate()
        // Initialize state with an index for the elf and a total for their associated calories
        .fold(
            (0, 0),
            |(max_elf, max_calories), (current_elf, inventory)| {
                // Split the current string slice at each line break and convert the string to an unsigned integer sum
                let current_calories = inventory
                    .lines()
                    .fold(0, |acc, cal| acc + cal.parse::<usize>().unwrap_or(0));

                // Compare the current elf with the elf carrying the most calories we've seen, and return the greater amount
                if current_calories < max_calories {
                    (max_elf, max_calories)
                } else {
                    (current_elf, current_calories)
                }
            },
        );

    format!("{}", max_calories)
}

/// Find the three largest grouped blocks of integers with the largest sums
fn solve_2(input: &str) -> String {
    // Store a constant value that represents the number of elves whose inventories we wish to track
    const TOP_ELVES: usize = 3;

    // Split the input at every double line break and track the index of the string slice we're observing
    let (_max_elves, shared_calories) = input
        .split("\n\n")
        .enumerate()
        // Initialize state with fixed-size slices so we can operate entirely on the stack instead of the heap
        .fold(
            ([0; TOP_ELVES], [0; TOP_ELVES]),
            |(mut max_elves, mut shared_calories), (current_elf, inventory)| {
                // Split the current string slice at each line break and convert the string to an unsigned integer sum
                let current_calories = inventory
                    .lines()
                    .fold(0, |prev, cal| prev + cal.parse::<usize>().unwrap());

                // Find the index of the first elf holding fewer calories than the current elf, if there are any
                if let Some(i) = shared_calories.iter().position(|c| c < &current_calories) {
                    // Iterate through all the elves holding fewer calories than this elf and demote them one place
                    (i + 1..TOP_ELVES).rev().for_each(|i| {
                        max_elves[i] = max_elves[i - 1];
                        shared_calories[i] = shared_calories[i - 1];
                    });

                    // Insert the current elf into the last vacated slot after demotions
                    max_elves[i] = current_elf;
                    shared_calories[i] = current_calories;
                };

                // Return the latest state after mutating in place
                (max_elves, shared_calories)
            },
        );

    // Collect the calorie totals of the top elves and add them together
    let max_calories: usize = shared_calories.iter().sum();

    format!("{}", max_calories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_1() {
        assert_eq!(solve_1(EXAMPLE), "24000");
        assert_eq!(solve_1(DATA), "69912");
        assert_eq!(solve_2(EXAMPLE), "45000");
        assert_eq!(solve_2(DATA), "208180");
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
