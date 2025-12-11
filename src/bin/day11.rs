use std::collections::HashMap;
use AdventOfCode2025::solve;

type Parsed = HashMap<String, Vec<String>>;

fn main() {
    solve(11, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines().map(|line| {
        let mut split = line.split(": ");
        let point = split.next().unwrap().to_string();
        let list = split.next().unwrap().split(" ")
            .map(|x| x.to_string())
            .collect();
        (point, list)
    }).collect()
}

fn part1(input: Parsed) -> usize {
    dfs_search(&input, "you", "out")
}

fn dfs_search_inner(input: &Parsed, curr: &str, end: &str, prev: &mut HashMap<String, usize>) -> usize {
    if let Some(value) = prev.get(curr) {
        return *value;
    }

    if curr == end {
        prev.insert(curr.to_string(), 1);

        return 1;
    }

    if curr == "out" {
        prev.insert(curr.to_string(), 0);

        return 0;
    }

    let mut total = 0;

    for neighbour in &input[curr] {
        total += dfs_search_inner(input, neighbour.as_str(), end, prev);
    }

    prev.insert(curr.to_string(), total);

    total
}

fn dfs_search(input: &Parsed, curr: &str, end: &str) -> usize {
    let mut visited = HashMap::new();
    dfs_search_inner(input, curr, end, &mut visited)
}

fn part2(input: Parsed) -> usize {
    let road1 = dfs_search(&input, "svr", "dac")
        * dfs_search(&input, "dac", "fft")
        * dfs_search(&input, "fft", "out");
    let road2 = dfs_search(&input, "svr", "fft")
        * dfs_search(&input, "fft", "dac")
        * dfs_search(&input, "dac", "out");
    road1 + road2
}