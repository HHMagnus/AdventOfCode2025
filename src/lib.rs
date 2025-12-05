use aoc_client::{AocClient, PuzzleDay, SubmissionOutcome};
use std::fmt::Display;
use std::fs::read_to_string;

pub fn solve<PF, P, F1, F2, R1, R2>(day: PuzzleDay, parse: PF, part1: F1, part2: F2)
where PF: Fn(&str) -> P,
      P: Clone,
      F1: Fn(P) -> R1,
      R1: Display,
      F2: Fn(P) -> R2,
      R2: Display,
{
    let client = aoc_client(day);
    let input = get_input(day, &client);
    if input.is_none() {
        println!("Day {}: Skipping due to no input", day);
        return;
    }
    let input = input.unwrap();
    let parsed = parse(&input);
    let part1_result = run_part(day, part1, "1", parsed.clone());
    let outcome = submit_part1(&client, part1_result.to_string());
    println!("Day {} part 1 submission: {:?}", day, outcome);
    match outcome {
        SubmissionOutcome::WrongLevel => {}
        _ => return
    };
    let part2_result = run_part(day, part2, "2", parsed);
    submit_part2(&client, part2_result.to_string());
}

fn run_part<P, F, R>(day: PuzzleDay, solver: F, part: &str, input: P) -> R
where
    F: Fn(P) -> R,
    R: Display
{
    let title = format!("Day {} part {}", day, part);
    let result = solver(input);
    println!("{}: {}", title, result);
    result
}

fn get_input(day: PuzzleDay, client: &AocClient) -> Option<String> {
    let file_name = format!("input/day{}.txt", day);
    let file = read_to_string(&file_name);

    if let Ok(file) = file {
        if !file.is_empty() {
            return Some(file);
        }
    }

    println!("Input file not found. Retrieving...");

    let input = fetch_input(client)?;

    std::fs::create_dir_all("input")
        .expect("Failed to create dir");
    std::fs::write(&file_name, &input)
        .expect("Unable to write input file");

    println!("Input file created: {}", file_name);

    Some(input)
}

fn fetch_input(client: &AocClient) -> Option<String> {
    let input = client.get_input();

    if input.is_err() {
        println!("Failed to fetch input file: {}", input.unwrap_err());
        return None;
    }

    Some(input.unwrap())
}

fn aoc_client(day: PuzzleDay) -> AocClient {
    AocClient::builder()
        .session_cookie_from_default_locations().expect("Failed to find session cookie")
        .year(2025).expect("Failed to get year")
        .day(day).expect("Failed to get day")
        .build().expect("Failed to build client")
}

fn submit_part1<R>(client: &AocClient, solution: R) -> SubmissionOutcome where R: Display {
    submit_part(client, "1", solution)
}

fn submit_part2<R>(client: &AocClient, solution: R) -> SubmissionOutcome where R: Display {
    submit_part(client, "2", solution)
}

fn submit_part<R>(client: &AocClient, part: &str, solution: R) -> SubmissionOutcome where R: Display {
    let outcome = client.submit_answer(part, solution)
        .expect("Failed to submit answer");
    outcome
}
