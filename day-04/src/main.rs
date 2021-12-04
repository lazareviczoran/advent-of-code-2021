use std::{fs::read_to_string, mem::take};

fn main() {
    let mut bingo = read("input.txt");
    println!("part1 solution: {:?}", bingo.get_first_winning_score());
    bingo.reset();
    println!("part2 solution: {:?}", bingo.get_last_winning_score());
}

type Field = (usize, bool);

#[derive(Clone)]
struct Ticket {
    fields: Vec<Vec<Field>>,
}
impl Ticket {
    pub fn new(fields: Vec<Vec<Field>>) -> Self {
        Self { fields }
    }

    pub fn mark_and_check(&mut self, target: &usize) -> bool {
        let needs_changes = self.fields.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, (val, _))| if val == target { Some((i, j)) } else { None })
        });
        needs_changes.map_or(false, |(i, j)| {
            self.fields[i][j].1 = true;
            self.has_won((i, j))
        })
    }

    pub fn has_won(&self, pos: (usize, usize)) -> bool {
        self.fields[pos.0].iter().all(|(_, is_used)| *is_used)
            || self.fields.iter().all(|row| row[pos.1].1)
    }

    pub fn sum_unset_fields(&self) -> usize {
        self.fields.iter().fold(0, |acc, rows| {
            acc + rows
                .iter()
                .filter(|(_, is_used)| !*is_used)
                .map(|(val, _is_used)| *val)
                .sum::<usize>()
        })
    }

    pub fn print(&self) {
        let format_field = |(val, is_used): &(usize, bool)| match *is_used {
            true => format!("\x1B[41;1m{:4}\x1B[0m", val),
            _ => format!("{:4}", val),
        };
        let format_line =
            |row: &Vec<(usize, bool)>| row.iter().map(format_field).collect::<String>();
        let lines = self.fields.iter().map(format_line).collect::<Vec<String>>();
        println!("ticket\n{}", lines.join("\n"));
    }
}
struct Bingo {
    tickets: Vec<Ticket>,
    drawn_numbers: Vec<usize>,
    initial_tickets: Vec<Ticket>,
}
impl Bingo {
    pub fn new(tickets: Vec<Ticket>, drawn_numbers: Vec<usize>) -> Self {
        Self {
            initial_tickets: tickets.clone(),
            tickets,
            drawn_numbers,
        }
    }

    pub fn get_first_winning_score(&mut self) -> Option<usize> {
        self.drawn_numbers.iter().find_map(|number| {
            self.tickets
                .iter_mut()
                .find_map(|t| match t.mark_and_check(number) {
                    true => Some(t.sum_unset_fields() * *number),
                    _ => None,
                })
        })
    }

    pub fn get_last_winning_score(&mut self) -> Option<usize> {
        let mut last_winning_ticket: Option<(Ticket, usize)> = None;
        let mut numbers_iter = self.drawn_numbers.iter().peekable();
        while numbers_iter.peek().is_some() && !self.tickets.is_empty() {
            let number = numbers_iter.next()?;
            take(&mut self.tickets).into_iter().for_each(|mut ticket| {
                match ticket.mark_and_check(number) {
                    true => last_winning_ticket = Some((ticket, *number)),
                    _ => self.tickets.push(ticket),
                }
            });
        }
        last_winning_ticket.map(|(ticket, number)| ticket.sum_unset_fields() * number)
    }

    pub fn reset(&mut self) {
        self.tickets = self.initial_tickets.clone();
    }
}

fn read(filename: &str) -> Bingo {
    let input = read_to_string(filename).expect("Failed to read file");
    let mut lines = input.lines();
    let drawn_nums = lines
        .next()
        .expect("no lines")
        .split_terminator(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut tickets = vec![];
    while lines.next().is_some() {
        let mut fields = vec![];
        for _ in 0..5 {
            fields.push(
                lines
                    .next()
                    .expect("no more lines")
                    .split_whitespace()
                    .filter_map(|s| Some((s.parse().ok()?, false)))
                    .collect(),
            );
        }
        tickets.push(Ticket::new(fields));
    }
    Bingo::new(tickets, drawn_nums)
}

#[cfg(test)]
mod tests {
    use crate::read;

    #[test]
    fn part1_test() {
        let mut bingo = read("test-input.txt");

        assert_eq!(bingo.get_first_winning_score(), Some(4512));
    }

    #[test]
    fn part2_test() {
        let mut bingo = read("test-input.txt");

        assert_eq!(bingo.get_last_winning_score(), Some(1924));
    }
}
