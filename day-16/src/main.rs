use std::{fs::read_to_string, iter::Peekable};

fn main() {
    let hex = read("input.txt");
    let decoder = PacketDecoder::new(&hex);
    println!(
        "part1 solution: {:?}",
        decoder.packet.calculate_version_sum()
    );
    println!("part2 solution: {:?}", decoder.packet.evaluate_packet());
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    size: usize,
    value: Option<usize>,
    sub_packets: Vec<Packet>,
}
impl Packet {
    pub fn new() -> Self {
        Self {
            version: 0,
            type_id: 0,
            size: 0,
            value: None,
            sub_packets: vec![],
        }
    }

    pub fn set_version<'a, T>(&mut self, iter: &mut Peekable<T>)
    where
        T: Iterator<Item = &'a u8>,
    {
        self.version = PacketDecoder::evaluate_binary(iter, 3);
        self.size += 3;
    }

    pub fn set_type_id<'a, T>(&mut self, iter: &mut Peekable<T>)
    where
        T: Iterator<Item = &'a u8>,
    {
        self.type_id = PacketDecoder::evaluate_binary(iter, 3);
        self.size += 3;
    }

    pub fn set_value<'a, T>(&mut self, iter: &mut Peekable<T>)
    where
        T: Iterator<Item = &'a u8>,
    {
        let mut bits = vec![];
        loop {
            let is_last = iter.next().unwrap();
            bits.extend((0..4).filter_map(|_| iter.next()));
            self.size += 5;
            if is_last == &0 {
                self.value = Some(PacketDecoder::evaluate_binary(
                    &mut bits.iter().peekable(),
                    bits.len(),
                ));
                break;
            }
        }
    }

    pub fn calculate_version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|packet| packet.calculate_version_sum())
                .sum::<usize>()
    }

    pub fn evaluate_packet(&self) -> usize {
        match self.type_id {
            4 => self.value.unwrap(),
            5 => {
                (self.sub_packets[0].evaluate_packet() > self.sub_packets[1].evaluate_packet())
                    as usize
            }
            6 => {
                (self.sub_packets[0].evaluate_packet() < self.sub_packets[1].evaluate_packet())
                    as usize
            }
            7 => {
                (self.sub_packets[0].evaluate_packet() == self.sub_packets[1].evaluate_packet())
                    as usize
            }
            _ => self.sub_packets[1..].iter().fold(
                self.sub_packets[0].evaluate_packet(),
                |acc, packet| match self.type_id {
                    0 => acc + packet.evaluate_packet(),
                    1 => acc * packet.evaluate_packet(),
                    2 => acc.min(packet.evaluate_packet()),
                    _ => acc.max(packet.evaluate_packet()),
                },
            ),
        }
    }
}

#[derive(Debug)]
struct PacketDecoder {
    packet: Packet,
}
impl PacketDecoder {
    pub fn new(hex_transmission: &str) -> Self {
        let bits = convert_hex_to_bin(hex_transmission);
        let packet = Self::decode_packet(&mut bits.iter().peekable());
        Self { packet }
    }

    fn evaluate_binary<'a, T>(iter: &mut Peekable<T>, length: usize) -> usize
    where
        T: Iterator<Item = &'a u8>,
    {
        (0..length)
            .filter_map(|_| iter.next())
            .enumerate()
            .fold(0, |acc, (pos, bit)| {
                acc | (*bit as usize * (1 << (length - pos - 1)))
            })
    }

    fn decode_packet<'a, T>(iter: &mut Peekable<T>) -> Packet
    where
        T: Iterator<Item = &'a u8>,
    {
        let mut packet = Packet::new();
        packet.set_version(iter);
        packet.set_type_id(iter);
        match packet.type_id {
            4 => packet.set_value(iter),
            _ => match iter.next() {
                Some(0) => {
                    let total_length = PacketDecoder::evaluate_binary(iter, 15);
                    packet.size += 15;
                    let mut internal_steps_count = 0;
                    loop {
                        let subpacket = Self::decode_packet(iter);
                        internal_steps_count += subpacket.size;
                        packet.sub_packets.push(subpacket);
                        if total_length - internal_steps_count < 8 {
                            packet.size += total_length;
                            break;
                        }
                    }
                }
                Some(1) => {
                    let num_of_11_bit_subpackets = PacketDecoder::evaluate_binary(iter, 11);
                    packet.size += 11;
                    for _ in 0..num_of_11_bit_subpackets {
                        let subpacket = Self::decode_packet(iter);
                        packet.size += subpacket.size;
                        packet.sub_packets.push(subpacket);
                    }
                }
                _ => unreachable!(),
            },
        }
        packet
    }
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
    use crate::{convert_hex_to_bin, PacketDecoder};

    #[test]
    fn test_evaluate() {
        assert_eq!(
            PacketDecoder::evaluate_binary(&mut [1, 0, 1].iter().peekable(), 3),
            5
        );
        assert_eq!(
            PacketDecoder::evaluate_binary(&mut [1, 0, 0].iter().peekable(), 3),
            4
        );
    }

    #[test]
    fn test_convert_hex_to_bin() {
        let decoder = PacketDecoder::new("D2FE28");
        assert_eq!(
            convert_hex_to_bin("D2FE28"),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
        assert_eq!(decoder.packet.calculate_version_sum(), 6);

        let decoder = PacketDecoder::new("38006F45291200");
        assert_eq!(
            convert_hex_to_bin("38006F45291200"),
            vec![
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0,
                0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(decoder.packet.calculate_version_sum(), 9);

        let decoder = PacketDecoder::new("EE00D40C823060");
        assert_eq!(
            convert_hex_to_bin("EE00D40C823060"),
            vec![
                1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
                1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(decoder.packet.calculate_version_sum(), 14);
    }

    #[test]
    fn part1_test() {
        let decoder = PacketDecoder::new("8A004A801A8002F478");
        assert_eq!(decoder.packet.calculate_version_sum(), 16);
        let decoder = PacketDecoder::new("620080001611562C8802118E34");
        assert_eq!(decoder.packet.calculate_version_sum(), 12);
        let decoder = PacketDecoder::new("C0015000016115A2E0802F182340");
        assert_eq!(decoder.packet.calculate_version_sum(), 23);
        let decoder = PacketDecoder::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(decoder.packet.calculate_version_sum(), 31);
    }

    #[test]
    fn part2_test() {
        let decoder = PacketDecoder::new("C200B40A82");
        assert_eq!(decoder.packet.evaluate_packet(), 3);
        let decoder = PacketDecoder::new("04005AC33890");
        assert_eq!(decoder.packet.evaluate_packet(), 54);
        let decoder = PacketDecoder::new("880086C3E88112");
        assert_eq!(decoder.packet.evaluate_packet(), 7);
        let decoder = PacketDecoder::new("CE00C43D881120");
        assert_eq!(decoder.packet.evaluate_packet(), 9);
        let decoder = PacketDecoder::new("D8005AC2A8F0");
        assert_eq!(decoder.packet.evaluate_packet(), 1);
        let decoder = PacketDecoder::new("F600BC2D8F");
        assert_eq!(decoder.packet.evaluate_packet(), 0);
        let decoder = PacketDecoder::new("9C005AC2F8F0");
        assert_eq!(decoder.packet.evaluate_packet(), 0);
        let decoder = PacketDecoder::new("9C0141080250320F1802104A08");
        assert_eq!(decoder.packet.evaluate_packet(), 1);
    }
}
