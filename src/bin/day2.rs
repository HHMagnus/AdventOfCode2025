use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2025)?
        .day(2)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(input.clone());
    let part1 = client.submit_answer(1, answer_part1)?;
    println!("{:?}", part1);

    let answer_part2 = part2(input.clone());
    let part2 = client.submit_answer(2, answer_part2)?;
    println!("{:?}", part2);

    Ok(())
}

fn part1(file: String) -> String {
    let input = parse(file);

    input.iter()
        .flat_map(|range| (range.0..range.1).into_iter())
        .filter(is_invalid_twice)
        .sum::<i64>()
        .to_string()
}

fn parse(file: String) -> Vec<(i64, i64)> {
    file.split_ascii_whitespace().next().unwrap().split(",").map(|range| {
        let mut split = range.split("-");
        (split.next().unwrap().parse::<i64>().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }).collect::<Vec<_>>()
}

fn is_invalid_twice(num: &i64) -> bool {
    let str = num.to_string();
    if str.len() % 2 != 0 {
        return false;
    }

    let chars = str.chars().collect::<Vec<_>>();
    let part1 = &chars[0..chars.len()/2];
    let part2 = &chars[chars.len()/2..chars.len()];
    part1 == part2
}

fn is_invalid_more(num: &i64) -> bool {
    let str = num.to_string();

    let mut substr = Vec::new();

    let iter = str.chars().collect::<Vec<char>>();

    'o: for i in 0..iter.len()/2 {
        substr.push(iter[i]);

        for j in (i+1..iter.len()).step_by(i+1) {
            let end = j+i+1;
            if end > iter.len() {
                continue 'o;
            }
            let subpart = &iter[j..end];
            if substr != subpart {
                continue 'o;
            }
        }
        return true;
    }

    false
}

fn part2(file: String) -> String {
    let input = parse(file);

    input.iter()
        .flat_map(|range| (range.0..range.1).into_iter())
        .filter(is_invalid_more)
        .sum::<i64>()
        .to_string()
}