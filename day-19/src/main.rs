use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

fn main() {
    let scanners = read("input.txt");

    println!("part1 solution: {}", 1);
}

type Point<const N: usize> = [isize; N];
fn distance_to<const N: usize>(pos: &Point<N>, other: &Point<N>) -> isize {
    let dist = (0..N).map(|i| (pos[i] - other[i]).pow(2)).sum::<isize>();
    // println!("distance between {:?} and {:?} is {}", pos, other, dist);
    dist
}

#[derive(Debug)]
struct DirMapping<const N: usize> {
    from: usize,
    to: usize,
    coef: [isize; N],
    pos_transforms: [usize; N],
}

fn find_full_map<const N: usize>(scanners: &[Vec<Point<N>>]) -> HashSet<Point<N>> {
    let mut res = HashSet::new();
    let mut all_mappings = vec![];
    let mut direction_mappings = HashMap::new();
    let mut scanner_positions: HashMap<usize, Point<N>> = HashMap::new();
    scanner_positions.insert(0, [0; N]);
    for pair in scanners.iter().enumerate().combinations(2) {
        if let Some(mappings) = find_point_mappings(pair[0].1, pair[1].1, 12) {
            println!("found mappings for {:?} and {:?}", pair[0].0, pair[1].0);

            // getting direction mapping
            let first_two = mappings.iter().take(2).collect::<Vec<_>>();
            let (diffs1, diffs2) = (0..N).fold(([0; N], [0; N]), |(mut acc1, mut acc2), i| {
                acc1[i] = first_two[0].0[i] - first_two[1].0[i];
                acc2[i] = first_two[0].1[i] - first_two[1].1[i];
                (acc1, acc2)
            });
            println!("diffs {:?}, {:?}", diffs1, diffs2);
            let mut pos_transforms = [0; N];
            let mut coef = [0; N];
            for i in 0..N {
                let pos = diffs2
                    .iter()
                    .position(|x| x.abs() == diffs1[i].abs())
                    .unwrap();
                pos_transforms[i] = pos;
                coef[i] = if diffs1[i].signum() == diffs2[pos].signum() {
                    1
                } else {
                    -1
                };
                direction_mappings.insert(
                    [pair[0].0, pair[1].0],
                    DirMapping {
                        from: pair[0].0,
                        to: pair[1].0,
                        pos_transforms,
                        coef,
                    },
                );
                direction_mappings.insert(
                    [pair[1].0, pair[0].0],
                    DirMapping {
                        from: pair[0].0,
                        to: pair[1].0,
                        pos_transforms,
                        coef,
                    },
                );
            }
            println!("transf {:?}", direction_mappings);
            println!("scanner positions {:?}, {}", scanner_positions, pair[0].0);
            let curr_pos = if let Some(position) = scanner_positions.get(&pair[0].0) {
                position
            } else {
                scanner_positions.get(&pair[1].0).unwrap()
            };
            let (from, to) = mappings.iter().next().unwrap();
            let mut other_pos = [0; N];
            println!("from {:?} to {:?}", from, to);
            curr_pos.iter().enumerate().for_each(|(i, val)| {
                println!(
                    "from[{}] {} - to[{}] {}",
                    i, from[i], pos_transforms[i], to[pos_transforms[i]]
                );
                other_pos[i] = val + from[i] - to[pos_transforms[i]] * coef[i];
            });
            println!("pos for scanner {} is {:?}", pair[1].0, other_pos);
            scanner_positions.insert(pair[1].0, other_pos);

            all_mappings.push(mappings);
        }
    }

    println!("all_mappings {:?}", all_mappings);
    res
}

fn find_relative_pos<const N: usize>(
    curr_pos: Point<N>,
    mappings: &HashMap<Point<N>, Point<N>>,
) -> Point<N> {
    mappings
        .iter()
        .next()
        .map(|(from, to)| {
            let mut res = [0; N];
            res.iter_mut()
                .enumerate()
                .for_each(|(i, curr)| *curr = curr_pos[i] + to[i] - from[i]);
            res
        })
        .unwrap()
}

fn find_point_mappings<const N: usize>(
    scanner1: &[Point<N>],
    scanner2: &[Point<N>],
    required_matches: usize,
) -> Option<HashMap<Point<N>, Point<N>>> {
    let pairs1 = scanner1.iter().permutations(2).collect::<Vec<_>>();
    let pairs2 = scanner2.iter().permutations(2).collect::<Vec<_>>();
    let mut candidates = HashMap::new();
    for pair1 in &pairs1 {
        for pair2 in &pairs2 {
            if distance_to(pair1[0], pair1[1]) == distance_to(pair2[0], pair2[1]) {
                candidates
                    .entry(*pair1[0])
                    .or_insert_with(HashSet::new)
                    .insert(*pair2[0]);
                candidates
                    .entry(*pair1[0])
                    .or_insert_with(HashSet::new)
                    .insert(*pair2[1]);
                candidates
                    .entry(*pair1[1])
                    .or_insert_with(HashSet::new)
                    .insert(*pair2[0]);
                candidates
                    .entry(*pair1[1])
                    .or_insert_with(HashSet::new)
                    .insert(*pair2[1]);
            }
        }
    }
    // println!("candidates {:?}", candidates);
    let mut final_mappings = HashMap::new();
    if find_point_mappings_rec(
        scanner1.iter().copied().collect(),
        &candidates,
        required_matches,
        HashSet::new(),
        HashSet::new(),
        Vec::new(),
        &mut final_mappings,
    ) {
        return Some(final_mappings);
    }
    None
}

fn find_point_mappings_rec<const N: usize>(
    remaining: HashSet<Point<N>>,
    mappings: &HashMap<Point<N>, HashSet<Point<N>>>,
    required_matches: usize,
    used_a: HashSet<Point<N>>,
    used_b: HashSet<Point<N>>,
    used_vec: Vec<Point<N>>,
    acc: &mut HashMap<Point<N>, Point<N>>,
) -> bool {
    // println!(
    //     "required_matches {},\nused_a {:?},\nused_b {:?},\nacc {:?}",
    //     required_matches, used_a, used_b, acc
    // );

    if required_matches == 0 {
        return true;
    }
    if (&remaining - &used_a).is_empty() {
        return false;
    }
    for &curr in &remaining {
        // println!("current item {:?}", curr);
        if let Some(candidates) = mappings.get(&curr) {
            // println!("candidates {:?}", candidates - &used_b);
            for candidate in candidates - &used_b {
                acc.insert(curr, candidate);
                let mut new_used_vec = used_vec.clone();
                new_used_vec.push(curr);
                if (used_vec.is_empty()
                    || distance_to(&curr, used_vec.last().unwrap())
                        == distance_to(&candidate, acc.get(used_vec.last().unwrap()).unwrap()))
                    && find_point_mappings_rec(
                        &remaining - &HashSet::from([curr]),
                        mappings,
                        required_matches - 1,
                        &used_a | &HashSet::from([curr]),
                        &used_b | &HashSet::from([candidate]),
                        new_used_vec,
                        acc,
                    )
                {
                    return true;
                }
                acc.remove(&curr);
            }
        }
    }
    false
}

fn read(filename: &str) -> Vec<Vec<Point<3>>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .split("\n\n")
        .map(|l| {
            l.lines()
                .skip(1)
                .map(|coord| {
                    let mut coord_iter = coord.split_terminator(',');
                    let x = coord_iter.next().unwrap().parse().ok().unwrap();
                    let y = coord_iter.next().unwrap().parse().ok().unwrap();
                    let z = coord_iter.next().unwrap().parse().ok().unwrap();
                    [x, y, z]
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{distance_to, find_full_map, find_point_mappings, find_relative_pos, read};

    #[test]
    fn part1_test() {
        let scanners = read("test-input.txt");

        assert_eq!(
            distance_to(&[-618, -824, -621], &[-537, -823, -458]),
            distance_to(&[686, 422, 578], &[605, 423, 415])
        );

        let mut mappings = find_point_mappings(&scanners[0], &scanners[1], 12).unwrap();
        mappings.insert([-618, -824, -621], [686, 422, 578]);
        // assert_eq!(find_relative_pos([0, 0, 0], &mappings), [68, -1246, -43]);
        find_full_map(&scanners);
        assert_eq!(2, 1);
    }
}
