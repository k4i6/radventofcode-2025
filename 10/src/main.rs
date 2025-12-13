use std::{fs::read_to_string, ops::BitXor};

use good_lp::{Expression, default_solver, Solution, SolverModel, Variable, constraint::eq, variable, variables};


const FILE_PATH: &str = "machines.txt";

struct Machine {
    desired_state: u16,
    buttons: Vec<u16>,
    button_numbers: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

fn main() {
    let machines = parse_machines();
    let counts: Vec<u32> = machines.iter()
        .map(|machine| btn_press_count(vec![0], machine, 0))
        .collect();
    let total_count: u32 = counts.iter().sum();
    println!("total btn press count: {}", total_count);

    // solves LP's using CBC solver
    // requires CBC library to be installed: https://github.com/rust-or/good_lp?tab=readme-ov-file#cbc
    let counts: Vec<u32> = machines.iter()
        .map(|machine| {
            let mut vars = variables!();
            let var_defintions: Vec<Variable> = machine.button_numbers.iter()
                .map(|_| vars.add(variable().integer().min(0)))
                .collect();
            let objective: Expression = var_defintions.iter()
                .fold(0.into(), |mut acc,var| {
                    acc += var;
                    return acc;
                });
            let mut problem = vars.minimise(objective).using(default_solver);
            machine.joltages.iter().enumerate().for_each(|(index, joltage)| {
                let sum: Expression = machine.button_numbers.iter().enumerate()
                    .filter(|(_, btn)| btn.contains(&(index as u32)))
                    .fold(0.into(), |mut acc, (btn_index, _)| {
                        acc += var_defintions[btn_index];
                        return acc;
                    });
                problem = problem.clone().with(eq(sum, *joltage));
            });
            let solution = problem.solve().unwrap();
            return var_defintions.iter().map(|var| solution.value(*var) as u32).sum()
        })
        .collect();
    let total_count: u32 = counts.iter().sum();
    println!("total btn press count for joltages: {}", total_count);
}

fn btn_press_count(states: Vec<u16>, machine: &Machine, count: u32) -> u32 {
    if let Some(_) = states.iter().find(|state| **state == machine.desired_state) {
        return count;
    }
    let new_states: Vec<u16> = states.iter()
        .flat_map(|state| machine.buttons.iter()
            .map(|btn| state.bitxor(btn))
        )
        .collect();
    return btn_press_count(new_states, machine, count + 1);
}

fn parse_machines() -> Vec<Machine> {
    return read_to_string(FILE_PATH).unwrap().lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let desired_state: u16 = parts[0].chars().skip(1)
                .take_while(|char| *char != ']')
                .enumerate()
                .fold(0, |acc, (index, char)| match char {
                    '#' => acc + (2 as u16).pow(index as u32),
                    _ => acc
                });
            let buttons: Vec<u16> = parts[1..parts.len()-1].iter()
                .map(|btn| btn.replace("(","").replace(")", "").split(",")
                    .fold(0, |acc, number| acc + (2 as u16).pow(number.parse().unwrap()))
                )
                .collect();
            let mut button_numbers: Vec<Vec<u32>> = parts[1..parts.len()-1].iter()
                .map(|btn| btn.replace("(","").replace(")", "").split(",")
                    .map(|number| number.parse().unwrap())
                    .collect()
                )
                .collect();
            button_numbers.sort_by(|btn1, btn2| btn1.len().partial_cmp(&btn2.len()).unwrap());
            button_numbers.reverse();
            let joltages: Vec<u32> = parts.last().unwrap().split(",")
                .map(|joltage| joltage.replace("{", "").replace("}", ""))
                .map(|joltage| joltage.parse().unwrap())
                .collect();
            return Machine { desired_state: desired_state, buttons: buttons, button_numbers: button_numbers, joltages: joltages }
        })
        .collect();
}