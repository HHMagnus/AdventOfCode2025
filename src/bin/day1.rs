
use aoc_client::{AocClient, AocResult};
use AdventOfCode2025::solve;

fn main() {
    solve(1, part1, part2);
}

fn main2() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2025)?
        .day(1)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(&input.clone());
    let part1 = client.submit_answer(1, answer_part1)?;
    println!("{:?}", part1);

    let answer_part2 = part2(&input.clone());
    let part2 = client.submit_answer(2, answer_part2)?;
    println!("{:?}", part2);

    Ok(())
}

fn part1(file: &str) -> i32 {
    let input = parse(file);

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

fn parse(file: &str) -> Vec<(bool, i32)> {
    file.lines().map(|line| {
        let left = line.starts_with('L');
        let pos = line[1..].parse::<i32>().unwrap();
        (left, pos)
    }).collect::<Vec<_>>()
}

fn part2(file: &str) -> i32 {
    let input = parse(file);

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