//! This is my solution for [Advent of Code - Day 16 - _Packet Decoder_](https://adventofcode.com/2021/day/16)
//!
//! After yesterday's very technical/theory heavy solution today was working through a fairly
//! complex specification, but the face-value implementation was efficient once all the parts were
//! in place.
//!
//! The bulk of the work is in parsing the input into the hierarchy of packets, [`parse_input`].
//! This is the entry point for a number of functions that are involved in the parsing process.
//! [`to_bits`] is a bit clunky, but returns the bits as a `Vec<bool>` in reverse order so that the
//! bits can be consumed with [`Vec::pop`] which is much more efficient than taking them from the
//! head of the `Vec`. [`take_bits`] consumes a specified number of bits from the tail, interpreting
//! them as a number. [`parse_packet`] consumes the version and [`PacketType`], then delegates to
//! [`parse_literal`] and [`parse_sub_packets`] based on the type. Each uses [`take_bits`] as
//! appropriate to consume and interpret the required bits according to the spec, and keeps track of
//! bits consumed to report back to any parent operation packet that is reading in bit length mode.
//!
//! Once that was done both part one [`Packet::version_sum`], and part two [`Packet::compute`]
//! recursively walk the packet tree compiling the appropriate solution.
use std::fs;

/// The eight possible packet types
#[derive(Eq, PartialEq, Debug)]
enum PacketType {
    /// Operation: Sum all contained packets
    Sum,
    /// Operation: Multiply all contained packets
    Product,
    /// Operation: Return the minimum of all contained packets
    Min,
    /// Operation: Return the maximum of all contained packets
    Max,
    /// Literal: This packet represents a literal number value
    Literal,
    /// Operation: Compare the first and only two sub packets, returns `1` if first is greater than
    /// second, `0` otherwise.
    GreaterThan,
    /// Operation: Compare the first and only two sub packets, returns `1` if first is less than
    /// second, `0` otherwise.
    LessThan,
    /// Operation: Compare the first and only two sub packets, returns `1` if first is equal to
    /// second, `0` otherwise.
    Equal,
}

impl From<usize> for PacketType {
    fn from(num: usize) -> Self {
        match num {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::Equal,
            _ => panic!("Invalid packet type {}", num),
        }
    }
}

/// Represents a packet in BITS
#[derive(Eq, PartialEq, Debug)]
struct Packet {
    /// The version (0-7)
    version: usize,
    /// Indicates what this packet represents
    packet_type: PacketType,
    /// List of sub-packets. For PacketType::Literal this will be empty
    sub_packets: Vec<Packet>,
    /// The literal value, for PacketTypes other than PacketType::Literal
    value: usize,
}

impl Packet {
    /// create a packet representing an operation on sub packets
    #[cfg(test)]
    fn new_operator(version: usize, packet_type: PacketType, sub_packets: Vec<Packet>) -> Packet {
        Packet {
            version,
            packet_type,
            sub_packets,
            value: 0,
        }
    }

    /// Create a packet representing a literal number
    #[cfg(test)]
    fn new_literal(version: usize, value: usize) -> Packet {
        Packet {
            version,
            packet_type: PacketType::Literal,
            sub_packets: Vec::new(),
            value,
        }
    }

    /// Solution to part one. Returns the sum of this packet's version and the version sum of all
    /// sub-packets
    fn version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(Packet::version_sum)
                .sum::<usize>()
    }

    /// Solution to part two. Recursively compute the value of applying the current operation to the
    /// contained sub-packets' computed values, or return the value in the case of a literal node.
    fn compute(&self) -> usize {
        match self.packet_type {
            PacketType::Sum => self.sub_packets.iter().map(Packet::compute).sum(),
            PacketType::Product => self.sub_packets.iter().map(Packet::compute).product(),
            PacketType::Min => self.sub_packets.iter().map(Packet::compute).min().unwrap(),
            PacketType::Max => self.sub_packets.iter().map(Packet::compute).max().unwrap(),
            PacketType::Literal => self.value,
            PacketType::GreaterThan => {
                (self.sub_packets[0].compute() > self.sub_packets[1].compute()) as usize
            }
            PacketType::LessThan => {
                (self.sub_packets[0].compute() < self.sub_packets[1].compute()) as usize
            }
            PacketType::Equal => {
                (self.sub_packets[0].compute() == self.sub_packets[1].compute()) as usize
            }
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

/// Parse a hexadecimal string as a sequence of bits. The returned list is reversed for ease of
/// consuming the bits via [`Vec::pop`].
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

/// Consume the last `count` bits from the end of the provided vector, interpreting them as a binary
/// representation of a usize.
fn take_bits(bits: &mut Vec<bool>, count: usize) -> usize {
    let mut out: usize = 0;
    for _ in 0..count {
        // Shift the next bit onto the left
        out = (out << 1) + (bits.pop().unwrap() as usize)
    }

    out
}

/// Parse the section of a literal packet representing the number. This will be in chunks of 5 bits,
/// the first being a flag that indicates if parsing should continue after this chunk, the next four
/// being the next four bits in the number. Once the continue flag is `0` indicating this is the
/// final chunk, all four-bit sections should be concatenated and interpreted as the binary
/// representation of a usize. Returns the value and number of bits consumed.
fn parse_literal(mut bits: &mut Vec<bool>) -> (usize, usize) {
    let mut value = 0;
    let mut bit_count = 0;

    loop {
        // Consume the next continue flag
        let last = take_bits(&mut bits, 1) == 0;
        // Shift the next four bits left from the bit stream.
        value = (value << 4) + take_bits(&mut bits, 4);
        bit_count += 5;
        if last {
            break;
        }
    }

    (value, bit_count)
}

/// Parse the sub-packets section of an operation packet.
/// 1. Consume one bit indicating the mode of consuming sub packets
///     * If `0` consume the next 15 bits as a bit length
///     * If `1` consume the nect 11 bits as a packet count
/// 2. Consume one sub-packet at a time using [`parse_packet`].
///     * Decrement the bit counter by the number of bits consumed, or the packet counter by `1` as
///       each packet is consumed.
///     * Keep a running total of bits consumed.
/// 3. Return the list of parsed packets, and the total bits consumed
fn parse_sub_packets(mut bits: &mut Vec<bool>) -> (Vec<Packet>, usize) {
    let mut bit_count: usize = 0;
    let mut sub_packets = Vec::new();

    let length_is_bits = take_bits(&mut bits, 1) == 0;
    bit_count += 1;

    if length_is_bits {
        let mut bits_to_take = take_bits(&mut bits, 15);
        bit_count += 15;

        while bits_to_take > 0 {
            let (sub_packet, bit_length) = parse_packet(&mut bits);
            sub_packets.push(sub_packet);
            bit_count += bit_length;
            bits_to_take -= bit_length;
        }
    } else {
        let mut packets_to_take = take_bits(&mut bits, 11);
        bit_count += 11;

        while packets_to_take > 0 {
            let (sub_packet, bit_length) = parse_packet(&mut bits);
            sub_packets.push(sub_packet);
            bit_count += bit_length;
            packets_to_take -= 1;
        }
    }
    (sub_packets, bit_count)
}

/// Read the packet header (version: 3 bits, type: 3 bits). Then based of the type delegate the
/// parsing of the payload to either [`parse_literal`] or [`parse_sub_packets`]. Return the parsed
/// [`Packet`] and number of bits consumed
fn parse_packet(mut bits: &mut Vec<bool>) -> (Packet, usize) {
    let version = take_bits(bits, 3);
    let packet_type = PacketType::from(take_bits(bits, 3));
    let root_bit_count = 6usize;
    if packet_type == PacketType::Literal {
        let (value, literal_bit_count) = parse_literal(&mut bits);
        (
            Packet {
                version,
                packet_type,
                sub_packets: Vec::new(),
                value,
            },
            root_bit_count + literal_bit_count,
        )
    } else {
        let (sub_packets, sub_bit_count) = parse_sub_packets(&mut bits);
        (
            Packet {
                version,
                packet_type,
                sub_packets,
                value: 0,
            },
            root_bit_count + sub_bit_count,
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
    use crate::day_16::{parse_input, take_bits, to_bits, Packet, PacketType};

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
                PacketType::LessThan,
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
                PacketType::Max,
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
