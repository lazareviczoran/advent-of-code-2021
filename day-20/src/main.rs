use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let (enhancement_algo, image) = read("input.txt");
    println!(
        "part1 solution: {}",
        count_light(&apply_algo_n_times(&enhancement_algo, &image, 2))
    );
    println!(
        "part2 solution: {}",
        count_light(&apply_algo_n_times(&enhancement_algo, &image, 50))
    );
}

fn count_light(image: &BTreeMap<(isize, isize), char>) -> usize {
    image.iter().filter(|&(_, &ch)| ch == '#').count()
}

fn apply_algo_n_times(
    algo: &str,
    image: &BTreeMap<(isize, isize), char>,
    n: isize,
) -> BTreeMap<(isize, isize), char> {
    let first_algo = algo.chars().next().unwrap();
    let last_algo = algo.chars().nth(9).unwrap();
    (1..=n).fold(image.clone(), |mut map, i| {
        map = apply_algo(algo, &map, first_algo == '#' && last_algo == '.');
        if first_algo == '#' && last_algo == '.' {
            let first = *map.iter().next().unwrap().0;
            let last = *map.iter().last().unwrap().0;
            if i % 2 == 0 {
                map.retain(|&(x, y), _val| {
                    x >= first.0 + 4 && x <= last.0 - 4 && y >= first.1 + 4 && y <= last.1 - 4
                });
            }
        }
        map
    })
}

fn apply_algo(
    algo: &str,
    image: &BTreeMap<(isize, isize), char>,
    invert: bool,
) -> BTreeMap<(isize, isize), char> {
    let mut new_image = image.clone();
    let first = image.iter().next().unwrap().0;
    let last = image.iter().last().unwrap().0;
    for x in first.0 - 3..=last.0 + 3 {
        for y in first.1 - 3..=last.1 + 3 {
            let value = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (x_diff, y_diff))| {
                acc | (((image.get(&(x + x_diff, y + y_diff)).unwrap_or(&'.') == &'#') as usize)
                    * (1 << (8 - i)))
            });
            let new_char = algo
                .chars()
                .nth(value)
                .unwrap_or(if invert { '#' } else { '.' });
            new_image.insert((x, y), new_char);
        }
    }

    new_image
}

#[allow(unused)]
fn print_map(map: &BTreeMap<(isize, isize), char>) {
    let first = map.iter().next().unwrap().0;
    let last = map.iter().last().unwrap().0;
    let mut s = String::new();
    for y in first.1..=last.1 {
        for x in first.0..=last.0 {
            s.push(*map.get(&(x, y)).unwrap_or(&'.'));
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn read(filename: &str) -> (String, BTreeMap<(isize, isize), char>) {
    let content = read_to_string(filename).expect("Failed to read file");
    let (enchancment_algo, image) = content.split_once("\n\n").unwrap();
    let mut image = image
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as isize, y as isize), ch))
        })
        .fold(BTreeMap::new(), |mut acc, items| {
            acc.extend(items);
            acc
        });

    let first = *image.iter().next().unwrap().0;
    let last = *image.iter().last().unwrap().0;

    for y in first.1..=last.1 {
        for x in first.0..=last.0 {
            if x < first.0 || x > last.0 || y < first.1 || y > last.1 {
                image.insert((x, y), '.');
            }
        }
    }

    (enchancment_algo.to_string(), image)
}

#[cfg(test)]
mod tests {
    use crate::{apply_algo_n_times, count_light, read};

    #[test]
    fn part1_test() {
        let (enhancement_algo, image) = read("test-input.txt");
        assert_eq!(
            count_light(&apply_algo_n_times(&enhancement_algo, &image, 2)),
            35
        );
    }

    #[test]
    fn part2_test() {
        let (enhancement_algo, image) = read("test-input.txt");
        assert_eq!(
            count_light(&apply_algo_n_times(&enhancement_algo, &image, 50)),
            3351
        );
    }
}
