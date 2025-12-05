use std::fmt::Display;
use std::fs::read_to_string;
use aoc_client::{AocClient, AocResult, PuzzleDay};

fn solve_part<F, R>(puzzle: String, file: &str, solver: F)
where F: Fn(&str) -> R,
      R: Display
{
    let result = solver(file);
    println!("{}: {}", puzzle, result);
}

pub fn solve<F1, F2, R1, R2>(day: PuzzleDay, part1: F1, part2: F2)
where F1: Fn(&str) -> R1,
      R1: Display,
      F2: Fn(&str) -> R2,
      R2: Display,
{
    let title = format!("Day {}", day);
    let input = get_input(day);
    if input.is_none() {
        println!("{}: Skipping due to no input", title);
        return;
    }
    let input = input.unwrap();
    run_part(day, part1, "1", &input);
    run_part(day, part2, "2", &input);
}

fn run_part<F, R>(day: PuzzleDay, solver: F, part: &str, input: &str)
where
    F: Fn(&str) -> R,
    R: Display
{
    let title = format!("Day {} part {}", day, part);
    solve_part(title, &input, solver);
}

fn get_input(day: PuzzleDay) -> Option<String> {
    let file_name = format!("input/day{}.txt", day);
    let file = read_to_string(&file_name);

    if let Ok(file) = file {
        if !file.is_empty() {
            return Some(file);
        }
    }

    println!("Input file not found. Retrieving...");

    let input = fetch_input(day);

    if input.is_err() {
        println!("Failed to fetch input file: {}", input.unwrap_err());
        return None;
    }
    let output = input.unwrap();

    std::fs::create_dir_all("input")
        .expect("Failed to create dir");
    std::fs::write(&file_name, &output)
        .expect("Unable to write input file");

    println!("Input file created: {}", file_name);

    Some(output)
}

fn fetch_input(day: PuzzleDay) -> AocResult<String> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2025)?
        .day(day)?
        .build()?;

    client.get_input()
}