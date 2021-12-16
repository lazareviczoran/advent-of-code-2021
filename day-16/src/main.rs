use std::{fs::read_to_string, iter::Peekable};

fn main() {
    let hex = read("input.txt");
    println!(
        "part1 solution: {}",
        decode_packet(convert_hex_to_bin(&hex).iter().peekable()).0
    );
}

fn convert_hex_to_bin(from: &str) -> Vec<u8> {
    from.chars()
        .map(|hex| match hex {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("unexpected input {}", hex),
        })
        .fold(vec![], |mut acc, binary| {
            acc.extend_from_slice(&binary);
            acc
        })
}

fn evaluate_binary(binary: &[u8]) -> usize {
    binary
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (pos, bit)| acc | (*bit as usize * (1 << pos)))
}

fn decode_packet<'a, T>(mut iter: Peekable<T>) -> (usize, usize, Peekable<T>)
where
    T: Iterator<Item = &'a u8>,
{
    let mut steps_count = 0;
    let mut sum_version_numbers = 0;
    println!("peek {:?}", iter.peek());
    // while iter.peek().is_some() {
    println!("size {:?}", iter.size_hint());
    let (size, _) = iter.size_hint();
    if size < 8 {
        (0..size).for_each(|_| {
            iter.next();
        });
        steps_count += size;
        return (sum_version_numbers, steps_count, iter);
    }
    let version = evaluate_binary(
        &(0..3)
            .filter_map(|_| iter.next())
            .copied()
            .collect::<Vec<_>>(),
    );
    // if version == 0 {
    //     return (sum_version_numbers, steps_count, iter);
    // }
    steps_count += 3;
    sum_version_numbers += version;
    let type_id = evaluate_binary(
        &(0..3)
            .filter_map(|_| iter.next())
            .copied()
            .collect::<Vec<_>>(),
    );
    steps_count += 3;
    println!(
        "version {} type_id {}, next bit {:?}",
        version,
        type_id,
        iter.peek()
    );
    match type_id {
        4 => loop {
            let is_last = iter.next().unwrap();
            println!("is last {}", is_last);
            (0..4).for_each(|_| {
                iter.next();
            });
            steps_count += 5;
            if is_last == &0 {
                break;
            }
        },
        _ => {
            if iter.next().unwrap() == &0 {
                let total_length = evaluate_binary(
                    &(0..15)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 15;
                let mut internal_steps_count = 0;
                loop {
                    println!(
                        "internal_steps_count {}, total_length {}",
                        internal_steps_count, total_length
                    );
                    let (subpackage_version, steps, remaining_iter) = decode_packet(iter);
                    sum_version_numbers += subpackage_version;
                    internal_steps_count += steps;
                    iter = remaining_iter;
                    if internal_steps_count > total_length {
                        steps_count += internal_steps_count;
                        return (sum_version_numbers, steps_count, iter);
                    }
                }
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                println!("num_of_11_bit_subpackets {}", num_of_11_bit_subpackets);
                steps_count += 11;
                for _ in 0..num_of_11_bit_subpackets {
                    let (subpacket_version, steps, remaining_iter) = decode_packet(iter);
                    println!("sp version {} steps {}", subpacket_version, steps);
                    sum_version_numbers += subpacket_version;
                    iter = remaining_iter;
                    steps_count += steps;
                }
                // let total_length = num_of_11_bit_subpackets * 11;
                // (0..total_length).for_each(|_| {
                //     iter.next();
                // });
            }
        }
    }
    println!("peek {:?}", iter.peek());
    // }
    (sum_version_numbers, steps_count, iter)
}

fn read(filename: &str) -> String {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .take(1)
        .map(|l| l.to_string())
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{convert_hex_to_bin, decode_packet, evaluate_binary};

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate_binary(&[1, 0, 0]), 4);
    }

    #[test]
    fn test_convert_hex_to_bin() {
        let bin = convert_hex_to_bin("D2FE28");
        assert_eq!(
            bin,
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
        assert_eq!(decode_packet(bin.iter().peekable()).0, 6);

        let bin = convert_hex_to_bin("38006F45291200");
        assert_eq!(
            bin,
            vec![
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0,
                0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(decode_packet(bin.iter().peekable()).0, 9);

        let bin = convert_hex_to_bin("EE00D40C823060");
        assert_eq!(
            bin,
            vec![
                1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
                1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(decode_packet(bin.iter().peekable()).0, 14);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            decode_packet(convert_hex_to_bin("8A004A801A8002F478").iter().peekable()).0,
            16
        );

        assert_eq!(
            decode_packet(
                convert_hex_to_bin("620080001611562C8802118E34")
                    .iter()
                    .peekable()
            )
            .0,
            12
        );
        assert_eq!(
            decode_packet(
                convert_hex_to_bin("C0015000016115A2E0802F182340")
                    .iter()
                    .peekable()
            )
            .0,
            23
        );
        assert_eq!(
            decode_packet(
                convert_hex_to_bin("A0016C880162017C3686B18A3D4780")
                    .iter()
                    .peekable()
            )
            .0,
            31
        );
    }
}
