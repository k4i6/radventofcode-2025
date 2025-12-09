use std::{cmp::{max, min}, fs::read_to_string};

const FILE_PATH: &str = "red_tiles.txt";

type Point = (u32, u32);
type Edge<'a> = (&'a Point, &'a Point);

fn main() {
    let red_tiles = parse_coordinates();
    let mut areas: Vec<u128> = red_tiles.iter().enumerate()
        .flat_map(|(index, (x, y))| red_tiles[index+1..].iter()
            .map(|(x2, y2)| (max(x.clone(), *x2) - min(x.clone(), *x2) + 1) as u128 * (max(y.clone(),*y2) - min(y.clone(),*y2) + 1) as u128))
        .collect();
    areas.sort();
    let biggest_area = areas.last().unwrap();
    println!("biggest area: {}", biggest_area);

    let lines: Vec<Edge> = red_tiles.iter().enumerate()
        .map(|(index, tile)| {
            let tile2 = if index + 1 == red_tiles.len() {
                &red_tiles[0]
            } else {
                &red_tiles[index + 1]
            };
            (tile,tile2)
        })
        .collect();
    let mut green_red_areas: Vec<u128> = red_tiles.iter().enumerate()
        .flat_map(|(index, (x, y))| red_tiles[index+1..].iter()
            .filter(|(x2, y2)|
                is_inside_bounds((*x2, *y), &lines) &&
                is_inside_bounds((*x, *y2), &lines) &&
                !has_any_intersection((&(*x,*y), &(*x2, *y)), &lines) &&
                !has_any_intersection((&(*x,*y), &(*x, *y2)), &lines) &&
                !has_any_intersection((&(*x2,*y2), &(*x2, *y)), &lines) &&
                !has_any_intersection((&(*x2,*y2), &(*x, *y2)), &lines)
            )
            .map(|(x2, y2)| (max(x.clone(), *x2) - min(x.clone(), *x2) + 1) as u128 * (max(y.clone(),*y2) - min(y.clone(),*y2) + 1) as u128))
        .collect();
    green_red_areas.sort();
    let biggest_green_red_area = green_red_areas.last().unwrap();
    println!("bigggest green/red area: {}", biggest_green_red_area);
}

fn has_any_intersection(line: Edge, lines: &Vec<Edge>) -> bool {
    for line2 in lines {
        if has_intersection(line, *line2) {
            return true;
        }
    }
    return false;
}

fn has_intersection(line1: Edge, line2: Edge) -> bool {
    let (p1,p2) = line1;
    let (p3, p4) = line2;
    let o1 = orientation_diff(p1, p2, p3);
    let o2 = orientation_diff(p1, p2, p4);
    let o3 = orientation_diff(p3, p4, p1);
    let o4 = orientation_diff(p3, p4, p2);
    return o1 * o2 < 0.0 && o3 * o4 < 0.0
}

fn orientation_diff(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    return (p2.0 as f64 - p1.0 as f64)*(p3.1 as f64-p1.1 as f64) - (p2.1 as f64-p1.1 as f64)*(p3.0 as f64-p1.0 as f64)
}

fn is_inside_bounds(point1: Point, lines: &Vec<Edge>) -> bool {
    let (x, y) = point1;
    let lies_on = lines.iter()
        .find(|((x1, y1), (x2, y2))| x <= max(*x1, *x2) && x >= min(*x1, *x2) && y <= max(*y1, *y2) && y >= min(*y1, *y2));
    if let Some(_) = lies_on {
        return true;
    }
    let crossings = lines.iter()
        .filter(|((_, y1), (_, y2))| y1 != y2)
        .filter(|((_, y1), (_, y2))| y > min(*y1, *y2) && y <= max(*y1, *y2))
        .map(|((x1, y1), (x2, y2))|  *x1 as f64 + (y as f64 - *y1 as f64)*(*x2 as f64-*x1 as f64)/(*y2 as f64-*y1 as f64))
        .filter(|x1| *x1 > x as f64)
        .count();
    return crossings % 2 == 1;
}

fn parse_coordinates() -> Vec<Point> {
    return read_to_string(FILE_PATH).unwrap().lines()
        .map(|line| {
            let mut split = line.split(",");
            return (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
        })
        .collect();
}
