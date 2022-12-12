#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 10");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &str) -> String {
    let extract = |strength, cycle, x| match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => Some(strength + cycle as isize * x),
        _ => None,
    };

    let (strength, ..) = input.lines().fold(
        (0, 0, 1, [0; 256]),
        |(strength, mut cycle, mut x, mut addx), line| {
            x += addx[cycle];
            cycle += 1;
            let during = extract(strength, cycle, x);
            x += addx[cycle];
            if let Some(s) = line.strip_prefix("addx ") {
                let delta = s.parse::<isize>().unwrap();
                cycle += 1;
                addx[cycle] += delta;
            }
            let after = extract(strength, cycle, x);
            (during.unwrap_or(after.unwrap_or(strength)), cycle, x, addx)
        },
    );

    strength.to_string()
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn solve_2(input: &str) -> String {
    let sprite =
        |x: isize, cycle| b'#' + (b'.' - b'#') * (x.abs_diff(cycle as isize % 40) > 1) as u8;
    let (.., crt) = input.lines().fold(
        (0, 1, [0_isize; 256], [b'\n'; WIDTH * HEIGHT + HEIGHT + 2]),
        |(mut cycle, mut x, mut addx, mut crt), line| {
            x += addx[cycle];
            crt[cycle + 1 + cycle / WIDTH] = sprite(x, cycle);
            cycle += 1;
            x += addx[cycle];
            crt[cycle + 1 + cycle / WIDTH] = sprite(x, cycle);
            if let Some(s) = line.strip_prefix("addx ") {
                let delta = s.parse::<isize>().unwrap();
                cycle += 1;
                addx[cycle] += delta;
            }
            (cycle, x, addx, crt)
        },
    );

    let display = std::str::from_utf8(&crt).ok().unwrap().to_string();
    println!("{display}");
    display
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_10() {
        assert_eq!(solve_1(EXAMPLE), "13140");
        assert_eq!(solve_1(DATA), "14560");
        assert_eq!( solve_2(EXAMPLE), "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n.");
        assert_eq!(solve_2(DATA), "\n####.#..#.###..#..#.####.###..#..#.####.\n#....#.#..#..#.#..#.#....#..#.#..#....#.\n###..##...#..#.####.###..#..#.#..#...#..\n#....#.#..###..#..#.#....###..#..#..#...\n#....#.#..#.#..#..#.#....#....#..#.#....\n####.#..#.#..#.#..#.####.#.....##..####.\n.");
    }

    #[bench]
    fn bench_10_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_10_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
