use std::{collections::HashSet, fs::read_to_string};

type Point = (usize, usize);
type FoldInstr = (char, usize);

fn main() {
    let (map, instructions) = read("input.txt").unwrap();
    println!("part1 solution: {}", fold(&map, instructions[0]).len());
    let after_all_folds = fold_all(&map, &instructions);
    println!("part2 solution:");
    print_map(&after_all_folds);
}

fn fold_all(map: &HashSet<Point>, instructions: &[FoldInstr]) -> HashSet<Point> {
    instructions
        .iter()
        .fold(map.clone(), |acc, &instr| fold(&acc, instr))
}

fn fold(map: &HashSet<Point>, (fold_by_axis, fold_position): FoldInstr) -> HashSet<Point> {
    map.iter()
        .filter_map(|&(curr_x, curr_y)| match fold_by_axis {
            'x' => match curr_x {
                x if x == fold_position => None,
                x if x < fold_position => Some((x, curr_y)),
                _ => Some((fold_position - (curr_x - fold_position), curr_y)),
            },
            _ => match curr_y {
                y if y == fold_position => None,
                y if y < fold_position => Some((curr_x, y)),
                _ => Some((curr_x, fold_position - (curr_y - fold_position))),
            },
        })
        .collect()
}

fn print_map(map: &HashSet<Point>) {
    let (width, height) = map
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));
    let s = (0..=height)
        .map(|y| {
            (0..=width)
                .map(|x| if map.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    println!("{}", s.join("\n"));
}

fn read(filename: &str) -> Option<(HashSet<Point>, Vec<FoldInstr>)> {
    read_to_string(filename)
        .expect("Failed to read file")
        .split_once("\n\n")
        .map(|(items, instructions)| {
            let map = items
                .lines()
                .filter_map(|l| {
                    l.split_once(',')
                        .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
                })
                .collect();
            let instr = instructions
                .lines()
                .filter_map(|l| {
                    l.strip_prefix("fold along ")?
                        .split_once('=')
                        .and_then(|(item, value)| Some((item.chars().next()?, value.parse().ok()?)))
                })
                .collect();
            (map, instr)
        })
}

#[cfg(test)]
mod tests {
    use crate::{fold, read};

    #[test]
    fn part1_test() {
        let (map, instructions) = read("test-input.txt").unwrap();
        assert_eq!(fold(&map, instructions[0]).len(), 17);
    }
}
