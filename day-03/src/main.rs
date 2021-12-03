use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!(
        "part 1 solution: {}",
        get_most_and_least_significant(&input)
    );
}

fn read(filename: &str) -> Vec<Vec<u8>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

fn get_most_and_least_significant(chars: &[Vec<u8>]) -> usize {
    let mut most_sign = 0;
    let mut least_sign = 0;
    let mut most: Vec<u8> = vec![];
    let mut least: Vec<u8> = vec![];
    for i in 0..chars[0].len() {
        let mut count0 = 0;
        let mut count1 = 0;
        for j in 0..chars.len() {
            match chars[j][i] {
                0 => count0 += 1,
                _ => count1 += 1,
            }
        }
        if count0 > count1 {
            println!("least before {}, c0 {}", least_sign, least_sign);
            least_sign |= 1 << (chars[0].len() - i - 1);
            most.push(0);
            least.push(1);
            println!("least after {}, c0 {}", least_sign, least_sign);
        } else {
            most_sign |= 1 << (chars[0].len() - i - 1);
            least.push(0);
            most.push(1);
            println!("shifting most c1 {}, c0 {}", most_sign, least_sign);
        }
    }

    println!("most {:?}, least {:?}", most, least);
    most_sign * least_sign
}

#[cfg(test)]
mod tests {
    use crate::{get_most_and_least_significant, read};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        assert_eq!(get_most_and_least_significant(&input), 198);
    }
}
