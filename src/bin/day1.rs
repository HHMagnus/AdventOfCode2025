use AdventOfCode2025::solve;

type Parsed = Vec<(bool, i32)>;

fn main() {
    solve(1, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines().map(|line| {
        let left = line.starts_with('L');
        let pos = line[1..].parse::<i32>().unwrap();
        (left, pos)
    }).collect::<Vec<_>>()
}

fn part1(input: Parsed) -> i32 {
    let mut clock = 50;

    let mut total = 0;
    for i in input {
        clock += (if i.0 { -1 } else { 1 }) * i.1;
        clock += 10000;
        clock %= 100;
        if clock == 0 {
            total += 1;
        }
    }

    total
}

fn part2(input: Parsed) -> i32 {
    let mut clock = 50;

    let mut total = 0;
    for i in input {
        for _ in 0..i.1 {
            clock += (if i.0 { -1 } else { 1 });
            clock %= 100;
            if clock == 0 {
                total += 1;
            }
        }
    }

    total
}