use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let map = read("input.txt");
    println!(
        "part1 solution: {}",
        get_score_after_n_steps(&mut map.clone(), 100)
    );
}

fn get_score_after_n_steps(map: &mut Vec<Vec<usize>>, steps: usize) -> usize {
    let mut count = 0;
    let mut has_flashed = HashSet::new();
    for _step in 0..steps {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                count += increase_item(map, (y as isize, x as isize), &mut has_flashed);
            }
        }
        has_flashed.clear();
        print_map(map);
    }
    count
}

fn increase_item(
    map: &mut Vec<Vec<usize>>,
    (y, x): (isize, isize),
    has_flashed: &mut HashSet<(isize, isize)>,
) -> usize {
    if y < 0
        || x < 0
        || y as usize >= map.len()
        || x as usize >= map[0].len()
        || map[y as usize][x as usize] == 0 && has_flashed.contains(&(y, x))
    {
        return 0;
    }
    map[y as usize][x as usize] += 1;
    let mut count = 0;
    has_flashed.insert((y, x));
    if map[y as usize][x as usize] > 9 {
        map[y as usize][x as usize] = 0;
        count += 1;
        count += increase_item(map, (y - 1, x - 1), has_flashed);
        count += increase_item(map, (y - 1, x), has_flashed);
        count += increase_item(map, (y - 1, x + 1), has_flashed);
        count += increase_item(map, (y, x - 1), has_flashed);
        count += increase_item(map, (y, x + 1), has_flashed);
        count += increase_item(map, (y + 1, x - 1), has_flashed);
        count += increase_item(map, (y + 1, x), has_flashed);
        count += increase_item(map, (y + 1, x + 1), has_flashed);
    }
    count
}

fn print_map(map: &mut Vec<Vec<usize>>) {
    let mut s = String::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            s.push((map[y][x] as u8 + b'0') as char);
        }
        s.push('\n');
    }
    println!("{}", s);
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
    use crate::{get_score_after_n_steps, read};

    #[test]
    fn part1_test() {
        let map = read("test-input.txt");
        assert_eq!(get_score_after_n_steps(&mut map.clone(), 10), 204);
        assert_eq!(get_score_after_n_steps(&mut map.clone(), 100), 1656);
    }
}
