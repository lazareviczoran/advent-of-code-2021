use std::fs::read_to_string;

fn main() {
    let lines = read("input.txt");
    println!("part1 solution: {}", get_compiler_score(&lines));
    println!("part2 solution: {}", get_compiler_middle_score(&lines));
}

fn get_compiler_score(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| get_stack_for_line(line))
        .filter(|stack| [')', ']', '}', '>'].contains(stack.last().unwrap()))
        .filter_map(|stack| Some(get_error_char_value(stack.last()?)))
        .sum()
}

fn get_compiler_middle_score(lines: &[String]) -> usize {
    let mut scores = lines
        .iter()
        .map(|line| get_stack_for_line(line))
        .filter(|stack| ![')', ']', '}', '>'].contains(stack.last().unwrap()))
        .map(|stack| get_completion_line_score(&stack))
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn get_error_char_value(ch: &char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected {}", ch),
    }
}

fn get_completion_char_value(ch: &char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected {}", ch),
    }
}

fn is_closing(a: char, b: char) -> bool {
    match a {
        '(' => b == ')',
        '[' => b == ']',
        '{' => b == '}',
        '<' => b == '>',
        _ => panic!("unexpected {}", a),
    }
}

fn get_stack_for_line(line: &str) -> Vec<char> {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            _ => {
                if let Some(curr) = stack.pop() {
                    if !is_closing(curr, ch) {
                        stack.push(curr);
                        stack.push(ch);
                        break;
                    }
                }
            }
        }
    }
    stack
}

fn get_completion_line_score(stack: &[char]) -> usize {
    stack
        .iter()
        .rev()
        .fold(0, |acc, ch| acc * 5 + get_completion_char_value(ch))
}

fn read(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_compiler_middle_score, get_compiler_score, read};

    #[test]
    fn part1_test() {
        let lines = read("test-input.txt");
        assert_eq!(get_compiler_score(&lines), 26397);
    }

    #[test]
    fn part2_test() {
        let lines = read("test-input.txt");
        assert_eq!(get_compiler_middle_score(&lines), 288957);
    }
}
