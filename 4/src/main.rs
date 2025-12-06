use std::fs::read_to_string;

const FILE_PATH: &str = "paper_roll_grid.txt";

fn main() {
    let mut paper_rolls = parse_paper_rolls();
    let accessible_roll_count = get_accessible_rolls(&paper_rolls).len();
    let mut accessible_roll_count_with_removing = 0;
    loop {
        let accessible_rolls = get_accessible_rolls(&paper_rolls);
        if accessible_rolls.is_empty() {
            break;
        }
        accessible_roll_count_with_removing += accessible_rolls.len();
        for (x, y) in accessible_rolls {
            paper_rolls[y][x] = '.';
        }
    }
    println!("accessible rolls: {}", accessible_roll_count);
    println!("accessible rolls with removing: {}", accessible_roll_count_with_removing);
}

fn get_accessible_rolls(lines: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible_rolls: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if *character != '@' {
                continue;
            }
            let neighbours = get_neighbours(x, y, line.len() - 1, lines.len() - 1);
            let count = neighbours.iter()
                .map(|(x2, y2)| lines[*y2][*x2])
                .filter(|character| *character == '@')
                .count();
            if count < 4 {
                accessible_rolls.push((x, y));
            }
        }
    }
    return accessible_rolls;
}

fn get_neighbours(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let mutations: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    for (x_mut, y_mut) in mutations {
        let xn = x as i32 + x_mut;
        let yn = y as i32 + y_mut;
        if xn < 0 || xn > x_max as i32 {
            continue;
        }
        if yn < 0 || yn > y_max as i32 {
            continue;
        }
        neighbours.push((xn as usize, yn as usize));
    }
    return neighbours;
}

fn parse_paper_rolls() -> Vec<Vec<char>> {
    return read_to_string(FILE_PATH).unwrap().lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();
}
