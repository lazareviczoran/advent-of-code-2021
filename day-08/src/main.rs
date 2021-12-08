use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", count_1_4_7_8_appearance(&input));
    println!("part2 solution: {:?}", sum_all_values(&input));
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

/// digit mappings is a usize -> usize mapping where the input value is the value
/// of the digital number represented by a binary string following the indices of
/// the it's segments as shown in the ascii image bellow.
/// If the segment is active, the char on index i is 1, otherwise it is 0
///
/// segments:     indices:
///  aaaa            0
/// b    c        1     2
/// b    c
///  dddd   --->     3
/// e    f        4     5
/// e    f
///  gggg            6
///
/// for example, number 7 maps to "1010010"
///
///  aaaa            1
///      b        0     1
///      b
///         --->     0      --->    1010010
///      c        0     1
///      c
///                  0
///
fn get_digit_mappings() -> Option<HashMap<usize, usize>> {
    Some(HashMap::from([
        (usize::from_str_radix("1110111", 2).ok()?, 0),
        (usize::from_str_radix("0010010", 2).ok()?, 1),
        (usize::from_str_radix("1011101", 2).ok()?, 2),
        (usize::from_str_radix("1011011", 2).ok()?, 3),
        (usize::from_str_radix("0111010", 2).ok()?, 4),
        (usize::from_str_radix("1101011", 2).ok()?, 5),
        (usize::from_str_radix("1101111", 2).ok()?, 6),
        (usize::from_str_radix("1010010", 2).ok()?, 7),
        (usize::from_str_radix("1111111", 2).ok()?, 8),
        (usize::from_str_radix("1111011", 2).ok()?, 9),
    ]))
}

fn sum_all_values(input: &[(Vec<String>, Vec<String>)]) -> Option<usize> {
    let digit_mappings = get_digit_mappings()?;
    Some(
        input
            .iter()
            .filter_map(|line| get_value_for_mapping(line, &digit_mappings))
            .sum(),
    )
}

fn get_value_for_mapping(
    line: &(Vec<String>, Vec<String>),
    digit_mappings: &HashMap<usize, usize>,
) -> Option<usize> {
    let (mut input, output) = line.clone();
    input.sort_by_key(|a| a.len());

    let mut segment_mappings = HashMap::new();
    let chars_per_item = input
        .iter()
        .map(|item| item.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    segment_mappings.insert(0, &chars_per_item[1] ^ &chars_per_item[0]);
    segment_mappings.insert(
        3,
        &(&(&chars_per_item[3] & &chars_per_item[4]) & &chars_per_item[5]) & &chars_per_item[2],
    );

    segment_mappings.insert(
        1,
        &(&chars_per_item[0] ^ &chars_per_item[2]) ^ segment_mappings.get(&3)?,
    );
    segment_mappings.insert(
        4,
        &((&(&chars_per_item[3] & &chars_per_item[4]) & &chars_per_item[5])
            .union(&chars_per_item[2])
            .copied()
            .collect::<HashSet<_>>())
            ^ &chars_per_item[9],
    );
    segment_mappings.insert(
        6,
        &(&(chars_per_item[1]
            .union(&chars_per_item[2])
            .copied()
            .collect::<HashSet<_>>())
            ^ &chars_per_item[9])
            ^ segment_mappings.get(&4)?,
    );

    segment_mappings.insert(
        5,
        &(&(&chars_per_item[6] ^ &chars_per_item[8]) ^ &chars_per_item[7]) & &chars_per_item[0],
    );

    segment_mappings.insert(2, &chars_per_item[0] ^ segment_mappings.get(&5)?);

    let char2digit_mappings = segment_mappings
        .iter()
        .filter_map(|(&pos, char_set)| Some((*char_set.iter().next()?, pos)))
        .collect::<HashMap<_, _>>();
    Some(
        output
            .iter()
            .rev()
            .filter_map(|item| {
                digit_mappings.get(&item.chars().fold(0, |acc, ch| {
                    acc | (1 << (6 - char2digit_mappings.get(&ch).unwrap()))
                }))
            })
            .enumerate()
            .fold(0, |acc, (pos, value)| acc + 10usize.pow(pos as u32) * value),
    )
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
    use crate::{
        count_1_4_7_8_appearance, get_digit_mappings, get_value_for_mapping, read, sum_all_values,
    };

    #[test]
    fn part1_test() {
        let input = read("test-input2.txt");
        assert_eq!(count_1_4_7_8_appearance(&input), 26);
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        assert_eq!(
            get_value_for_mapping(&input[0], &get_digit_mappings().unwrap()),
            Some(5353)
        );
    }

    #[test]
    fn part2_test2() {
        let input = read("test-input2.txt");
        assert_eq!(sum_all_values(&input), Some(61229));
    }
}
