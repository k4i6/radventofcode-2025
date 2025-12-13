use std::fs::read_to_string;


const FILE_PATH: &str = "presents.txt";
const PRESENT_COUNT: usize = 6;
const PRESENT_DIMENSION: usize = 3;

struct Grid {
    width: u32,
    heigth: u32,
    present_count: [u32; PRESENT_COUNT],
}

fn main() {
    let (present_weights, grids) = parse_presents();
    let count: u32 = grids.iter().map(|grid| {
        let area = grid.heigth * grid.width;
        let needed_area: u32 = grid.present_count.iter().enumerate()
            .map(|(index, count)| count * present_weights[index] as u32)
            .sum();
        return if needed_area <= area {
            1
        } else {
            0
        }
    })
    .sum();
    println!("count: {}", count);
}

fn parse_presents() -> ([u8;PRESENT_COUNT], Vec<Grid>) {
    let file = read_to_string(FILE_PATH).unwrap();
    let mut lines = file.lines();
    let mut present_weights: [u8;PRESENT_COUNT] = [0;PRESENT_COUNT];
    for i in 0..PRESENT_COUNT {
        lines.next().unwrap();
        let mut weight: u8 = 0;
        for _ in 0..PRESENT_DIMENSION {
            let line = lines.next().unwrap();
            weight += line.chars()
                .fold(0, |acc, char| match char {
                    '#' => acc + 1,
                    _ => acc,
                });
        }
        present_weights[i] = weight;
        lines.next();
    }

    let grids: Vec<Grid> = lines.map(|line| {
        let [dimension, present_counts] = line.split(":").collect::<Vec<&str>>().try_into().unwrap();
        let [width, height] = dimension.split("x").map(|num| num.parse().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
        let present_count: [u32; PRESENT_COUNT] = present_counts.trim().split(" ").map(|count| count.parse().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
        return Grid { width: width, heigth: height, present_count: present_count }
    })
    .collect();
    return (present_weights, grids);
}
