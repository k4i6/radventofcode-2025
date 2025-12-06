use std::fs::read_to_string;

const FILE_PATH: &str = "ingredients.txt";

fn main() {
    let (ids, ranges) = parse_ingredients();
    let mut fresh_count = 0;
    for id in ids {
        if ranges.iter().filter(|(low, high)| id >= *low && id <= *high).next() != Option::None {
            fresh_count += 1;
        }
    }
    println!("fresh ingredients count: {}", fresh_count);

    let fresh_count_total = fresh_total_count(&ranges);
    println!("total fresh ingredients count: {}", fresh_count_total);
}

fn fresh_total_count(ranges: &Vec<(u128, u128)>) -> u128 {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|(low1, _), (low2, _)| low1.partial_cmp(low2).unwrap());
    let mut fresh_count_total = 0;
    let mut max_id = 0;
    for (range_low, range_high) in sorted_ranges {
        if range_high <= max_id {
            continue;
        }
        let low = if range_low <= max_id {
            max_id + 1
        } else {
            range_low
        };
        if range_high >= range_low {
            fresh_count_total += range_high - low + 1;
            max_id = range_high;
        }
    }
    return fresh_count_total
}

fn parse_ingredients() -> (Vec<u128>, Vec<(u128, u128)>) {
    let binding = read_to_string(FILE_PATH).unwrap();
    let mut lines = binding.lines();
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut ids: Vec<u128> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.trim().is_empty() {
            break;
        }
        let mut splitter = line.split("-");
        ranges.push((splitter.next().unwrap().parse().unwrap(), splitter.next().unwrap().parse().unwrap()));
    }
    for line in lines {
        ids.push(line.parse().unwrap());
    }
    return (ids, ranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_total_count() {
        assert_eq!(14, fresh_total_count(&vec![(3, 5), (10, 14), (16, 20), (12, 18)]));
        assert_eq!(14, fresh_total_count(&vec![(3, 5), (10, 14), (16, 20), (12, 18), (11, 18)]));
        assert_eq!(14, fresh_total_count(&vec![(3, 5), (10, 14), (16, 20), (12, 18), (11, 18), (3, 5)]));
        assert_eq!(15, fresh_total_count(&vec![(3, 5), (10, 14), (16, 20), (12, 18), (11, 18), (3, 6)]));
    }
}