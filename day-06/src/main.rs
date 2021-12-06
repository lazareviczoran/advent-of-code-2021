use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    println!("part1 solution: {}", run_cycle(&mut input.clone(), 80));
}

fn run_cycle(input: &mut Vec<usize>, period: usize) -> usize {
    for day in 0..period {
        let mut new_fishes = Vec::new();
        input.iter_mut().for_each(|fish| {
            if fish == &0 {
                new_fishes.push(8);
                *fish = 6;
            } else {
                *fish -= 1;
            }
        });
        // println!("{:?}", input);
        input.extend(new_fishes);
    }
    input.iter().count()
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
    use crate::{read, run_cycle};

    #[test]
    fn part1_test() {
        let mut input = read("test-input.txt");
        assert_eq!(run_cycle(&mut input.clone(), 18), 26);
        assert_eq!(run_cycle(&mut input, 80), 5934);
    }
}
