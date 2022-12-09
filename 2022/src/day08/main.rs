#![feature(test)]
extern crate test;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 08");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

// Computes whether a tree is visible from the outer edge of the map
fn visible(
    grid: &[u8],   // the map
    depth: usize,  // the best case number of trees that can be seen
    start: usize,  // determines whether we traverse the map in reverse
    end: usize,    // - just exchange the start and end to go backwards
    skip_x: usize, // determines how the map is partitioned in each axis
    skip_y: usize, // - you want to jump either 1 or width (ie, to get to the next row)
) -> Vec<(u64, bool)> {
    let direction = if start < end { 1 } else { -1 };
    let traverse = |x, y| (start as i64 + (direction * x)) as usize * skip_x + y * skip_y;
    let height = |x, y| grid[traverse(x, y)];

    // Iterate through each row of the grid
    let (sight, _) = (0..depth).into_iter().fold(
        // Initialize a (view, visibility) map, along with a stack to hold intermediate values
        (vec![(0, false); grid.len()], Vec::new()),
        |(mut sight, mut seen), y| {
            // From the end of the direction of vision, step back to the tallest tree
            (0..(1 + start.abs_diff(end) as i64)).rev().for_each(|x| {
                // While the last item on the stack is shorter than the current, pop it off
                // the stack and store its directional view in the map with an "unseen" flag
                while let Some(true) = seen.last().map(|x2| height(x, y) >= height(*x2, y)) {
                    let x2 = seen.pop().unwrap();
                    sight[traverse(x2, y)] = (x2.abs_diff(x), false);
                }
                // Push the index of the current tree into the stack
                seen.push(x);
            });

            // Empty the stack and store the view of each visible tree in our visibility map
            seen.drain(..).for_each(|x| {
                sight[traverse(x, y)] = (x.unsigned_abs(), true);
            });

            (sight, seen)
        },
    );

    sight
}

/// Count the number of digits in the grid that are monotonically increasing from any vertical or
/// horizontal direction
fn solve_1(input: &str) -> String {
    // Get a flat string of bytes without the new_line characters
    let bump_map = input.lines().flat_map(|l| l.bytes()).collect::<Vec<_>>();

    // Get the length of a row
    let width: usize = input.lines().next().unwrap().len();
    // Get the length of a column from the total bytes divided by width
    let height: usize = bump_map.len() / width;

    // Calculate the visible trees in each direction by rotating the map
    let east = visible(&bump_map, height, 0, width - 1, 1, width);
    let south = visible(&bump_map, width, 0, height - 1, width, 1);
    let west = visible(&bump_map, height, width - 1, 0, 1, width);
    let north = visible(&bump_map, width, height - 1, 0, width, 1);

    // Collect the values and filter visible trees with a bitwise OR on each cardinal direction
    east.into_iter()
        .zip(south.into_iter())
        .zip(west.into_iter())
        .zip(north.into_iter())
        .filter(|(((e, s), w), n)| e.1 | s.1 | w.1 | n.1)
        .count()
        .to_string()
}

/// Count the number of digits in the grid that are smaller than the tallest trees
fn solve_2(input: &str) -> String {
    let bump_map = input
        .lines()
        .flat_map(|l| l.trim().bytes())
        .collect::<Vec<_>>();
    let width: usize = input.lines().next().unwrap().trim().len();
    let height: usize = bump_map.len() / width;

    let east = visible(&bump_map, height, 0, width - 1, 1, width);
    let south = visible(&bump_map, width, 0, height - 1, width, 1);
    let west = visible(&bump_map, height, width - 1, 0, 1, width);
    let north = visible(&bump_map, width, height - 1, 0, width, 1);

    // Collect the values and map each tree's combined view in each direction
    east.into_iter()
        .zip(south.into_iter())
        .zip(west.into_iter())
        .zip(north.into_iter())
        .map(|(((e, s), w), n)| e.0 * s.0 * w.0 * n.0)
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_8() {
        assert_eq!(solve_1(EXAMPLE), "21");
        assert_eq!(solve_1(DATA), "1851");
        assert_eq!(solve_2(EXAMPLE), "8");
        assert_eq!(solve_2(DATA), "574080");
    }

    #[bench]
    fn bench_8_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_8_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
