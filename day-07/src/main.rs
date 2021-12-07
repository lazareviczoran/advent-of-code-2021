use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {:?}", calc_min_fuel_single_step(&input));
    println!("part2 solution: {:?}", calc_min_fuel_multi_step(&input));
}

fn calc_min_fuel_single_step(positions: &[isize]) -> Option<isize> {
    calc_min_fuel(positions, |x| x)
}

fn calc_min_fuel_multi_step(positions: &[isize]) -> Option<isize> {
    calc_min_fuel(positions, |x| (1..=x).sum())
}

fn calc_min_fuel(positions: &[isize], step_fn: impl Fn(isize) -> isize) -> Option<isize> {
    (0..positions.len() as isize)
        .map(|target_pos| calc_fuel_consumption_for_target_pos(positions, &target_pos, &step_fn))
        .min()
}

fn calc_fuel_consumption_for_target_pos(
    positions: &[isize],
    target: &isize,
    step_fn: impl Fn(isize) -> isize,
) -> isize {
    let calc_required_fuel = |pos: &isize, target: &isize| (pos - target).abs();
    positions
        .iter()
        .map(|pos| step_fn(calc_required_fuel(pos, target)))
        .sum()
}

fn read(filename: &str) -> Vec<isize> {
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
    use crate::{calc_min_fuel_multi_step, calc_min_fuel_single_step, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(calc_min_fuel_single_step(&input), Some(37));
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        assert_eq!(calc_min_fuel_multi_step(&input), Some(168));
    }
}
