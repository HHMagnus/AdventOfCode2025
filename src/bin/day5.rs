use AdventOfCode2025::solve;

fn main() {
    solve(5, part1, part2);
}

fn part1(input: &str) -> usize {
    let (ranges, nums) = parse(input);

    nums.into_iter()
        .filter(|num| ranges.iter().any(|(r1, r2)| r1 <= num && num <= r2))
        .count()
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut split = input.split("\n\n");
    let ranges = split.next().unwrap()
        .lines().map(|line| {
        let mut split = line.split("-");
        let r1 = split.next().unwrap().parse::<usize>().unwrap();
        let r2 = split.next().unwrap().parse::<usize>().unwrap();
        (r1, r2)
    }).collect::<Vec<_>>();
    let nums = split.next().unwrap().lines().map(|line| {
        line.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();
    (ranges, nums)
}

fn part2(input: &str) -> usize {
    let (mut ranges, nums) = parse(input);

    ranges.sort();

    let mut total = 0;
    let mut last_max = 0;
    for range in ranges {
        let r1 = range.0.max(last_max);
        if r1 > range.1 {
            continue;
        }
        total += range.1 - r1 + 1;
        last_max = range.1 + 1;
    }

    total
}