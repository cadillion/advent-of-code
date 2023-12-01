#![feature(test)]
#![allow(dead_code)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 15");
    let r1 = solve_1(10, EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(2_000_000, DATA);
    println!("Part 2: {r2}");
}

fn solve_1(row: isize, input: &str) -> String {
    // plot the sensors and beacons
    let seen = input.lines().fold((0, 0), |acc, line| {
        // parse locations
        let &[sx, sy, bx, by] = &line
                .split(&['=', ',', ':'])
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()[..] else {unreachable!()};

        // get manhattan distance between points
        let md = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;

        // mark any area within the manhattan distance of sensor and beacon as seen
        let fog = md - sy.abs_diff(row) as isize;
        // extend the unseen range in the row to include the new seen tiles
        if sy - md <= row && row <= sy + md {
            return match (acc.0 <= sx - fog, sx + fog <= acc.1) {
                (true, true) => acc,
                (true, false) => (acc.0, sx + fog),
                (false, true) => (sx - fog, acc.1),
                (false, false) => (sx - fog, sx + fog),
            };
        }
        acc
    });

    // count the number of seen tiles in row 10 for example, 2_000_000 for data
    (seen.1 - seen.0).to_string()
}

fn solve_2(rows: isize, input: &str) -> String {
    let width = rows * 2;
    let frequency = |x: usize, y: usize| x * 4_000_000 + y;
    let min = |n: isize| 0.max(n) as usize;
    let max = |n: isize| (width - 1).min(n) as usize;

    // get the sensor positions and manhattan distances
    let sensors = input.lines().fold(vec![], |mut acc, line| {
        // parse locations
        let &[sx, sy, bx, by] = &line
                .split(&['=', ',', ':'])
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()[..] else {unreachable!()};

        // get manhattan distance between points
        let md = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;

        acc.push((sx, sy, md));
        acc
    });

    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(sensors.len());
    for y in 0..(width as usize) {
        ranges.clear();
        sensors
            .iter()
            // filter out sensors that don't cover this area and mark the sight range in this row
            .filter_map(|(sx, sy, md)| {
                let fog = md - sy.abs_diff(y as isize) as isize;
                match 0 < fog {
                    true => Some((min(sx - fog), max(sx + fog))),
                    false => None,
                }
            })
            .for_each(|range| ranges.push(range));

        // look at each sensor's output in this row and look for a discontinuity
        let mut x: usize = 0;
        while x < width as usize {
            match ranges
                .iter()
                .filter(|(_, x_max)| x <= *x_max)
                .fold(x, |pos, (min_x, max_x)| {
                    // if the range includes the previous maximum, store the range max + 1
                    if (min_x..=max_x).contains(&&pos) {
                        return max_x + 1;
                    }
                    pos
                }) {
                cur if cur == x => return frequency(x, y).to_string(),
                cur => x = cur,
            };
        }
    }

    "Did not find beacon".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_15() {
        assert_eq!(solve_1(10, EXAMPLE), "26");
        assert_eq!(solve_1(2_000_000, DATA), "6425133");
        assert_eq!(solve_2(10, EXAMPLE), "56000011");
        assert_eq!(solve_2(2_000_000, DATA), "10996191429555");
    }

    #[bench]
    fn bench_15_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(2_000_000), black_box(DATA)));
    }

    #[bench]
    fn bench_15_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(2_000_000), black_box(DATA)));
    }
}
