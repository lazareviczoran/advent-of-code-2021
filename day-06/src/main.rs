use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", count_lanternfish(&input, 80));
    println!("part2 solution: {}", count_lanternfish(&input, 256));
}

fn count_lanternfish(input: &[usize], period: usize) -> usize {
    let mut memo = HashMap::new();
    input
        .iter()
        .map(|&fish| 1 + get_number_of_fishes_for_single(fish, 0, period, &mut memo))
        .sum()
}

fn get_number_of_fishes_for_single(
    fish: usize,
    start: usize,
    period: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if start > period {
        return 0;
    }

    (start + fish + 1..=period)
        .step_by(7)
        .map(|i| {
            if let Some(&count) = memo.get(&(8, i)) {
                return count;
            }
            let res = 1 + get_number_of_fishes_for_single(8, i, period, memo);
            memo.insert((8, i), res);
            res
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
    use crate::{count_lanternfish, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(count_lanternfish(&input, 18), 26);
        assert_eq!(count_lanternfish(&input, 80), 5934);
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        assert_eq!(count_lanternfish(&input, 256), 26984457539);
    }
}
