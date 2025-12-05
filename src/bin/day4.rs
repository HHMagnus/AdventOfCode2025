use std::collections::HashMap;
use AdventOfCode2025::solve;

type Parsed = HashMap<(i32, i32), Type>;

fn main() {
    solve(4, parse, part1, part2);
}

#[derive(PartialEq, Clone)]
enum Type {
    Paper,
    Empty
}

impl Type {
    fn new(c: char) -> Self {
        match c {
            '@' => Type::Paper,
            '.' => Type::Empty,
            x => panic!("Unknown type: {}", x)
        }
    }
}

fn part1(input: Parsed) -> usize {
    let removable = find_removable(&input);

    removable.iter().count()
}

fn find_removable(matrix: &HashMap<(i32, i32), Type>) -> Vec<(i32, i32)> {
    let mut removable = Vec::new();

    for (&(x, y), typ) in matrix.iter() {
        if typ == &Type::Empty {
            continue;
        }

        let neighbours = [
            (x - 1, y),
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
        ].into_iter()
            .filter(|xy| matrix.contains_key(xy) && matrix[xy] == Type::Paper)
            .count();

        if neighbours < 4 {
            removable.push((x, y));
        }
    }
    removable
}

fn parse(input: &str) -> Parsed {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, ch)| ((x as i32, y as i32), Type::new(ch)))
    }).collect::<HashMap<_, _>>()
}

fn part2(mut input: Parsed) -> usize {
    let mut total = 0;

    loop {
        let removable = find_removable(&input);
        if (removable.len() == 0) {
            break;
        }
        removable.iter().for_each(|xy| {
            input.remove(xy);
            total += 1;
        });
    }

    total
}