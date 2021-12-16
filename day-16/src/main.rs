use std::{fs::read_to_string, iter::Peekable};

fn main() {
    let hex = read("input.txt");
    println!(
        "part1 solution: {}",
        decode_packet(convert_hex_to_bin(&hex).iter().peekable()).0
    );
    println!(
        "part2 solution: {:?}",
        evaluate_packet(convert_hex_to_bin(&hex).iter().peekable()).0
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

fn evaluate_binary_usize(binary: &[u8]) -> usize {
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
    steps_count += 3;
    sum_version_numbers += version;
    let type_id = evaluate_binary(
        &(0..3)
            .filter_map(|_| iter.next())
            .copied()
            .collect::<Vec<_>>(),
    );
    steps_count += 3;
    match type_id {
        4 => loop {
            let is_last = iter.next().unwrap();
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
                    let (subpacket_version, steps, remaining_iter) = decode_packet(iter);
                    sum_version_numbers += subpacket_version;
                    internal_steps_count += steps;
                    iter = remaining_iter;
                    if internal_steps_count >= total_length {
                        steps_count += internal_steps_count;
                        break;
                    }
                }
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 11;
                for _ in 0..num_of_11_bit_subpackets {
                    let (subpacket_version, steps, remaining_iter) = decode_packet(iter);
                    sum_version_numbers += subpacket_version;
                    iter = remaining_iter;
                    steps_count += steps;
                }
            }
        }
    }
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

fn evaluate_packet<'a, T>(iter: Peekable<T>) -> (Option<usize>, usize, Peekable<T>)
where
    T: Iterator<Item = &'a u8>,
{
    evaluate_packet_internal(iter, None)
}

fn evaluate_packet_internal<'a, T>(
    mut iter: Peekable<T>,
    length_limit: Option<usize>,
) -> (Option<usize>, usize, Peekable<T>)
where
    T: Iterator<Item = &'a u8>,
{
    let mut steps_count = 0;
    let mut packet_value = 0;
    let _version = evaluate_binary(
        &(0..3)
            .filter_map(|_| iter.next())
            .copied()
            .collect::<Vec<_>>(),
    );
    steps_count += 3;
    let type_id = evaluate_binary(
        &(0..3)
            .filter_map(|_| iter.next())
            .copied()
            .collect::<Vec<_>>(),
    );
    steps_count += 3;
    match type_id {
        4 => {
            let mut bits = vec![];
            loop {
                let is_last = iter.next().unwrap();
                (0..4)
                    .filter_map(|_| Some(*iter.next()?))
                    .for_each(|bit| bits.push(bit));

                steps_count += 5;
                if is_last == &0 {
                    packet_value += evaluate_binary_usize(&bits);
                    // if let Some(limit) = length_limit {
                    //     if limit - steps_count < 8 {
                    //         (0..=limit - steps_count).for_each(|_| {
                    //             steps_count += 1;
                    //             iter.next();
                    //         });
                    //     }
                    // }
                    break;
                }
            }
        }
        5 => {
            if iter.next().unwrap() == &0 {
                let total_length = evaluate_binary(
                    &(0..15)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 15;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, Some(total_length - steps1));
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 > subpacket_value2) as usize;
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                // assert_eq!(num_of_11_bit_subpackets, 2);
                steps_count += 11;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 > subpacket_value2) as usize;
            }
        }
        6 => {
            if iter.next().unwrap() == &0 {
                let total_length = evaluate_binary(
                    &(0..15)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 15;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, Some(total_length - steps1));
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 < subpacket_value2) as usize;
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                // assert_eq!(num_of_11_bit_subpackets, 2);
                steps_count += 11;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 < subpacket_value2) as usize;
            }
        }
        7 => {
            if iter.next().unwrap() == &0 {
                let total_length = evaluate_binary(
                    &(0..15)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 15;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, Some(total_length - steps1));
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 == subpacket_value2) as usize;
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                // assert_eq!(num_of_11_bit_subpackets, 2);
                steps_count += 11;
                let (subpacket_value1, steps1, remaining_iter1) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter1;
                steps_count += steps1;
                let (subpacket_value2, steps2, remaining_iter2) =
                    evaluate_packet_internal(iter, None);
                iter = remaining_iter2;
                steps_count += steps2;
                packet_value += (subpacket_value1 == subpacket_value2) as usize;
            }
        }
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
                let mut internal_value = if [0, 3].contains(&type_id) {
                    0
                } else if [1].contains(&type_id) {
                    1
                } else {
                    usize::MAX
                };
                loop {
                    let (subpacket_value, steps, remaining_iter) =
                        evaluate_packet_internal(iter, Some(total_length - internal_steps_count));
                    if let Some(subpacket_value) = subpacket_value {
                        match type_id {
                            0 => internal_value += subpacket_value,
                            1 => internal_value *= subpacket_value,
                            2 => internal_value = internal_value.min(subpacket_value),
                            _ => internal_value = internal_value.max(subpacket_value),
                        }
                    }
                    iter = remaining_iter;
                    internal_steps_count += steps;
                    if total_length - internal_steps_count < 8 {
                        packet_value += internal_value;
                        steps_count += total_length;
                        break;
                    }
                }
            } else {
                let num_of_11_bit_subpackets = evaluate_binary(
                    &(0..11)
                        .filter_map(|_| iter.next())
                        .copied()
                        .collect::<Vec<_>>(),
                );
                steps_count += 11;
                let mut internal_value = if [0, 3].contains(&type_id) {
                    0
                } else if [1].contains(&type_id) {
                    1
                } else {
                    usize::MAX
                };
                let mut internal_steps = 0;
                for _ in 1..=num_of_11_bit_subpackets {
                    let (subpacket_value, steps, remaining_iter) =
                        evaluate_packet_internal(iter, None);
                    if let Some(subpacket_value) = subpacket_value {
                        match type_id {
                            0 => internal_value += subpacket_value,
                            1 => internal_value *= subpacket_value,
                            2 => internal_value = internal_value.min(subpacket_value),
                            _ => internal_value = internal_value.max(subpacket_value),
                        }
                    }
                    iter = remaining_iter;
                    internal_steps += steps;
                }
                steps_count += internal_steps;
                packet_value += internal_value;
            }
        }
    }

    (Some(packet_value), steps_count, iter)
}

#[cfg(test)]
mod tests {
    use crate::{convert_hex_to_bin, decode_packet, evaluate_binary, evaluate_packet};

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

    #[test]
    fn part2_test() {
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("C200B40A82").iter().peekable()).0,
            Some(3)
        );

        assert_eq!(
            evaluate_packet(convert_hex_to_bin("04005AC33890").iter().peekable()).0,
            Some(54)
        );
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("880086C3E88112").iter().peekable()).0,
            Some(7)
        );
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("CE00C43D881120").iter().peekable()).0,
            Some(9)
        );
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("D8005AC2A8F0").iter().peekable()).0,
            Some(1)
        );
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("F600BC2D8F").iter().peekable()).0,
            Some(0)
        );
        assert_eq!(
            evaluate_packet(convert_hex_to_bin("9C005AC2F8F0").iter().peekable()).0,
            Some(0)
        );
        assert_eq!(
            evaluate_packet(
                convert_hex_to_bin("9C0141080250320F1802104A08")
                    .iter()
                    .peekable()
            )
            .0,
            Some(1)
        );
    }
}
