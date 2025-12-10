use std::collections::VecDeque;
use AdventOfCode2025::solve;

type Parsed = Vec<Machine>;

#[derive(Debug, Clone)]
struct Machine {
    indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn parse(line: &str) -> Machine {
        let mut split = line.split("] (");
        let indicator_lights = split.next().unwrap()
            .chars()
            .filter(|&c| c != '[')
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        let mut split = split.next().unwrap().split(") {");

        let button_wiring = split.next().unwrap()
            .split(") (")
            .map(|pair| pair.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let joltage_requirements = split.next().unwrap()
            .replace("}", "")
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Machine {
            indicator_lights,
            button_wiring,
            joltage_requirements,
        }
    }
}

fn main() {
    solve(10, parse, part1, part2);
}

fn parse(file: &str) -> Parsed {
    file.lines().map(Machine::parse).collect()
}

impl Machine {
    fn fewest_presses_to_turn_on_lights(&self) -> usize {
        let mut queue = VecDeque::new();
        let intial = self.indicator_lights.clone()
            .into_iter()
            .map(|_| false)
            .collect::<Vec<_>>();
        queue.push_back((0, intial));

        while let Some((depth, lights)) = queue.pop_front() {
            if lights == self.indicator_lights {
                return depth;
            }

            for buttons in &self.button_wiring {
                let mut clone = lights.clone();
                for &b in buttons {
                    clone[b] = !clone[b];
                }
                queue.push_back((depth + 1, clone));
            }
        }

        panic!("No solution to fewest presses found");
    }
}

fn part1(input: Parsed) -> usize {
    input.into_iter()
        .map(|machine| machine.fewest_presses_to_turn_on_lights())
        .sum()
}

fn part2(input: Parsed) -> usize {
    0
}