use AdventOfCode2025::solve;

#[derive(Debug, Clone)]
enum Symbol {
    Time,
    Plus,
}

impl Symbol {
    fn new(symbol: char) -> Self {
        match symbol {
            '*' => Symbol::Time,
            '+' => Symbol::Plus,
            x => panic!("Unknown symbol: {}", x),
        }
    }
}

type Parsed = (Vec<(Vec<Vec<char>>, Symbol)>);

fn main() {
    solve(6, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    let lines = file.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = Vec::new();

    let symbols = &lines[lines.len() - 1];

    let splits = symbols.iter().enumerate()
        .filter(|&(_, &s)| s == '*' || s == '+')
        .collect::<Vec<_>>();

    let mut iter = splits.into_iter();

    let mut last = iter.next().unwrap();

    for (i, c) in iter {
        let symbol = Symbol::new(*last.1);

        let mut vec = Vec::new();

        for line in &lines[..lines.len()-1] {
            vec.push(line[last.0..i].into_iter().cloned().collect());
        }

        result.push((vec, symbol));

        last = (i, c)
    }

    let symbol = Symbol::new(*last.1);

    let mut vec = Vec::new();

    for line in &lines[..lines.len()-1] {
        vec.push(line[last.0..].into_iter().cloned().collect());
    }

    result.push((vec, symbol));

    result
}

fn part1(input: Parsed) -> u128 {
    let mut total = 0;

    for (chs, symbol) in input {

        let nums = chs.into_iter()
            .map(|c| c.iter()
                .filter(|&&c| c != ' ')
                .collect::<String>()
                .parse::<u128>()
                .unwrap())
            .collect::<Vec<_>>();

        let sum = match symbol {
            Symbol::Plus => nums.iter().sum::<u128>(),
            Symbol::Time => nums.iter().product::<u128>(),
        };

        total += sum;
    }

    total
}

fn part2(input: Parsed) -> u128 {
    let mut total = 0;

    for (chs, symbol) in input {

        let mut nums = Vec::new();

        let len = chs.iter()
            .map(|cs| cs.len())
            .max()
            .unwrap();

        for i in 0..len {
            let str = chs.iter()
                .map(|c| c.get(i).unwrap_or(&' '))
                .filter(|&&s| s != ' ')
                .collect::<String>();

            if str.is_empty() {
                continue;
            }

            let num= str.parse::<u128>().unwrap();
            nums.push(num);
        }


        let sum = match symbol {
            Symbol::Plus => nums.iter().sum::<u128>(),
            Symbol::Time => nums.iter().product::<u128>(),
        };

        total += sum;
    }

    total
}