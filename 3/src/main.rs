use std::{fs::File, io::{self, BufRead}};

const FILE_PATH: &str = "batterie_banks.txt";

fn main() {
    let banks = parse_batterie_banks();
    let mut sum: u128 = 0;
    let mut sum_2: u128 = 0;
    for bank in banks {
        sum += get_highest_number(&bank, 2);
        sum_2 += get_highest_number(&bank, 12);
    }
    println!("the sum is: {}", sum);
    println!("the sum 2 is: {}", sum_2);
}

fn get_highest_number(bank: &Vec<u8>, digit_count: usize) -> u128 {
    let mut number_string = "".to_string();
    let mut start_index = 0;
    for i in 0..digit_count {
        let (index, number) = bank[start_index..bank.len()-digit_count+1+i]
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap()).unwrap();
        start_index = index + start_index + 1;
        number_string += &number.to_string();
    }
    return number_string.parse().unwrap();
}

fn parse_batterie_banks() -> Vec<Vec<u8>> {
    let file = File::open(FILE_PATH).unwrap();
    let mut banks: Vec<Vec<u8>> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let mut bank: Vec<u8> = Vec::new();
        for c in line.unwrap().chars() {
            bank.push(c.to_string().parse().unwrap());
        }
        banks.push(bank);
    }
    return banks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hightest_number_2() {
        assert_eq!(98, get_highest_number(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1].to_vec(), 2));
        assert_eq!(89, get_highest_number(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9].to_vec(), 2));
    }

    #[test]
    fn hightest_number_12() {
        assert_eq!(987654321111, get_highest_number(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1].to_vec(), 12));
        assert_eq!(811111111119, get_highest_number(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9].to_vec(), 12));
    }
}