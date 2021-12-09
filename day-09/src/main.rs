use std::fs::read_to_string;

fn main() {
    let map = read("input.txt");
    println!("part1 solution: {}", sum_risk_level(&map));
}

fn sum_risk_level(map: &[Vec<usize>]) -> usize {
    println!("{:?}", map);
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut positions = vec![];
            if x > 0 {
                positions.push((y, x - 1));
            }
            if x < map[0].len() - 1 {
                positions.push((y, x + 1));
            }
            if y > 0 {
                positions.push((y - 1, x));
            }
            if y < map.len() - 1 {
                positions.push((y + 1, x));
            }
            if positions.iter().all(|&(y1, x1)| map[y1][x1] > map[y][x]) {
                sum += map[y][x] + 1;
            }
        }
    }
    sum
}

fn read(filename: &str) -> Vec<Vec<usize>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.chars().map(|ch| (ch as u8 - b'0') as usize).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{read, sum_risk_level};

    #[test]
    fn part1_test() {
        let map = read("test-input.txt");
        assert_eq!(sum_risk_level(&map), 15);
    }
}
