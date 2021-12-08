use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", count_1_4_7_8_appearance(&input));
}

fn count_1_4_7_8_appearance(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|item| [2, 3, 4, 7].contains(&item.len()))
                .count()
        })
        .sum()
}

fn read(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter_map(|l| {
            let (from, to) = l.split_once('|')?;
            Some((
                from.split_whitespace()
                    .map(|item| item.to_string())
                    .collect(),
                to.split_whitespace().map(|item| item.to_string()).collect(),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_1_4_7_8_appearance, read};

    #[test]
    fn part1_test() {
        let input = read("test-input2.txt");
        assert_eq!(count_1_4_7_8_appearance(&input), 26);
    }
}
