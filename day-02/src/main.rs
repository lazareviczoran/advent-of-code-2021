use core::panic;
use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    let mut submarine = Submarine::new();
    submarine.apply_commands(&input);
    println!("part 1 solution: {}", submarine.get_score());
}

struct Submarine {
    pos: (usize, usize),
}
impl Submarine {
    pub fn new() -> Self {
        Self { pos: (0, 0) }
    }

    pub fn apply_commands(&mut self, commands: &[Direction]) {
        for dir in commands {
            match dir {
                Direction::Forward(val) => {
                    self.pos.0 += val;
                }
                Direction::Up(val) => {
                    self.pos.1 -= val;
                }
                Direction::Down(val) => {
                    self.pos.1 += val;
                }
            }
        }
    }

    pub fn get_score(&self) -> usize {
        self.pos.0 * self.pos.1
    }
}

enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn read(filename: &str) -> Vec<Direction> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let mut items = l.split_terminator(" ");
            let dir = items.next().unwrap();
            let val = items.next().unwrap().parse().unwrap();
            match dir {
                "forward" => Direction::Forward(val),
                "up" => Direction::Up(val),
                "down" => Direction::Down(val),
                _ => panic!("unexpected value"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read;

    #[test]
    fn part1_test() {
        assert_eq!(2, 1);
    }
}
