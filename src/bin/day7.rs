use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use AdventOfCode2025::solve;

type Parsed = HashMap<(i32, i32), Point>;

#[derive(Clone, Copy)]
enum Point {
    Space,
    Splitter,
    Start,
}

impl Point {
    fn new(c: char) -> Self {
        match c {
            '^' => Self::Splitter,
            '.' => Self::Space,
            'S' => Self::Start,
            x => panic!("Unexpected character: {}", x),
        }
    }
}

fn main() {
    solve(7, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), Point::new(c)))
    }).collect()
}

fn part1(input: Parsed) -> usize {
    let s = input.iter().find(|(_, p)| matches!(p, Point::Start)).unwrap().0.clone();

    let max_y = input.iter().map(|((_, y), _)| y).max().unwrap().clone();

    let mut vecs = HashSet::new();
    vecs.insert(s);

    let mut splits = 0;

    while vecs.iter().any(|&(_, y)| y < max_y) {
        vecs = vecs.into_iter()
            .map(|(x, y)| {
                (x, y + 1)
            })
            .flat_map(|xy| {
                if input.contains_key(&xy) {
                    match input[&xy] {
                        Point::Splitter => {
                            splits += 1;
                            vec![(xy.0 - 1, xy.1), (xy.0 + 1, xy.1)]
                        },
                        _ => vec![xy],
                    }
                } else {
                    vec![xy]
                }
            })
            .collect()
    }

    splits
}

fn part2(input: Parsed) -> usize {
    let s = input.iter().find(|(_, p)| matches!(p, Point::Start)).unwrap().0.clone();

    let max_y = input.iter().map(|((_, y), _)| y).max().unwrap().clone();

    let mut vecs = vec![(s, 1)];

    while vecs.iter().any(|&((_, y), _)| y < max_y) {
        vecs = vecs.into_iter()
            .map(|((x, y), times)| {
                ((x, y + 1), times)
            })
            .flat_map(|(xy, times)| {
                if input.contains_key(&xy) {
                    match input[&xy] {
                        Point::Splitter => vec![((xy.0 - 1, xy.1), times), ((xy.0 + 1, xy.1), times)],
                        _ => vec![(xy, times)],
                    }
                } else {
                    vec![(xy, times)]
                }
            })
            .sorted()
            .chunk_by(|(xy, _)| xy.clone())
            .into_iter()
            .map(|(xy, group)| {
                (xy.clone(), group.map(|(_, times)| times).sum())
            })
            .collect()
    }

    vecs.into_iter().map(|(_, times)| times).sum()
}