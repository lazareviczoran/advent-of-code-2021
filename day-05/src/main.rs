use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", count_intersections(&input, false));
    println!("part2 solution: {}", count_intersections(&input, true));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn count_intersections(mappings: &[(Point, Point)], include_diagonals: bool) -> usize {
    get_marked_field(mappings, include_diagonals)
        .values()
        .filter(|v| **v > 1)
        .count()
}

fn get_marked_field(mappings: &[(Point, Point)], include_diagonals: bool) -> HashMap<Point, usize> {
    let is_diagonal = |a: &Point, b: &Point| (a.x - b.x).abs() == (a.y - b.y).abs();
    let is_hor_or_vert = |a: &Point, b: &Point| a.x == b.x || a.y == b.y;
    mappings
        .iter()
        .filter(|(a, b)| (include_diagonals && is_diagonal(a, b)) || is_hor_or_vert(a, b))
        .fold(HashMap::new(), mark_line)
}

fn mark_line(mut map: HashMap<Point, usize>, (from, to): &(Point, Point)) -> HashMap<Point, usize> {
    let step_value = |a: isize, b: isize| match a.cmp(&b) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        _ => 0,
    };
    let mut curr = *from;
    loop {
        *map.entry(curr).or_insert(0) += 1;
        if &curr == to {
            break;
        }
        curr.x += step_value(curr.x, to.x);
        curr.y += step_value(curr.y, to.y);
    }
    map
}

fn read(filename: &str) -> Vec<(Point, Point)> {
    let parse_fn = |point_str: &str| {
        point_str
            .split_once(",")
            .and_then(|(x, y)| Some(Point::new(x.parse().ok()?, y.parse().ok()?)))
    };
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter_map(|l| {
            l.split_once(" -> ")
                .and_then(|(from_str, to_str)| Some((parse_fn(from_str)?, parse_fn(to_str)?)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_intersections, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(count_intersections(&input, false), 5);
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        assert_eq!(count_intersections(&input, true), 12);
    }
}
