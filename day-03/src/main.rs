use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part 1 solution: {}", calculate_power_consumption(&input));
    println!("part 2 solution: {}", calculate_life_support_rating(&input));
}

#[derive(Debug)]
enum DominantValue {
    Zero,
    One,
    Equal,
}

fn calculate_power_consumption(rows: &[Vec<u8>]) -> usize {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..rows[0].len() {
        match get_dominant_at_col(rows, i) {
            DominantValue::Zero => epsilon_rate |= 1 << (rows[0].len() - i - 1),
            _ => gamma_rate |= 1 << (rows[0].len() - i - 1),
        }
    }
    gamma_rate * epsilon_rate
}

fn get_dominant_at_col(values: &[Vec<u8>], pos: usize) -> DominantValue {
    let sum_ones = values.iter().filter(|row| row[pos] == 1).count();
    let m = (values.len() + 1) / 2;
    let is_even_sized = values.len() % 2 == 0;
    if is_even_sized && sum_ones == m {
        DominantValue::Equal
    } else if sum_ones >= m {
        DominantValue::One
    } else {
        DominantValue::Zero
    }
}

fn evaluate_row(values: &[u8]) -> usize {
    values
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, a)| acc | (*a as usize * (1 << i)))
}

fn calculate_life_support_rating(values: &[Vec<u8>]) -> usize {
    let oxygen_gen_rating = get_rating(values, |dom| !matches!(dom, DominantValue::Zero) as u8);
    let co2_scrubber_ratting = get_rating(values, |dom| matches!(dom, DominantValue::Zero) as u8);

    oxygen_gen_rating * co2_scrubber_ratting
}

fn get_rating<F>(values: &[Vec<u8>], mut filter_fn: F) -> usize
where
    F: FnMut(&DominantValue) -> u8 + Copy,
{
    let mut values = values.to_vec();
    for i in 0..values[0].len() {
        let dom = get_dominant_at_col(&values, i);
        values.retain(|row| row[i] == filter_fn(&dom));
        if values.len() == 1 {
            break;
        }
    }
    evaluate_row(&values[0])
}

fn read(filename: &str) -> Vec<Vec<u8>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_life_support_rating, calculate_power_consumption, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(calculate_power_consumption(&input), 198);
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        assert_eq!(calculate_life_support_rating(&input), 230);
    }
}
