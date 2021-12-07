use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", calculate_min_fuel(&input));
}

fn calculate_min_fuel(positions: &[usize]) -> usize {
    let mut min_consumption = usize::MAX;
    for target_pos in 0..positions.len() {
        let fuel_consumption = calculate_fuel_consumption_for_target_pos(positions, &target_pos);
        if fuel_consumption < min_consumption {
            min_consumption = fuel_consumption;
        }
    }

    // println!("{:?}", most_common);
    min_consumption
}

fn calculate_fuel_consumption_for_target_pos(positions: &[usize], target: &usize) -> usize {
    positions
        .iter()
        .map(|pos| {
            if pos != target {
                if pos > target {
                    *pos - target
                } else {
                    target - *pos
                }
            } else {
                0
            }
        })
        .sum()
}

fn read(filename: &str) -> Vec<usize> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .next()
        .map(|l| {
            l.split_terminator(',')
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_min_fuel, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(calculate_min_fuel(&input), 37);
    }
}
