use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

const FILE_PATH: &str = "tachyon_diagram.txt";

fn main() {
    let (start_index, diagram) = parse_diagram();
    let mut beam_queue: VecDeque<(usize, usize)> = VecDeque::from([(start_index, 0)]);
    let mut splits: HashSet<(usize, usize)> = HashSet::new();
    let mut time_lines: HashMap<(usize, usize), u128> = HashMap::new();
    time_lines.insert((start_index, 0), 1);
    while let Some((x, y)) = beam_queue.pop_back() {
        if y + 1 >= diagram.len() {
            continue;
        }
        let time_line_count = if let Some(count) = time_lines.remove(&(x, y)) {
            count
        } else {
            continue;
        };
        if diagram[y][x] == '.' {
            let new_beam = (x, y+1);
            beam_queue.push_front(new_beam);
            time_lines.entry(new_beam).and_modify(|count| *count += time_line_count).or_insert(time_line_count);
            continue;
        }
        let time_lines_handle = &mut time_lines;
        for new_beam in [(x-1, y+1), (x+1, y+1)] {
            if !beam_queue.contains(&new_beam) {
                beam_queue.push_front(new_beam);
            }
            time_lines_handle.entry(new_beam).and_modify(|count| *count += time_line_count).or_insert(time_line_count);
        }
        splits.insert((x,y));
    }
    println!("split count: {}", splits.len());
    println!("time lines: {}", time_lines.values().sum::<u128>());
}

fn parse_diagram() -> (usize, Vec<Vec<char>>) {
    let mut diagram: Vec<Vec<char>> = read_to_string(FILE_PATH).unwrap().lines().map(|line| line.chars().collect()).collect();
    let start_index = diagram[0].iter().enumerate().find(|(_,character)| **character == 'S').unwrap().0;
    diagram.remove(0);
    return (start_index, diagram)
}
