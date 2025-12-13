use std::{collections::HashMap, fs::read_to_string};

const FILE_PATH: &str = "devices.txt";

fn main() {
    let devices = parse_devices();
    let count = count_possible_ways("you", "out", &devices, &mut HashMap::new());
    println!("count: {}", count);

    let svr_to_fft = count_possible_ways("svr", "fft", &devices, &mut HashMap::new());
    let fft_to_dac = count_possible_ways("fft", "dac", &devices, &mut HashMap::new());
    let dac_to_out = count_possible_ways("dac", "out", &devices, &mut HashMap::new());
    let svr_to_dac = count_possible_ways("svr", "dac", &devices, &mut HashMap::new());
    let dac_to_fft = count_possible_ways("dac", "fft", &devices, &mut HashMap::new());
    let fft_to_out = count_possible_ways("fft", "out", &devices, &mut HashMap::new());
    let count2 = svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out;
    println!("{}", count2);
}

fn count_possible_ways(current: &str, target: &str, devices: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, u64>) -> u64 {
    if current == target {
        return 1;
    }
    if let Some(count) = cache.get(current) {
        return *count;
    }
    if !devices.contains_key(current) {
        return 0;
    }
    let mut count = 0;
    for next in devices[current].iter() {
        count += count_possible_ways(&next, target, devices, cache);
    }
    cache.insert(current.to_string(), count);
    return count;
}

fn parse_devices() -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for line in read_to_string(FILE_PATH).unwrap().lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let connections: Vec<String> = words[1..].iter().map(|conn| conn.to_string()).collect();
        result.insert(words[0].replace(":", ""), connections);
    }
    return result;
}
