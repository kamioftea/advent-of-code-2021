//! This is my solution for [Advent of Code - Day 16 - _Title_](https://adventofcode.com/2021/day/16)
//!
//!

use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: usize,
    packet_type: usize,
    sub_packets: Vec<Packet>,
    value: usize,
}

impl Packet {
    #[cfg(test)]
    fn new_operator(version: usize, packet_type: usize, sub_packets: Vec<Packet>) -> Packet {
        Packet {
            version,
            packet_type,
            sub_packets,
            value: 0,
        }
    }

    #[cfg(test)]
    fn new_literal(version: usize, value: usize) -> Packet {
        Packet {
            version,
            packet_type: 4,
            sub_packets: Vec::new(),
            value,
        }
    }

    fn version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(Packet::version_sum)
                .sum::<usize>()
    }

    fn compute(&self) -> usize {
        match self.packet_type {
            0 => self.sub_packets.iter().map(Packet::compute).sum(),
            1 => self.sub_packets.iter().map(Packet::compute).product(),
            2 => self.sub_packets.iter().map(Packet::compute).min().unwrap(),
            3 => self.sub_packets.iter().map(Packet::compute).max().unwrap(),
            4 => self.value,
            5 => (self.sub_packets[0].compute() > self.sub_packets[1].compute()) as usize,
            6 => (self.sub_packets[0].compute() < self.sub_packets[1].compute()) as usize,
            7 => (self.sub_packets[0].compute() == self.sub_packets[1].compute()) as usize,
            _ => 0,
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-16-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 16.
pub fn run() {
    let contents = fs::read_to_string("res/day-16-input").expect("Failed to read file");
    let root = parse_input(&contents);

    println!("The version sum is: {}", root.version_sum());
    println!("The result of the operation is: {}", root.compute());
}

fn to_bits(input: &String) -> Vec<bool> {
    input
        .chars()
        .flat_map(|c| {
            c.to_digit(16)
                .iter()
                .flat_map(|&num| vec![num & 8 == 8, num & 4 == 4, num & 2 == 2, num & 1 == 1])
                .collect::<Vec<bool>>()
        })
        .rev()
        .collect()
}

fn take_bits(bits: &mut Vec<bool>, count: usize) -> usize {
    let mut out: usize = 0;
    for _ in 0..count {
        out = (out << 1) + (bits.pop().unwrap() as usize)
    }

    out
}

fn parse_literal(mut bits: &mut Vec<bool>) -> (usize, usize) {
    let mut value = 0;
    let mut bit_count = 0;

    loop {
        let last = take_bits(&mut bits, 1) == 0;
        value = (value << 4) + take_bits(&mut bits, 4);
        bit_count += 5;
        if last {
            break;
        }
    }

    (value, bit_count)
}

fn parse_packet(mut bits: &mut Vec<bool>) -> (Packet, usize) {
    let version = take_bits(bits, 3);
    let packet_type = take_bits(bits, 3);
    let mut taken = 6usize;
    if packet_type == 4 {
        let (value, bit_count) = parse_literal(&mut bits);
        (
            Packet {
                version,
                packet_type,
                sub_packets: Vec::new(),
                value,
            },
            taken + bit_count,
        )
    } else {
        let length_is_bits = take_bits(&mut bits, 1) == 0;
        let mut sub_packets = Vec::new();
        taken += 1;
        if length_is_bits {
            let mut bits_to_take = take_bits(&mut bits, 15);
            taken = taken + 15;

            while bits_to_take > 0 {
                let (sub_packet, bit_length) = parse_packet(&mut bits);
                sub_packets.push(sub_packet);
                taken += bit_length;
                bits_to_take -= bit_length;
            }
        } else {
            let mut packets_to_take = take_bits(&mut bits, 11);
            taken = taken + 11;

            while packets_to_take > 0 {
                let (sub_packet, bit_length) = parse_packet(&mut bits);
                sub_packets.push(sub_packet);
                taken += bit_length;
                packets_to_take -= 1;
            }
        }

        (
            Packet {
                version,
                packet_type,
                sub_packets,
                value: 0,
            },
            taken,
        )
    }
}

fn parse_input(input: &String) -> Packet {
    let mut bits: Vec<bool> = to_bits(input);
    let (packet, _) = parse_packet(&mut bits);
    packet
}

#[cfg(test)]
mod tests {
    use crate::day_16::{parse_input, take_bits, to_bits, Packet};

    fn sample_literal() -> Vec<bool> {
        "110100101111111000101000"
            .chars()
            .map(|c| c == '1')
            .rev()
            .collect::<Vec<bool>>()
    }

    #[test]
    fn can_parse_to_bits() {
        assert_eq!(to_bits(&"D2FE28".to_string()), sample_literal());
    }

    #[test]
    fn can_take_bits() {
        let mut bits: Vec<bool> = sample_literal();
        assert_eq!(take_bits(&mut bits, 3), 6usize);
        assert_eq!(take_bits(&mut bits, 3), 4usize);
        assert_eq!(take_bits(&mut bits, 1), 1usize);
        assert_eq!(take_bits(&mut bits, 4), 7usize);
        assert_eq!(take_bits(&mut bits, 1), 1usize);
        assert_eq!(take_bits(&mut bits, 4), 14usize);
        assert_eq!(take_bits(&mut bits, 1), 0usize);
        assert_eq!(take_bits(&mut bits, 4), 5usize);
    }

    #[test]
    fn can_parse_literal() {
        assert_eq!(
            parse_input(&"D2FE28".to_string()),
            Packet::new_literal(6, 2021)
        )
    }

    #[test]
    fn can_parse_operator_with_bit_length() {
        assert_eq!(
            parse_input(&"38006F45291200".to_string()),
            Packet::new_operator(
                1,
                6,
                Vec::from([Packet::new_literal(6, 10), Packet::new_literal(2, 20)])
            )
        )
    }

    #[test]
    fn can_parse_operator_with_packet_length() {
        assert_eq!(
            parse_input(&"EE00D40C823060".to_string()),
            Packet::new_operator(
                7,
                3,
                Vec::from([
                    Packet::new_literal(2, 1),
                    Packet::new_literal(4, 2),
                    Packet::new_literal(1, 3),
                ])
            )
        )
    }

    #[test]
    fn can_sum_versions() {
        assert_eq!(
            parse_input(&"8A004A801A8002F478".to_string()).version_sum(),
            16
        );
        assert_eq!(
            parse_input(&"620080001611562C8802118E34".to_string()).version_sum(),
            12
        );
        assert_eq!(
            parse_input(&"C0015000016115A2E0802F182340".to_string()).version_sum(),
            23
        );
        assert_eq!(
            parse_input(&"A0016C880162017C3686B18A3D4780".to_string()).version_sum(),
            31
        );
    }

    #[test]
    fn can_compute() {
        assert_eq!(parse_input(&"C200B40A82".to_string()).compute(), 3);
        assert_eq!(parse_input(&"04005AC33890".to_string()).compute(), 54);
        assert_eq!(parse_input(&"880086C3E88112".to_string()).compute(), 7);
        assert_eq!(parse_input(&"CE00C43D881120".to_string()).compute(), 9);
        assert_eq!(parse_input(&"D8005AC2A8F0".to_string()).compute(), 1);
        assert_eq!(parse_input(&"F600BC2D8F".to_string()).compute(), 0);
        assert_eq!(parse_input(&"9C005AC2F8F0".to_string()).compute(), 0);
        assert_eq!(
            parse_input(&"9C0141080250320F1802104A08".to_string()).compute(),
            1
        );
    }
}
