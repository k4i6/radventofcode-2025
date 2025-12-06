use std::fs::read_to_string;

const FILE_PATH: &str = "math.txt";

fn main() {
    let (operations, number_lines) = parse_math();
    let mut sheet_results: Vec<u128> = operations.iter().map(|operation| match operation {
        '*' => 1,
        '+' => 0,
        _ => panic!("unknown operator")
    }).collect();
    for number_line in &number_lines {
        number_line.split(' ').map(|slice| slice.trim()).filter(|slice| !slice.is_empty()).enumerate().for_each(|(index, number)| {
            let number: u128 = number.parse().unwrap();
            sheet_results[index] = match operations[index] {
                '*' => number * sheet_results[index],
                '+' => number + sheet_results[index],
                _ => panic!("unknown operator"),
            };
        });
    }
    let sum: u128 = sheet_results.iter().sum();
    println!("the result is: {}", sum);

    let numbers_2 = parse_columns(&number_lines);
    let sum_2: u128 = numbers_2.iter().enumerate().map(|(col_index, numbers)| {
        match operations[col_index] {
            '*' => numbers.iter().fold(1, |res, number| res * number),
            '+' => numbers.iter().fold(0, |res, number| res + number),
            _ => panic!("unknown operation")
        }
    }).sum();
    println!("the sum 2 is: {}", sum_2);
}

fn parse_columns(number_lines: &Vec<String>) -> Vec<Vec<u128>> {
    let mut result: Vec<Vec<u128>> = Vec::new();
    let number_lines: Vec<Vec<char>> = number_lines.iter().map(|line| line.chars().collect()).collect();
    let mut numbers: Vec<u128> = Vec::new();
    for i in 0..number_lines[0].len() {
        let number: Vec<char> = number_lines.iter().map(|line| line[i]).filter(|digit| *digit != ' ').collect();
        if number.is_empty() {
            result.push(numbers.clone());
            numbers.clear();
            continue;
        }
        numbers.push(number.iter().fold(String::new(), |string, character| string + &character.to_string()).parse().unwrap());
    }
    if !numbers.is_empty() {
        result.push(numbers);
    }
    return result;
}

fn parse_math() -> (Vec<char>, Vec<String>) {
    let sheet = read_to_string(FILE_PATH).unwrap();
    let mut lines = sheet.lines();
    let mut number_lines: Vec<String> = Vec::new();
    for _ in 0..lines.clone().count() - 1 {
        number_lines.push(lines.next().unwrap().to_string());
    }
    let operations: Vec<char> = lines.next().unwrap().chars().filter(|op| *op != ' ').collect();
    return (operations, number_lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_column() {
        assert_eq!(vec![vec![1,24, 356], vec![369, 248, 8], vec![32, 581, 175], vec![623, 431, 4]], parse_columns(&vec!["123 328  51 64 ".to_string(), " 45 64  387 23 ".to_string(), "  6 98  215 314".to_string()]));
    }
}