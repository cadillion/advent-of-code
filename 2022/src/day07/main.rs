#![feature(test)]
extern crate test;

use std::collections::BTreeMap;

pub const EXAMPLE: &str = include_str!("example.txt");
pub const DATA: &str = include_str!("data.txt");

fn main() {
    println!("Day 07");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

/// A directory structure that stores the sum of the file sizes inside and a list of child
/// directories
#[derive(Clone, Debug)]
pub struct Directory<'s> {
    size: usize,
    children: BTreeMap<&'s str, Self>,
}

impl<'s> Directory<'s> {
    /// Creates a new directory
    pub fn new() -> Self {
        Self {
            size: 0,
            children: BTreeMap::new(),
        }
    }

    /// Adds a child to the root directory at path
    pub fn add_child(&mut self, child: Directory<'s>, mut path: Vec<&'s str>) -> &mut Self {
        let end = path.pop().unwrap();
        let parent = self.get_child(path);
        parent.children.insert(end, child);
        self
    }

    /// Adds a file to the specified directory and increments the size of every parent
    pub fn add_file(&mut self, size: usize, mut path: Vec<&'s str>) -> &mut Self {
        self.size += size;
        let mut path = path.iter_mut();
        path.next();
        path.fold(self, |acc: &mut Directory, dir| {
            acc.children.entry(dir).and_modify(|dir| dir.size += size);
            acc.children.get_mut(dir).unwrap()
        })
    }

    /// Gets the child node at path
    pub fn get_child(&mut self, mut path: Vec<&'s str>) -> &mut Self {
        let mut path = path.iter_mut();
        path.next();
        path.fold(self, |acc: &mut Directory, dir| {
            acc.children.get_mut(dir).unwrap()
        })
    }

    /// Consumes the directory and flattens its children into a vector
    pub fn consume(&mut self) -> Vec<Self> {
        let mut vec: Vec<Self> = Vec::new();
        self.children
            .iter_mut()
            .for_each(|(_path, child)| vec.extend(child.consume()));
        vec.push(self.clone());
        vec
    }
}

/// Iterate through all the commands and build a map of the file_tree
/// Store the size of all the files on the directory parent.size, and a list of children directories on children
pub fn build_file_tree<'s>(input: &'s str) -> Directory<'s> {
    let (_path, root) = input.split("$ cd ").fold(
        (Vec::new(), Directory::new()),
        |(mut path, mut parent), block| {
            // Split the block into the directory and the directory contents
            let (dir, contents) = match block.split_once("\n$ ls\n") {
                Some(tuple) => tuple,
                None => {
                    path.pop();
                    return (path, parent);
                }
            };

            // Push the current path into the path tracker
            path.push(dir);

            // For each element of the contents, check if it's a directory or a file
            // Push directories into parent.childrent and add the files to parent.size
            contents
                .lines()
                .for_each(|line| match line.split_once(' ') {
                    Some(("dir", subdir)) => {
                        let child = Directory::new();
                        let mut child_path = path.clone();
                        child_path.push(subdir);
                        parent.add_child(child, child_path);
                    }
                    Some((size_str, _file)) => {
                        parent.add_file(size_str.parse::<usize>().unwrap(), path.clone());
                    }
                    _ => unreachable!(),
                });

            (path, parent)
        },
    );

    root
}

/// Find each directory with less than 100,000 bytes and sum them, even if they are nested in each other
fn solve_1(input: &str) -> String {
    build_file_tree(input)
        .consume()
        .into_iter()
        .map(|v| v.size)
        .filter(|v| v <= &100_000)
        .sum::<usize>()
        .to_string()
}

/// Find the smallest folder you can delete if the available disk space on the hard drive is 70000000 bytes
/// and the update requires 30000000 bytes free
fn solve_2(input: &str) -> String {
    let mut file_tree = build_file_tree(input);
    let free = 70000000 - file_tree.size;
    let needed = 30000000 - free;

    file_tree
        .consume()
        .into_iter()
        .map(|v| v.size)
        .filter(|v| v > &needed)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_7() {
        assert_eq!(solve_1(EXAMPLE), "95437");
        assert_eq!(solve_1(DATA), "1501149");
        assert_eq!(solve_2(EXAMPLE), "24933642");
        assert_eq!(solve_2(DATA), "10096985");
    }

    #[bench]
    fn bench_7_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_7_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
