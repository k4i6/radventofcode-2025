use std::fs::read_to_string;

const FILE_PATH: &str = "junction_boxes.txt";

fn main() {
    let coordinates = parse_coordinates();
    let pairs: Vec<(&[u32;3],&[u32;3])> = coordinates.iter()
        .enumerate()
        .flat_map(|(index,coordinate)| coordinates[index+1..].iter()
            .map(|coordinate2| (coordinate, coordinate2))
            .collect::<Vec<(&[u32;3],&[u32;3])>>())
        .collect();
    let mut distances: Vec<(usize, f64)> = pairs.iter()
        .map(|(coor1, coor2)| (coor1[0] as f64 - coor2[0] as f64).powf(2.0) + (coor1[1] as f64 - coor2[1] as f64).powf(2.0) + (coor1[2] as f64 - coor2[2] as f64).powf(2.0))
        .enumerate()
        .collect();
    distances.sort_by(|(_, distance1), (_, distance2)| distance1.partial_cmp(distance2).unwrap());

    let mut circuits: Vec<Vec<&[u32;3]>> = Vec::new();
    let _ = produce_circuits(&mut circuits, &distances[..1000], &pairs, coordinates.len());

    let mut circuit_lenghts: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_lenghts.sort();
    circuit_lenghts.reverse();
    let product: usize = circuit_lenghts.iter().take(3).product();
    println!("product: {}", product);

    let last_pair = produce_circuits(&mut circuits, &distances[1000..], &pairs, coordinates.len());
    println!("product of final pair x-coordinates: {}", last_pair.unwrap().0[0] as u128 * last_pair.unwrap().1[0] as u128);
}

fn produce_circuits<'a>(circuits: &mut Vec<Vec<&'a[u32;3]>>, distances: &[(usize, f64)], pairs: &Vec<(&'a[u32;3],&'a[u32;3])>, total_coordinates: usize) -> Option<([u32;3],[u32;3])> {
    for (index,_) in distances.iter() {
        let (coord1, coord2) = pairs[*index];
        let existing_circuit1 = circuits.iter().enumerate().find(|(_,circuit)| circuit.contains(&coord1));
        let existing_circuit2 = circuits.iter().enumerate().find(|(_,circuit)| circuit.contains(&coord2));
        if let Some((index1,circuit1)) = existing_circuit1 {
            if let Some((index2,circuit2)) = existing_circuit2 {
                if index1 == index2 {
                    continue;
                }
                let new_circuit: Vec<&[u32; 3]> = circuit1.iter().chain(circuit2.iter()).map(|circuit| *circuit).collect();
                if new_circuit.len() == total_coordinates {
                    return Some((*coord1, *coord2));
                }
                circuits.push(new_circuit);
                if index1 < index2 {
                    circuits.remove(index2);
                    circuits.remove(index1);
                } else {
                    circuits.remove(index1);
                    circuits.remove(index2);
                }
            } else {
                circuits[index1].push(coord2);
            }
        } else if let Some((index2,_)) = existing_circuit2 {
            circuits[index2].push(coord1);
        } else {
            circuits.push(vec![coord1, coord2]);
        }
    }
    return Option::None;
}

fn parse_coordinates() -> Vec<[u32; 3]> {
    return read_to_string(FILE_PATH).unwrap()
        .lines()
        .map(|line| line.split(",").map(|number| number.parse().unwrap()).collect::<Vec<u32>>().try_into().unwrap())
        .collect();
}