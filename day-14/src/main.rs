use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let (formula, rules) = read("input.txt");
    // let (formula, rules) = read("input.txt");
    println!(
        "part1 solution: {}",
        find_polymerization_score(&mut formula.clone(), &rules, 10)
    );
}

fn find_polymerization_score(
    formula: &mut Vec<char>,
    rules: &HashMap<String, char>,
    steps: usize,
) -> usize {
    for step in 1..=steps {
        // println!("running step {}", step);
        apply_transformation(formula, rules);
        // println!("formula is now {:?}", formula);
    }
    let items = formula.iter().fold(HashMap::new(), |mut acc, &ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    });
    // println!("{:?}", items);
    get_score(&items)
}

fn apply_transformation(formula: &mut Vec<char>, rules: &HashMap<String, char>) {
    let mut i = 0;
    while i < formula.len() - 1 {
        let current_pair: String = formula[i..=i + 1].iter().copied().collect();
        // println!(
        //     "i {}, current pair {}, current_formula {:?}",
        //     i, current_pair, formula
        // );
        if let Some(&ch) = rules.get(&current_pair) {
            // println!("inserting {} to pos {}", ch, i + 1);
            formula.insert(i + 1, ch);
            i += 2;
        } else {
            i += 1;
        }
        // println!("i {}, len {}, formula {:?}", i, formula.len(), formula);
    }
}

fn get_score(items: &HashMap<char, usize>) -> usize {
    let (least_count, most_count) = items
        .iter()
        .fold((usize::MAX, 0), |acc, (_current, &count)| {
            (acc.0.min(count), acc.1.max(count))
        });
    most_count - least_count
}

fn read(filename: &str) -> (Vec<char>, HashMap<String, char>) {
    let content = read_to_string(filename).expect("Failed to read file");
    let mut lines = content.lines();
    let formula = lines.next().unwrap().chars().collect();
    let rules = lines
        .skip(1)
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            (from.to_string(), to.chars().next().unwrap())
        })
        .collect();
    (formula, rules)
}

#[cfg(test)]
mod tests {
    use crate::{find_polymerization_score, read};

    #[test]
    fn test1() {
        let (mut formula, rules) = read("test-input.txt");
        assert_eq!(find_polymerization_score(&mut formula, &rules, 10), 1588);
    }
}
