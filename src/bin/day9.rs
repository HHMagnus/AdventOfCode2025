use AdventOfCode2025::solve;

type Parsed = Vec<(usize, usize)>;

fn main() {
    solve(9, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines().map(|line| {
        let mut split = line.split(',');
        (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
    }).collect()
}

fn part1(input: Parsed) -> usize {
    let mut max = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (a, b) = input[i];
            let (x, y) = input[j];
            let n = (a.abs_diff(x)+1) * (b.abs_diff(y)+1);
            max = max.max(n);
        }
    }

    max
}

fn part2(input: Parsed) -> usize {
    0
}