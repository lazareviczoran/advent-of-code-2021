use std::{
    fs::read_to_string,
    io::{Error, ErrorKind},
};

fn main() -> Result<(), Error> {
    let input = read("input.txt")?;
    let mut submarine = Submarine::new();
    submarine.apply_commands(&input);
    println!("part 1 solution: {}", submarine.get_score());

    submarine.reset();
    submarine.apply_commands2(&input);
    println!("part 2 solution: {}", submarine.get_score());
    Ok(())
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

fn read(filename: &str) -> Result<Vec<Direction>, Error> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let (dir, val) = l
                .split_once(' ')
                .ok_or_else(|| Error::new(ErrorKind::Other, "input not in correct format"))?;
            let val = val
                .parse()
                .map_err(|_| Error::new(ErrorKind::Other, "cannot convert to usize"))?;
            match dir {
                "forward" => Ok(Direction::Forward(val)),
                "up" => Ok(Direction::Up(val)),
                "down" => Ok(Direction::Down(val)),
                _ => Err(Error::new(ErrorKind::Other, "unexpected value")),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{read, Submarine};
    use std::io::Error;

    #[test]
    fn part1_test() -> Result<(), Error> {
        let input = read("test-input.txt")?;
        let mut submarine = Submarine::new();
        submarine.apply_commands(&input);

        assert_eq!(submarine.get_score(), 150);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<(), Error> {
        let input = read("test-input.txt")?;
        let mut submarine = Submarine::new();
        submarine.apply_commands2(&input);

        assert_eq!(submarine.get_score(), 900);
        Ok(())
    }
}
