use good_lp::{highs, variable, variables, Expression, Solution, SolverModel};
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

impl Machine {
    fn fewest_presses_to_joltage_requirement(&self) -> usize {
        let mut problem_variables = variables!();
        let mut list = self.joltage_requirements.iter()
            .map(|_| Vec::new())
            .collect::<Vec<_>>();
        let variables = self.button_wiring.iter()
            .map(|buttons| {
                let variable = problem_variables.add(variable().min(0).integer());
                for &b in buttons {
                    list[b].push(variable);
                }
                variable
            }).collect::<Vec<_>>();

        let mut problem = highs(problem_variables.minimise(variables.iter().sum::<Expression>()));

        for (i, vars) in list.into_iter().enumerate() {
            let expression = vars.iter().sum::<Expression>();
            let value = self.joltage_requirements[i];
            problem.add_constraint(expression.eq(value as u32));
        }

        let solution = problem.solve().unwrap();

        let values = variables.iter().map(|&x| solution.value(x).round() as usize)
            .collect::<Vec<_>>();

        values.iter().sum()
    }
}

fn part2(input: Parsed) -> usize {
    input.into_iter()
        .map(|machine| machine.fewest_presses_to_joltage_requirement())
        .sum()
}