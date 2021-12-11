use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut map = read("input.txt");
    println!(
        "part1 solution: {}",
        score_after_n_steps(&mut map.clone(), 100)
    );
    println!("part2 solution: {}", find_first_sync_step(&mut map));
}

fn find_first_sync_step(map: &mut Vec<Vec<usize>>) -> usize {
    let mut has_flashed = HashSet::new();
    for step in 1.. {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                increase_item(map, (y as isize, x as isize), &mut has_flashed);
            }
        }
        if has_flashed.len() == map.len() * map[0].len() {
            return step;
        }
        has_flashed.clear();
    }
    unreachable!()
}

fn score_after_n_steps(map: &mut Vec<Vec<usize>>, steps: usize) -> usize {
    let mut count = 0;
    let mut has_flashed = HashSet::new();
    for _step in 0..steps {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                count += increase_item(map, (y as isize, x as isize), &mut has_flashed);
            }
        }
        has_flashed.clear();
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
    if map[y as usize][x as usize] > 9 {
        map[y as usize][x as usize] = 0;
        has_flashed.insert((y, x));
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

fn read(filename: &str) -> Vec<Vec<usize>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.chars().map(|ch| (ch as u8 - b'0') as usize).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{find_first_sync_step, read, score_after_n_steps};

    #[test]
    fn part1_test() {
        let mut map = read("test-input.txt");
        assert_eq!(score_after_n_steps(&mut map.clone(), 10), 204);
        assert_eq!(score_after_n_steps(&mut map, 100), 1656);
    }

    #[test]
    fn part2_test() {
        let mut map = read("test-input.txt");
        assert_eq!(find_first_sync_step(&mut map), 195);
    }
}
