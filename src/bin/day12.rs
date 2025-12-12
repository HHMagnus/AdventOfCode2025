use std::collections::HashMap;
use AdventOfCode2025::solve;

type Parsed = (HashMap<usize, Vec<(usize, usize)>>, Vec<((usize, usize), Vec<usize>)>);

fn main() {
    solve(12, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    let mut map = HashMap::new();
    let mut vec = Vec::new();
    for split in file.split("\n\n") {
        let lines = split.lines().collect::<Vec<_>>();
        if !lines[0].contains("x") {
            let index = lines[0].replace(":", "").parse::<usize>().unwrap();
            let vec = lines[1..].into_iter().enumerate()
                .flat_map(|(y, line)| line.chars().enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| (x, y)))
                .collect::<Vec<_>>();
            map.insert(index, vec);
        } else {
            vec = lines.into_iter().map(|line| {
                let mut split = line.split(": ");
                let mut split2 = split.next().unwrap().split("x");
                let x = split2.next().unwrap().parse::<usize>().unwrap();
                let y = split2.next().unwrap().parse::<usize>().unwrap();
                let indexes = split.next().unwrap().split(" ")
                    .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
                ((x, y), indexes)
            }).collect::<Vec<_>>();
        }
    }
    (map, vec)
}

fn part1(input: Parsed) -> usize {
    input.1.into_iter().filter(|((x, y), indexes)| {
        let size = y * x;

        let total = indexes.iter().enumerate().map(|(idx, amount)| input.0[&idx].len() * amount).sum::<usize>();
        size > total
    }).count()
}

fn part2(input: Parsed) -> usize {
    panic!("No part 2 for last day");
}