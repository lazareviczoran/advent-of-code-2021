use std::{cmp::Ordering, collections::BinaryHeap, fs::read_to_string};

fn main() {
    let map = read("input.txt");
    println!("part1 solution: {:?}", shortest_path(&map, (0, 0),));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (isize, isize),
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &[Vec<usize>], start: (isize, isize)) -> Option<usize> {
    let target = (map[0].len() as isize - 1, map.len() as isize - 1);
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];

    let mut heap = BinaryHeap::new();

    dist[start.1 as usize][start.0 as usize] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == target {
            return Some(cost);
        }

        let (x, y) = position;
        if cost > dist[y as usize][x as usize] {
            continue;
        }

        for (next_x, next_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|&(next_x, next_y)| {
                next_x >= 0
                    && (next_x as usize) < map[0].len()
                    && next_y >= 0
                    && (next_y as usize) < map.len()
            })
        {
            let next = State {
                cost: cost + map[next_y as usize][next_x as usize],
                position: (next_x, next_y),
            };

            if next.cost < dist[next_y as usize][next_x as usize] {
                heap.push(next);
                dist[next_y as usize][next_x as usize] = next.cost;
            }
        }
    }

    None
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
    use crate::{read, shortest_path};

    #[test]
    fn part1_test() {
        let map = read("test-input.txt");
        assert_eq!(shortest_path(&map, (0, 0)), Some(40));
    }
}
