use AdventOfCode2025::solve;

type Parsed = Vec<Vec<u64>>;

fn main() {
    solve(3, parse, part1, part2);
}

fn part1(input: Parsed) -> u64 {
    input.into_iter()
        .map(|line| {
            let mut vec = Vec::new();
            for i in 0..line.len()-1 {
                for j in i+1..line.len() {
                    vec.push(line[i] * 10 + line[j]);
                }
            }
            vec.into_iter().max().unwrap()
        }).sum()
}

fn parse(input: &str) -> Parsed {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn part2(input: Parsed) -> u64 {
    input.into_iter()
        .map(|line| {
            let mut vec = Vec::new();
            let mut last = 0;
            for behind in (0..12).rev() {
                let i = max_behind(&line[last..], behind);
                vec.push(i+last);
                last = i+last+1;
            }
            let mut res = 0;
            let mut mul = 1;
            vec.reverse();
            for i in vec {
                res += line[i] * mul;
                mul *= 10;
            }
            res
        }).sum()
}

fn max_behind(line: &[u64], behind: usize) -> usize {
    let mut max = 0;
    let mut ret = 0;
    for i in 0..line.len()-behind {
        if line[i] > max {
            max = line[i];
            ret = i;
        }
    }
    ret
}