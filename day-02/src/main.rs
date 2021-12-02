use core::panic;
use std::fs::read_to_string;

fn main() {
    let input = read("input.txt");
    let mut submarine = Submarine::new();
    submarine.apply_commands(&input);
    println!("part 1 solution: {}", submarine.get_score());

    submarine.reset();
    submarine.apply_commands2(&input);
    println!("part 2 solution: {}", submarine.get_score());
}

struct Position {
    x: usize,
    y: usize,
    aim: usize,
}
impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

struct Submarine {
    pos: Position,
}
impl Submarine {
    pub fn new() -> Self {
        Self {
            pos: Position::new(),
        }
    }

    pub fn apply_commands(&mut self, commands: &[Direction]) {
        for dir in commands {
            match dir {
                Direction::Forward(val) => self.pos.x += val,
                Direction::Up(val) => self.pos.y -= val,
                Direction::Down(val) => self.pos.y += val,
            }
        }
    }

    pub fn apply_commands2(&mut self, commands: &[Direction]) {
        for dir in commands {
            match dir {
                Direction::Forward(val) => {
                    self.pos.x += val;
                    self.pos.y += val * self.pos.aim;
                }
                Direction::Up(val) => self.pos.aim -= val,
                Direction::Down(val) => self.pos.aim += val,
            }
        }
    }

    pub fn get_score(&self) -> usize {
        self.pos.x * self.pos.y
    }

    pub fn reset(&mut self) {
        self.pos = Position::new();
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
            let mut items = l.split_terminator(' ');
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
    use crate::{read, Submarine};

    #[test]
    fn part1_test() {
        let input = read("test-input.txt");
        let mut submarine = Submarine::new();
        submarine.apply_commands(&input);

        assert_eq!(submarine.get_score(), 150);
    }

    #[test]
    fn part2_test() {
        let input = read("test-input.txt");
        let mut submarine = Submarine::new();
        submarine.apply_commands2(&input);

        assert_eq!(submarine.get_score(), 900);
    }
}
