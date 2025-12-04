use std::collections::HashMap;
use AdventOfCode2025::solve;

fn main() {
    solve(4, part1, part2);
}

#[derive(PartialEq)]
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

fn part1(input: &str) -> usize {
    let matrix = parse(input);

    let removable = find_removable(&matrix);

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

fn parse(input: &str) -> HashMap<(i32, i32), Type> {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, ch)| ((x as i32, y as i32), Type::new(ch)))
    }).collect::<HashMap<_, _>>()
}

fn part2(input: &str) -> usize {
    let mut matrix = parse(input);

    let mut total = 0;

    loop {
        let removable = find_removable(&matrix);
        if (removable.len() == 0) {
            break;
        }
        removable.iter().for_each(|xy| {
            matrix.remove(xy);
            total += 1;
        });
    }

    total
}