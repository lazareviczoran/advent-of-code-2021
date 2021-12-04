use std::fs::read_to_string;

fn main() {
    let mut input = read("input.txt");

    println!("part1 solution: {}", get_score(&mut input.1, &input.0));
}

fn get_score(tickets: &mut Vec<Vec<Vec<(usize, bool)>>>, drawn_nums: &[usize]) -> usize {
    let mut finished = false;
    for number in drawn_nums {
        for t in tickets.iter_mut() {
            finished |= find_and_set(t, number);
            if finished {
                let sum_of_unset = t.iter().fold(0, |acc, rows| {
                    acc + rows
                        .iter()
                        .filter(|(_, is_used)| !*is_used)
                        .map(|(val, _is_used)| *val)
                        .sum::<usize>()
                });

                return sum_of_unset * *number;
            }
        }
    }
    panic!("shouldn't get here");
}

fn find_and_set(ticket: &mut Vec<Vec<(usize, bool)>>, target: &usize) -> bool {
    for i in 0..ticket.len() {
        for j in 0..ticket[i].len() {
            if &ticket[i][j].0 == target {
                ticket[i][j].1 = true;
                if check_horizontal(ticket, i) || check_vertical(ticket, j) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_horizontal(ticket: &mut [Vec<(usize, bool)>], row: usize) -> bool {
    if ticket[row].iter().all(|(_val, used)| *used) {
        return true;
    }
    false
}

fn check_vertical(ticket: &mut [Vec<(usize, bool)>], col: usize) -> bool {
    if ticket.iter().all(|row| row[col].1) {
        return true;
    }
    false
}

fn read(filename: &str) -> (Vec<usize>, Vec<Vec<Vec<(usize, bool)>>>) {
    let input = read_to_string(filename).expect("Failed to read file");
    let mut lines = input.lines().peekable();
    let drawn_nums = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut tickets = vec![];
    while let Some(_) = lines.next() {
        let mut ticket = vec![];
        for _ in 0..5 {
            ticket.push(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| (s.parse().unwrap(), false))
                    .collect(),
            );
        }
        tickets.push(ticket);
    }
    (drawn_nums, tickets)
}

#[cfg(test)]
mod tests {
    use crate::{get_score, read};

    #[test]
    fn part1_test() {
        let mut input = read("test-input.txt");

        assert_eq!(get_score(&mut input.1, &input.0), 4512);
    }
}
