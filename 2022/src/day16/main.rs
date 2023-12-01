#![feature(test)]
#![allow(dead_code)]
extern crate test;

use pathfinding::directed::bfs::bfs;
use std::collections::HashMap;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 16");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow_rate: usize,
    edges: Vec<String>,
}

impl Ord for Valve {
    fn cmp(&self, other: &Valve) -> std::cmp::Ordering {
        self.flow_rate.cmp(&other.flow_rate)
    }
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Valve {
    fn next(&mut self, map: &mut HashMap<String, Valve>) -> Vec<Self> {
        self.edges.sort();
        self.edges
            .iter()
            .filter_map(|e| map.remove(e))
            .collect::<Vec<Self>>()
    }

    fn explore(
        &mut self,
        map: &mut HashMap<String, Valve>,
        dag: &mut Vec<Vec<Valve>>,
        timer: usize,
    ) {
        let next = self.next(map);
        dag[timer].push(self.clone());
        next.into_iter()
            .for_each(|mut v| v.explore(map, dag, timer + 1))
    }
}

fn solve_1(input: &str) -> String {
    let map = input.lines().fold(HashMap::new(), |mut map, line| {
        let items: Vec<&str> = line.split(&[' ', '=', ';', ',']).collect();
        let name = items[1].to_string();
        map.insert(
            name.clone(),
            Valve {
                name,
                flow_rate: items[5].parse().unwrap(),
                edges: items[11..]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
        );
        map
    });
    // let mut dag = vec![vec![]; 58];
    let start = map.get("AA").unwrap();
    // start.explore(&mut map, &mut dag, 0);
    // dag.iter_mut().for_each(|v| v.sort());

    let successors = |(t, v): &(usize, Valve)| {
        v.edges
            .iter()
            .map(|e| (t.clone() - 1, map.get(e).unwrap().clone()))
    };
    let success = |(t, _v): &(usize, Valve)| t == &0;

    (bfs(&(30_usize, start.clone()), successors, success)
        .unwrap()
        .len())
    .to_string()
}

fn solve_2(input: &str) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_16() {
        assert_eq!(solve_1(EXAMPLE), "1651");
        assert_eq!(solve_1(DATA), "");
        assert_eq!(solve_2(EXAMPLE), "");
        assert_eq!(solve_2(DATA), "");
    }

    #[bench]
    fn bench_16_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_16_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
