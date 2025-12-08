use std::io::Read;
use itertools::Itertools;
use AdventOfCode2025::solve;

type Parsed = Vec<(i64, i64, i64)>;

fn main() {
    solve(8, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines()
        .map(|line| {
            let mut line = line.split(",")
                .map(|s| s.parse::<i64>().unwrap());
            (line.next().unwrap(), line.next().unwrap(), line.next().unwrap())
        }).collect()
}

fn distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> f64 {
    let inner = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2);
    (inner as f64).sqrt()
}

fn part1(input: Parsed) -> usize {
    let mut dists = Vec::new();

    for i in 0..input.len() {
        for j in i+1..input.len() {
            dists.push((distance(input[i], input[j]), i, j));
        }
    }

    dists.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    dists.reverse();
    let mut groups = input.iter().enumerate().map(|(x, _)| vec![x]).collect::<Vec<_>>();

    for _ in 0..1000 {
        let next_pair = dists.pop().unwrap();

        let group1 = groups.iter().find_position(|xs| xs.iter().contains(&next_pair.1))
            .map(|x| x.0);
        let group2 = groups.iter().find_position(|xs| xs.iter().contains(&next_pair.2))
            .map(|x| x.0);

        if !group1.is_none() && group1 == group2 {
            continue;
        }

        match (group1, group2) {
            (Some(group1idx), Some(group2)) => {
                let group1 = groups.get_mut(group1idx).unwrap();
                let mut vec = group1.clone();
                group1.clear();
                let group2 = groups.get_mut(group2).unwrap();
                group2.append(&mut vec);
                groups.remove(group1idx);
            },
            (Some(group1), None) => {
                let mut group1 = groups.get_mut(group1).unwrap();
                group1.push(next_pair.2);
            },
            (None, Some(group2)) => {
                let mut group2 = groups.get_mut(group2).unwrap();
                group2.push(next_pair.1);
            },
            (None, None) => {
                groups.push(vec![next_pair.1, next_pair.2]);
            }
        }
    }

    let mut groups = groups.into_iter().map(|x| x.len()).collect::<Vec<_>>();

    groups.sort();
    groups.reverse();

    groups.into_iter().take(3).product()
}

fn part2(input: Parsed) -> i64 {
    let mut dists = Vec::new();

    for i in 0..input.len() {
        for j in i+1..input.len() {
            dists.push((distance(input[i], input[j]), i, j));
        }
    }

    dists.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    dists.reverse();
    let mut groups = input.iter().enumerate().map(|(x, _)| vec![x]).collect::<Vec<_>>();

    let mut pair = (0, 0);
    while groups.len() > 1 {
        let next_pair = dists.pop().unwrap();

        let group1 = groups.iter().find_position(|xs| xs.iter().contains(&next_pair.1))
            .map(|x| x.0);
        let group2 = groups.iter().find_position(|xs| xs.iter().contains(&next_pair.2))
            .map(|x| x.0);

        if !group1.is_none() && group1 == group2 {
            continue;
        }
        
        pair = (next_pair.1, next_pair.2);

        match (group1, group2) {
            (Some(group1idx), Some(group2)) => {
                let group1 = groups.get_mut(group1idx).unwrap();
                let mut vec = group1.clone();
                group1.clear();
                let group2 = groups.get_mut(group2).unwrap();
                group2.append(&mut vec);
                groups.remove(group1idx);
            },
            (Some(group1), None) => {
                let mut group1 = groups.get_mut(group1).unwrap();
                group1.push(next_pair.2);
            },
            (None, Some(group2)) => {
                let mut group2 = groups.get_mut(group2).unwrap();
                group2.push(next_pair.1);
            },
            (None, None) => {
                groups.push(vec![next_pair.1, next_pair.2]);
            }
        }
    }

    input[pair.0].0 * input[pair.1].0
}