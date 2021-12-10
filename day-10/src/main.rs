use std::fs::read_to_string;

fn main() {
    let lines = read("input.txt");
    println!("part1 solution: {}", get_compiler_score(&lines));
}

fn get_compiler_score(lines: &[String]) -> usize {
    lines.iter().filter_map(get_error_line_score).sum()
}

fn get_char_value(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected {}", ch),
    }
}

fn get_error_line_score(line: &String) -> Option<usize> {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(current_open) => {
                    if current_open == '(' && ch != ')'
                        || current_open == '[' && ch != ']'
                        || current_open == '{' && ch != '}'
                        || current_open == '<' && ch != '>'
                    {
                        return Some(get_char_value(ch));
                    }
                }
                _ => panic!("unexpected char {}", ch),
            },
            _ => panic!("unexpected char {}", ch),
        }
    }
    None
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
    use crate::{get_compiler_score, read};

    #[test]
    fn part1_test() {
        let lines = read("test-input.txt");
        assert_eq!(get_compiler_score(&lines), 26397);
    }
}
