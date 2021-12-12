use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let connections = read("input.txt");
    println!("part1 solution: {:?}", count_paths(&connections, false));
    println!("part2 solution: {:?}", count_paths(&connections, true));
}

fn count_paths(
    connections: &HashMap<String, Vec<String>>,
    can_visit_one_small_twice: bool,
) -> Option<usize> {
    let mut q = vec![(String::from("start"), HashSet::new(), false)];
    let mut count = 0;
    while !q.is_empty() {
        let (current, mut visited, mut visited_twice) = q.pop()?;
        if &current == "end" {
            count += 1;
            continue;
        }
        if current.chars().next()?.is_lowercase() {
            let already_visited = visited.contains(&current);
            if already_visited
                && (!can_visit_one_small_twice || visited_twice || &current == "start")
            {
                continue;
            }
            if already_visited {
                visited_twice = true;
            }
            visited.insert(current.clone());
        }
        let neighbours = connections.get(&current)?;
        for neighbour in neighbours {
            q.push((neighbour.to_string(), visited.clone(), visited_twice));
        }
    }

    Some(count)
}

fn read(filename: &str) -> HashMap<String, Vec<String>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .fold(HashMap::new(), |mut acc, l| {
            let (from, to) = l.split_once('-').unwrap();
            acc.entry(from.to_string())
                .or_insert_with(Vec::new)
                .push(to.to_string());
            acc.entry(to.to_string())
                .or_insert_with(Vec::new)
                .push(from.to_string());

            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::{count_paths, read};

    #[test]
    fn part1_test1() {
        let connections = read("test-input.txt");
        assert_eq!(count_paths(&connections, false), Some(10));
    }

    #[test]
    fn part1_test2() {
        let connections = read("test-input2.txt");
        assert_eq!(count_paths(&connections, false), Some(19));
    }

    #[test]
    fn part1_test3() {
        let connections = read("test-input3.txt");
        assert_eq!(count_paths(&connections, false), Some(226));
    }

    #[test]
    fn part2_test1() {
        let connections = read("test-input.txt");
        assert_eq!(count_paths(&connections, true), Some(36));
    }

    #[test]
    fn part2_test2() {
        let connections = read("test-input2.txt");
        assert_eq!(count_paths(&connections, true), Some(103));
    }

    #[test]
    fn part2_test3() {
        let connections = read("test-input3.txt");
        assert_eq!(count_paths(&connections, true), Some(3509));
    }
}
