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
    let mut x_walls = Vec::new();
    let mut y_walls = Vec::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (a, b) = input[i];
            let (x, y) = input[j];
            if a == x  {
                y_walls.push((x, (b, y)));
            }
            if b == y {
                x_walls.push((y, (a, x)));
            }
        }
    }

    let mut max = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (a, b) = input[i];
            let (x, y) = input[j];

            if hits_wall(((a, b), (x, y)), &x_walls, &y_walls) {
                continue;
            }

            let n = (a.abs_diff(x)+1) * (b.abs_diff(y)+1);
            max = max.max(n);
        }
    }

    max
}

fn hits_wall(
    ((a, b), (x, y)): ((usize, usize), (usize, usize)),
    x_walls: &Vec<(usize, (usize, usize))>,
    y_walls: &Vec<(usize, (usize, usize))>
) -> bool {
    let min_x = a.min(x);
    let max_x = a.max(x);
    let min_y = b.min(y);
    let max_y = b.max(y);

    if x_walls.iter().any(|wall| {
        let between_y = min_y < wall.0 && wall.0 < max_y;
        let before_x = wall.1.0.max(wall.1.1) < min_x;
        let after_x = wall.1.0.min(wall.1.1) > max_x;
        between_y && !before_x && !after_x
    }) {
        return true;
    }

    if y_walls.iter().any(|wall| {
        let between_x = min_x < wall.0 && wall.0 < max_x;
        let before_y = wall.1.0.max(wall.1.1) < min_y;
        let after_y = wall.1.0.min(wall.1.1) > max_y;
        between_x && !before_y && !after_y
    }) {
        return true;
    }

    false
}