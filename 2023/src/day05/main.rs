#![feature(test)]
#![allow(dead_code)]
extern crate test;
use std::{collections::*, ops::Range};

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 05");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

type Route<'a> = HashMap<&'a str, HashMap<&'a str, HashMap<Range<usize>, Range<usize>>>>;

fn get_next(map: &Route, src: &str, dst: &str, n: usize) -> usize {
    match map
        .get(src)
        .unwrap()
        .get(dst)
        .unwrap()
        .iter()
        .find(|(s, _)| s.contains(&n))
    {
        Some((s, d)) => d.start + n - s.start,
        None => n,
    }
}

fn solve_1(input: &str) -> String {
    let mut map: HashMap<_, HashMap<_, HashMap<_, Range<usize>>>> = HashMap::new();

    input
        .lines()
        .filter(|l| l.starts_with(char::is_alphanumeric))
        .skip(1)
        .fold(("", ""), |(src, dst), l| {
            if !l.starts_with(char::is_numeric) {
                let (src, tail) = l.split_once("-to-").unwrap();
                let (dst, _) = tail.split_once(' ').unwrap();
                map.entry(src).or_default().entry(dst).or_default();
                (src, dst)
            } else {
                if let [d, s, len] = l.split(' ').collect::<Vec<_>>()[..] {
                    let d = d.parse::<usize>().unwrap();
                    let s = s.parse::<usize>().unwrap();
                    let len = len.parse::<usize>().unwrap();
                    map.entry(src)
                        .or_default()
                        .entry(dst)
                        .or_default()
                        .insert(s..s + len, d..d + len);
                }
                (src, dst)
            }
        });

    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .into_iter()
        .skip(1)
        .map(|n| {
            let sed = n.parse::<usize>().unwrap();
            let sol = get_next(&map, "seed", "soil", sed);
            let frt = get_next(&map, "soil", "fertilizer", sol);
            let wtr = get_next(&map, "fertilizer", "water", frt);
            let lgt = get_next(&map, "water", "light", wtr);
            let tmp = get_next(&map, "light", "temperature", lgt);
            let hmt = get_next(&map, "temperature", "humidity", tmp);
            get_next(&map, "humidity", "location", hmt)
        })
        .min()
        .unwrap()
        .to_string()
}

fn get_nexts<'a>(
    map: &'a Route,
    src: &str,
    dst: &str,
    min: usize,
    max: usize,
) -> impl Iterator<Item = (Range<usize>, Range<usize>)> + 'a {
    let route = map.get(src).unwrap().get(dst).unwrap();
    println!("{route:?}");

    route
        .iter()
        .filter(move |(s, _)| s.start < min && min < s.end || s.start < max && max < s.end)
        .filter_map(move |(s, d)| {
            if s.start < min {
                Some((min..s.end, d.start + min - s.start..d.end))
            } else if max < s.end {
                Some((s.start..max, d.start..d.end + max - s.end))
            } else {
                None
            }
        })
}

fn intersect(a: Range<i64>, b: Range<i64>) -> [Range<i64>; 3] {
    let i = a.start.max(b.start)..a.end.min(b.end);
    let l = b.start..i.start;
    let r = i.end..b.end;
    [l, i, r]
}

fn solve_2(input: &str) -> String {
    let maps = input.split("\n\n").collect::<Vec<_>>();
    let mut seeds = maps[0][7..]
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<_>>();
    let mut seeds_ = vec![];

    for map in maps {
        for l in map.lines().skip(1) {
            if let [dst, src, len] = l
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                let mut i = 0;
                while i < seeds.len() {
                    let [lft, ixn, rgt] = intersect(src..src + len, seeds[i].clone());
                    match ixn.is_empty() {
                        true => i += 1,
                        false => {
                            seeds_.push(ixn.start - src + dst..ixn.end - src + dst);
                            seeds.swap_remove(i);
                            if !lft.is_empty() {
                                seeds.push(lft);
                            }
                            if !rgt.is_empty() {
                                seeds.push(rgt);
                            }
                        }
                    }
                }
            }
        }
        seeds.append(&mut seeds_);
    }
    seeds.iter().map(|r| r.start).min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_5() {
        assert_eq!(solve_1(EXAMPLE), "35");
        assert_eq!(solve_1(DATA), "323142486");
        assert_eq!(solve_2(EXAMPLE), "46");
        assert_eq!(solve_2(DATA), "79874951");
    }

    #[bench]
    fn bench_05_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_05_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
