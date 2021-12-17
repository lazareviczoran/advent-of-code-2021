use std::fs::read_to_string;

fn main() {
    let (x_target_range, y_target_range) = read("input.txt");

    println!(
        "part1 solution: {}",
        find_highest_y((x_target_range, y_target_range))
    );
}

fn find_highest_y((x_range, y_range): ((isize, isize), (isize, isize))) -> isize {
    println!("target_x {}", x_range.0);
    let mut max_y = isize::MIN;
    for vel_x in 0..x_range.0 {
        for vel_y in 0..1000 {
            if let Some(trajectory) = launch_probe((vel_x, vel_y), (x_range, y_range)) {
                max_y = trajectory.iter().fold(max_y, |acc, &(_, y)| acc.max(y));
                if max_y == 45 {
                    println!("vel_x {}, vel_y {}", vel_x, vel_y);
                    println!("{:?}", trajectory);
                }
            }
        }
    }
    max_y
}

fn launch_probe(
    initial_velocity: (isize, isize),
    (x_range, y_range): ((isize, isize), (isize, isize)),
) -> Option<Vec<(isize, isize)>> {
    let (mut x, mut y) = (0, 0);
    let mut trajectory = vec![(0, 0)];
    let mut velocity = initial_velocity;
    while y >= y_range.0 {
        x += velocity.0;
        y += velocity.1;
        trajectory.push((x, y));
        velocity.0 += match velocity.0 {
            val if val > 0 => -1,
            val if val < 0 => 1,
            _ => 0,
        };
        velocity.1 -= 1;
        if has_hit_target_region((x, y), (x_range, y_range)) {
            return Some(trajectory);
        }
    }
    None
}

fn has_hit_target_region(
    position: (isize, isize),
    (x_range, y_range): ((isize, isize), (isize, isize)),
) -> bool {
    position.0 >= x_range.0
        && position.0 <= x_range.1
        && position.1 >= y_range.0
        && position.1 <= y_range.1
}

fn read(filename: &str) -> ((isize, isize), (isize, isize)) {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .next()
        .map(|l| {
            let stripped = l.strip_prefix("target area: ").unwrap();
            let (x_range_str, y_range_str) = stripped.split_once(", ").unwrap();
            (
                x_range_str
                    .strip_prefix("x=")
                    .map(|content| {
                        content
                            .split_once("..")
                            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
                            .unwrap()
                    })
                    .unwrap(),
                y_range_str
                    .strip_prefix("y=")
                    .map(|content| {
                        content
                            .split_once("..")
                            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
                            .unwrap()
                    })
                    .unwrap(),
            )
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{find_highest_y, launch_probe, read};

    #[test]
    fn part1_test() {
        let (x_target_range, y_target_range) = read("test-input.txt");
        assert_eq!(
            launch_probe((7, 2), (x_target_range, y_target_range)),
            Some(vec![
                (0, 0),
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ])
        );
        assert_eq!(
            launch_probe((6, 3), (x_target_range, y_target_range)),
            Some(vec![
                (0, 0),
                (6, 3),
                (11, 5),
                (15, 6),
                (18, 6),
                (20, 5),
                (21, 3),
                (21, 0),
                (21, -4),
                (21, -9),
            ])
        );
        assert_eq!(
            launch_probe((9, 0), (x_target_range, y_target_range)),
            Some(vec![(0, 0), (9, 0), (17, -1), (24, -3), (30, -6)])
        );
        assert_eq!(
            launch_probe((17, 4), (x_target_range, y_target_range)),
            None
        );
        assert_eq!(find_highest_y((x_target_range, y_target_range)), 451);
    }
}
