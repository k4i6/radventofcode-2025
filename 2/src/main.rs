use std::fs::read_to_string;

const FILE_PATH: &str = "id-ranges.txt";

fn main() {
    let id_ranges = parse_ranges();
    let (invalid_id_sum, invalid_id_sum_2) = invalid_id_sum(id_ranges);
    println!("invlid id sum: {}", invalid_id_sum);
    println!("invlid id sum 2: {}", invalid_id_sum_2);
}

fn invalid_id_sum(id_ranges: Vec<(u128, u128)>) -> (u128, u128) {
    let mut sum: u128 = 0;
    let mut sum2: u128 = 0;
    for range in id_ranges {
        let (start_id, end_id) = range;
        for id in start_id..=end_id {
            if is_invalid(id) {
                sum += id;
            }
            if is_invalid_2(id) {
                sum2 += id;
            }
        }
    }
    return (sum, sum2);
}

fn is_invalid(id: u128) -> bool {
    let id_string = id.to_string();
    let char_count = id_string.chars().count();
    if char_count % 2 != 0 {
        return false;
    }
    let middle_index = char_count / 2;
    return id_string[..middle_index] == id_string[middle_index..];
}

fn is_invalid_2(id: u128) -> bool {
    let id_string = id.to_string();
    let char_count = id_string.chars().count();
    for index in 1..=char_count/2 {
        if is_repeated_pattern(&id_string, &index) {
            return true;
        }
    }
    return false;
}

fn is_repeated_pattern(id: &str, end_index: &usize) -> bool {
    let char_count = id.chars().count();
    if char_count % end_index != 0 {
        return false;
    }
    for i in 0..(char_count / end_index) {
        if id[..*end_index] != id[(end_index*i)..(end_index*i+end_index)] {
            return false;
        }
    }
    return true;
}

fn parse_ranges() -> Vec<(u128, u128)> {
    let content = read_to_string(FILE_PATH).unwrap();
    return content.split(",").map(|slice| {
        let mut split = slice.split("-");
        let start: u128 = split.next().unwrap().parse().unwrap();
        let end: u128 = split.next().unwrap().parse().unwrap();
        return (start, end);
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_sum() {
        assert_eq!((222222, 222222), invalid_id_sum([(222220, 222224)].to_vec()));
        assert_eq!((0, 2121212121), invalid_id_sum([(2121212118, 2121212124)].to_vec()));
        assert_eq!((0, 565656), invalid_id_sum([(565653, 565659)].to_vec()));
        assert_eq!((38593859, 38593859), invalid_id_sum([(38593856, 38593862)].to_vec()));
    }
}
