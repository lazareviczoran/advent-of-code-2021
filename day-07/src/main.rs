use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {:?}", calc_min_fuel_single_step(&input));
    println!("part2 solution: {:?}", calc_min_fuel_multi_step(&input));
}

fn calc_min_fuel_single_step(positions: &[usize]) -> Option<usize> {
    calc_min_fuel(positions, |x| x)
}

fn calc_min_fuel_multi_step(positions: &[usize]) -> Option<usize> {
    calc_min_fuel(positions, |x| (1..=x).sum())
}

fn calc_min_fuel<F>(positions: &[usize], step_fn: F) -> Option<usize>
where
    F: FnMut(usize) -> usize + Copy,
{
    (0..positions.len())
        .map(|target_pos| calc_fuel_consumption_for_target_pos(positions, &target_pos, step_fn))
        .min()
}

fn calc_fuel_consumption_for_target_pos<F>(
    positions: &[usize],
    target: &usize,
    mut step_fn: F,
) -> usize
where
    F: FnMut(usize) -> usize + Copy,
{
    let calc_required_fuel = |pos: &usize, target: &usize| pos.max(target) - pos.min(target);
    positions
        .iter()
        .map(|pos| step_fn(calc_required_fuel(pos, target)))
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
