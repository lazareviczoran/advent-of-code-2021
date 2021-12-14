use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let (formula, rules) = read("input.txt");
    println!(
        "part1 solution: {}",
        find_polymerization_score(&formula, &rules, 10)
    );
    println!(
        "part2 solution: {}",
        find_polymerization_score(&formula, &rules, 40)
    );
}

fn find_polymerization_score(
    formula: &[char],
    rules: &HashMap<(char, char), char>,
    steps: usize,
) -> usize {
    let mut memo = HashMap::new();
    let mut item_counts = HashMap::new();

    let mut iter = formula.windows(2).peekable();
    while let Some(items) = iter.next() {
        let partial = find_polymerization_score_rec((items[0], items[1]), rules, steps, &mut memo);
        partial.iter().for_each(|(&k, &v)| {
            *item_counts.entry(k).or_insert(0) += v;
        });
        if iter.peek().is_some() {
            // don't count end char twice
            *item_counts.entry(items[1]).or_insert(0) -= 1;
        }
    }

    get_score(&item_counts)
}

fn find_polymerization_score_rec(
    formula: (char, char),
    rules: &HashMap<(char, char), char>,
    steps: usize,
    memo: &mut HashMap<((char, char), usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if let Some(items) = memo.get(&(formula, steps)) {
        return items.clone();
    }
    if steps == 1 {
        let mut items = HashMap::new();
        if let Some(&ch) = rules.get(&formula) {
            *items.entry(formula.0).or_insert(0) += 1;
            *items.entry(ch).or_insert(0) += 1;
            *items.entry(formula.1).or_insert(0) += 1;
        }
        memo.insert((formula, steps), items.clone());
        return items;
    }
    let mut items = HashMap::new();
    if let Some(ch) = rules.get(&formula) {
        let partial1 = find_polymerization_score_rec((formula.0, *ch), rules, steps - 1, memo);
        partial1
            .iter()
            .for_each(|(&k, &v)| *items.entry(k).or_insert(0) += v);
        let partial2 = find_polymerization_score_rec((*ch, formula.1), rules, steps - 1, memo);
        partial2
            .iter()
            .for_each(|(&k, &v)| *items.entry(k).or_insert(0) += v);
        // don't count end char twice
        *items.entry(*ch).or_insert(0) -= 1;
    }

    memo.insert((formula, steps), items.clone());
    items
}

fn get_score(items: &HashMap<char, usize>) -> usize {
    let (least_count, most_count) = items
        .iter()
        .fold((usize::MAX, 0), |acc, (_current, &count)| {
            (acc.0.min(count), acc.1.max(count))
        });
    most_count - least_count
}

fn read(filename: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let content = read_to_string(filename).expect("Failed to read file");
    let mut lines = content.lines();
    let formula = lines.next().unwrap().chars().collect();
    let rules = lines
        .skip(1)
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let mut iter = from.chars();
            (
                (iter.next().unwrap(), iter.next().unwrap()),
                to.chars().next().unwrap(),
            )
        })
        .collect();
    (formula, rules)
}

#[cfg(test)]
mod tests {
    use crate::{find_polymerization_score, read};

    #[test]
    fn test1() {
        let (formula, rules) = read("test-input.txt");
        println!("rules {:?}", rules);
        assert_eq!(find_polymerization_score(&formula, &rules, 1), 1);
        assert_eq!(find_polymerization_score(&formula, &rules, 2), 5);
        assert_eq!(find_polymerization_score(&formula, &rules, 3), 7);
        assert_eq!(find_polymerization_score(&formula, &rules, 10), 1588);
        assert_eq!(
            find_polymerization_score(&formula, &rules, 40),
            2188189693529
        );
    }
}
