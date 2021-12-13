use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let (mut map, instructions) = read("input.txt").unwrap();
    println!(
        "part1 solution: {}",
        fold(&mut map.clone(), instructions[0])
    );
    fold_all(&mut map, &instructions);
    println!("part2 solution:");
    print_map(&map);
}

fn fold_all(map: &mut HashSet<(usize, usize)>, instructions: &[(char, usize)]) {
    for &(fold_by_axis, fold_position) in instructions {
        fold(map, ((fold_by_axis, fold_position)));
    }
}

fn fold(map: &mut HashSet<(usize, usize)>, (fold_by_axis, fold_position): (char, usize)) -> usize {
    *map = map
        .iter()
        .filter_map(|&(x, y)| match fold_by_axis {
            'y' => {
                if x == fold_position {
                    return None;
                } else if x < fold_position {
                    return Some((x, y));
                } else {
                    return Some((fold_position - (x - fold_position), y));
                }
            }
            _ => {
                if y == fold_position {
                    return None;
                } else if y < fold_position {
                    return Some((x, y));
                } else {
                    return Some((x, fold_position - (y - fold_position)));
                }
            }
        })
        .collect();
    map.len()
}

fn print_map(map: &HashSet<(usize, usize)>) {
    let (width, height) = map
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));
    let mut s = String::new();
    for x in 0..=width {
        for y in 0..=height {
            if map.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn read(filename: &str) -> Option<(HashSet<(usize, usize)>, Vec<(char, usize)>)> {
    let content = read_to_string(filename).expect("Failed to read file");
    let (items, instructions) = content.split_once("\n\n")?;
    let map = items
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(x, y)| Some((y.parse().unwrap(), x.parse().unwrap())))
                .unwrap()
                .unwrap()
        })
        .collect();
    let instr = instructions
        .lines()
        .map(|l| {
            l.strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .map(|(item, value)| (item.chars().next().unwrap(), value.parse().unwrap()))
                .unwrap()
        })
        .collect();
    Some((map, instr))
}

#[cfg(test)]
mod tests {
    use crate::{fold, read};

    #[test]
    fn part1_test() {
        let (mut map, instructions) = read("test-input.txt").unwrap();
        assert_eq!(fold(&mut map, instructions[0]), 17);
    }
}
