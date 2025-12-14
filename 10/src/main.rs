use std::{cmp::min, fs::read_to_string, ops::BitXor};

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


    // felt a bit like cheating using a solver and external lib...
    // ... second attempt for part two, without any additional libs
    let counts: Vec<u32> = machines.iter()
        .map(|machine| {
            let mut coefficient_matrix: Vec<Vec<f64>> = machine.joltages.iter().enumerate()
                .map(|(joltage_index, _)| machine.button_numbers.iter()
                    .map(|btn| if btn.contains(&(joltage_index as u32)){
                            1.0
                        } else {
                            0.0
                        }
                    )
                    .collect()
                )
                .collect();
            let mut target_vec = machine.joltages.iter().map(|joltage| *joltage as f64).collect();
            into_row_echelon_form(&mut coefficient_matrix, &mut target_vec);
            let mut ordered_btns = machine.button_numbers.clone();
            ordered_btns.sort_by(|btn1, btn2| btn2.len().partial_cmp(&btn1.len()).unwrap());
            let max_joltage = machine.joltages.iter().max().unwrap();
            let start_min: u32 = machine.joltages.iter().sum();
            let min = find_min(&coefficient_matrix, 0, &target_vec, start_min, *max_joltage, coefficient_matrix.first().unwrap().iter().map(|_| Option::None).collect());
            return min;
        })
        .collect();
    let total_count: u32 = counts.iter().sum();
    println!("total btn press count for joltages: {}", total_count);

}

fn into_row_echelon_form (coefficient_matrix: &mut Vec<Vec<f64>>, target_vec: &mut Vec<f64>) {
    let mut row: usize = 0;
    for col in 0..coefficient_matrix.first().unwrap().len() {
        let row_to_swap = match coefficient_matrix.iter().enumerate()
            .skip(row)
            .find(|(_, row)| row[col].abs() > 0.001) {
                Some((row, _)) => row,
                None => continue,
        };
        if row_to_swap != row {
            let temp_row = coefficient_matrix[row].clone();
            coefficient_matrix[row] = coefficient_matrix[row_to_swap].clone();
            coefficient_matrix[row_to_swap] = temp_row;

            let temp_val = target_vec[row];
            target_vec[row] = target_vec[row_to_swap];
            target_vec[row_to_swap] = temp_val;
        }
        let divisor = coefficient_matrix[row][col];
        for col2 in col..coefficient_matrix.first().unwrap().len() {
            coefficient_matrix[row][col2] /= divisor;
        }
        target_vec[row] /= divisor;
        for row2 in row+1..coefficient_matrix.len() {
            if coefficient_matrix[row2][col] == 0.0 {
                continue;
            }
            let factor =  -coefficient_matrix[row2][col];
            for col in col..coefficient_matrix.first().unwrap().len() {
                coefficient_matrix[row2][col] += coefficient_matrix[row][col] * factor;
            }
            assert!(coefficient_matrix[row2][col] == 0.0);
            target_vec[row2] += target_vec[row] * factor;
        }
        row += 1;
    }
    coefficient_matrix.reverse();
    target_vec.reverse();
}

fn find_min(coefficient_matrix: &Vec<Vec<f64>>, row: usize, target_vec: &Vec<f64>, current_min: u32, max_joltage: u32, mut config: Vec<Option<u32>>) -> u32 {
    if row == coefficient_matrix.len() {
        let min = min(current_min, config.iter().map(|val| val.unwrap()).sum());
        return min;
    }
    if config.iter().map(|val| val.unwrap_or(0)).sum::<u32>() >= current_min {
        return current_min;
    }
    let next_row = row + 1;
    if !coefficient_matrix[row].iter().any(|val| *val != 0.0) {
        return find_min(coefficient_matrix, next_row, target_vec, current_min, max_joltage, config);
    }
    let known_vars: Vec<usize> = config.iter().enumerate()
        .filter(|(index, _)| coefficient_matrix[row][*index] != 0.0)
        .filter(|(_, val)| **val != Option::None)
        .map(|(index,_)| index)
        .collect();
    let unknown_vars: Vec<usize> = config.iter().enumerate()
        .filter(|(index, _)| coefficient_matrix[row][*index] != 0.0)
        .filter(|(_, val)| **val == Option::None)
        .map(|(index,_)| index)
        .collect();
    if unknown_vars.is_empty() {
        if (target_vec[row] - known_vars.iter().map(|index| (config[*index].unwrap() as f64) * coefficient_matrix[row][*index]).sum::<f64>()).abs() > 0.001 {
            return current_min;
        }
        return find_min(coefficient_matrix, next_row, target_vec, current_min, max_joltage, config);
    }
    let free_vars: Vec<usize> = unknown_vars.iter().skip(1).map(|index| *index).collect();
    let next_defined_var = *unknown_vars.first().unwrap();
    assert!(coefficient_matrix[row][next_defined_var] == 1.0);
    if free_vars.len() == 0 {
        let next_defined_var_val: f64 = target_vec[row] -
            known_vars.iter().map(|index| (config[*index].unwrap() as f64) * coefficient_matrix[row][*index]).sum::<f64>();
        if next_defined_var_val < -0.001 {
            return current_min;
        }
        if (next_defined_var_val % 1.0).abs() > 0.001 && (1.0 - (next_defined_var_val % 1.0)).abs() > 0.001 {
            return current_min;
        }
        config[next_defined_var] = Some(next_defined_var_val.round() as u32);
        return find_min(coefficient_matrix, next_row, target_vec, current_min, max_joltage, config);
    }
    let combinations = all_combinations(&free_vars, max_joltage, Vec::new());
    return combinations.iter()
        .fold(current_min, |acc, combination| {
            let mut new_config = config.clone();
            let next_defined_var_val: f64 = target_vec[row] -
                combination.iter().map(|(index,val)| (*val as f64) * coefficient_matrix[row][*index]).sum::<f64>() -
                known_vars.iter().map(|index| (config[*index].unwrap() as f64) * coefficient_matrix[row][*index]).sum::<f64>();
            if next_defined_var_val < -0.001 {
                return acc;
            }
            if (next_defined_var_val % 1.0).abs() > 0.001 && (1.0 - (next_defined_var_val % 1.0)).abs() > 0.001 {
                return acc;
            }
            new_config[next_defined_var] = Some(next_defined_var_val.round() as u32);
            for (index, val) in combination {
                new_config[*index] = Some(*val);
            }
            return find_min(coefficient_matrix, next_row, target_vec, acc, max_joltage, new_config);
        })
}

fn all_combinations(free_vars: &Vec<usize>, max: u32, acc: Vec<(usize, u32)>) -> Vec<Vec<(usize, u32)>> {
    if free_vars.is_empty() {
        return vec![acc];
    }
    let mut result: Vec<Vec<(usize, u32)>> = Vec::new();
    let mut free_vars = free_vars.clone();
    let current_var = free_vars.pop().unwrap();
    for val in 0..=max {
        let mut new_acc = acc.clone();
        new_acc.push((current_var, val));
        result.append(&mut all_combinations(&free_vars.clone(), max, new_acc));
    }

    return result;
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