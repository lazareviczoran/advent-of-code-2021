use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const START: &str = "start";
const END: &str = "end";

fn main() {
    let content = read("input.txt");
    let connections = parse(&content);
    println!("part1 solution: {:?}", count_paths(&connections, false));
    println!("part2 solution: {:?}", count_paths(&connections, true));
}

fn count_paths<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    can_visit_one_small_twice: bool,
) -> Option<usize> {
    let mut q = vec![(START, HashSet::new(), false)];
    let mut count = 0;
    while !q.is_empty() {
        let (current, mut visited, mut visited_twice) = q.pop()?;
        if current == END {
            count += 1;
            continue;
        }
        if current.chars().next()?.is_lowercase() {
            let already_visited = visited.contains(&current);
            if already_visited && (!can_visit_one_small_twice || visited_twice || current == START)
            {
                continue;
            }
            if already_visited {
                visited_twice = true;
            }
            visited.insert(current);
        }
        for neighbour in connections.get(&current)? {
            q.push((neighbour, visited.clone(), visited_twice));
        }
    }

    Some(count)
}

fn read(filename: &str) -> String {
    read_to_string(filename).expect("Failed to read file")
}

fn parse(content: &str) -> HashMap<&str, Vec<&str>> {
    content.lines().fold(HashMap::new(), |mut acc, l| {
        let (from, to) = l.split_once('-').unwrap();
        acc.entry(from).or_insert_with(Vec::new).push(to);
        acc.entry(to).or_insert_with(Vec::new).push(from);

        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::{count_paths, parse, read};

    #[test]
    fn test1() {
        let content = read("test-input.txt");
        let connections = parse(&content);
        assert_eq!(count_paths(&connections, false), Some(10));
        assert_eq!(count_paths(&connections, true), Some(36));
    }

    #[test]
    fn test2() {
        let content = read("test-input2.txt");
        let connections = parse(&content);
        assert_eq!(count_paths(&connections, false), Some(19));
        assert_eq!(count_paths(&connections, true), Some(103));
    }

    #[test]
    fn test3() {
        let content = read("test-input3.txt");
        let connections = parse(&content);
        assert_eq!(count_paths(&connections, false), Some(226));
        assert_eq!(count_paths(&connections, true), Some(3509));
    }
}
