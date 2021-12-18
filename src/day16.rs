use aoc_runner_derive::{aoc, aoc_generator};

use bitvec::prelude::*;

type Slice = BitSlice<Msb0, u8>;

#[aoc_generator(day16)]
pub fn to_bitstream(input: &str) -> Vec<u8> {
    hex::decode(input).expect("not hex")
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    contents: PacketContents,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketContents {
    Literal(i64),
    Operator(u8, Vec<Packet>),
}

fn parse3(bits: &mut &Slice) -> u8 {
    let triad = bits
        .iter()
        .take(3)
        .fold(0u8, |acc, b| (acc << 1) | (*b as u8));
    *bits = &bits[3..];
    triad
}

fn parse_literal(bits: &mut &Slice) -> i64 {
    let mut literal = 0i64;
    loop {
        let should_continue = bits[0];
        let chunk = bits[1..5].iter().fold(0u8, |acc, b| (acc << 1) | *b as u8);
        literal <<= 4;
        literal |= chunk as i64;
        *bits = &bits[5..];

        if !should_continue {
            break;
        }
    }
    literal
}

fn parse_packet(bits: &mut &Slice) -> Packet {
    let version = parse3(bits);
    let kind = parse3(bits);
    let contents = match kind {
        4 => PacketContents::Literal(parse_literal(bits)),
        op => PacketContents::Operator(op, parse_subpackets(bits)),
    };

    Packet { version, contents }
}

// An operator packet contains one or more packets.
// To indicate which subsequent binary data represents its sub-packets,
// an operator packet can use one of two modes indicated by the bit immediately
// after the packet header; this is called the length type ID:
//
// - If the length type ID is 0, then the next 15 bits are a number
//   that represents the total length in bits of the sub-packets contained
//   by this packet.
//
// - If the length type ID is 1, then the next 11 bits are a number
//    that represents the number of sub-packets immediately contained by this packet.
//
fn parse_subpackets(bits: &mut &Slice) -> Vec<Packet> {
    let length_type = bits[0];
    *bits = &bits[1..];

    let mut subpackets = Vec::new();

    if !length_type {
        let num_subpacket_bits = bits[0..15]
            .iter()
            .fold(0usize, |acc, b| (acc << 1) | *b as usize);
        *bits = &bits[15..];

        let mut subpacket_bits = &bits[..num_subpacket_bits];
        while !subpacket_bits.is_empty() {
            subpackets.push(parse_packet(&mut subpacket_bits));
        }
        *bits = &bits[num_subpacket_bits..];
    } else {
        let num_subpackets = bits[0..11]
            .iter()
            .fold(0u16, |acc, b| (acc << 1) | *b as u16);
        *bits = &bits[11..];

        subpackets.reserve(num_subpackets as usize);

        for _ in 0..num_subpackets {
            subpackets.push(parse_packet(bits));
        }
    }

    subpackets
}

fn sum_versions(packet: &Packet) -> i64 {
    static EMPTY: Vec<Packet> = vec![];

    let children: &Vec<_> = match &packet.contents {
        PacketContents::Literal(_) => &EMPTY,
        PacketContents::Operator(_, subs) => subs,
    };

    packet.version as i64
        + children
            .iter()
            .fold(0i64, |acc, c| acc + sum_versions(c) as i64)
}

#[aoc(day16, part1)]
pub fn part1(bytes: &[u8]) -> i64 {
    let mut bits = bytes.view_bits::<Msb0>();
    let packet = parse_packet(&mut bits);
    sum_versions(&packet)
}

fn eval(packet: &Packet) -> i64 {
    match &packet.contents {
        PacketContents::Literal(l) => *l,
        PacketContents::Operator(op, subs) => eval_operator(*op, subs),
    }
}

fn eval_operator(op: u8, subpackets: &[Packet]) -> i64 {
    match op {
        // Packets with type ID 0 are sum packets - their value is the sum
        // of the values of their sub-packets. If they only have a single
        // sub-packet, their value is the value of the sub-packet.
        0 => subpackets.iter().fold(0i64, |acc, s| acc + eval(s)),

        // Packets with type ID 1 are product packets - their value is the result
        // of multiplying together the values of their sub-packets. If they only
        // have a single sub-packet, their value is the value of the sub-packet.
        1 => subpackets.iter().fold(1i64, |acc, s| acc * eval(s)),

        // Packets with type ID 2 are minimum packets - their value is the
        // minimum of the values of their sub-packets.
        2 => subpackets
            .iter()
            .fold(i64::MAX, |acc, s| std::cmp::min(acc, eval(s))),

        // Packets with type ID 3 are maximum packets - their value is the
        // maximum of the values of their sub-packets
        3 => subpackets
            .iter()
            .fold(i64::MIN, |acc, s| std::cmp::max(acc, eval(s))),

        // Packets with type ID 5 are greater than packets - their value is 1
        // if the value of the first sub-packet is greater than the value of
        // the second sub-packet; otherwise, their value is 0.
        // These packets always have exactly two sub-packets.
        5 => {
            assert_eq!(subpackets.len(), 2);
            (eval(&subpackets[0]) > eval(&subpackets[1])) as i64
        }

        // Packets with type ID 6 are less than packets - their value is 1
        // if the value of the first sub-packet is less than the value of
        // the second sub-packet; otherwise, their value is 0.
        // These packets always have exactly two sub-packets.
        6 => {
            assert_eq!(subpackets.len(), 2);
            (eval(&subpackets[0]) < eval(&subpackets[1])) as i64
        }

        // Packets with type ID 7 are equal to packets - their value is 1
        // if the value of the first sub-packet is equal to the value of
        // the second sub-packet; otherwise, their value is 0.
        // These packets always have exactly two sub-packets.
        7 => {
            assert_eq!(subpackets.len(), 2);
            (eval(&subpackets[0]) == eval(&subpackets[1])) as i64
        }

        op => panic!("Unexpected op {}", op),
    }
}

#[aoc(day16, part2)]
pub fn part2(bytes: &[u8]) -> i64 {
    let mut bits = bytes.view_bits::<Msb0>();
    let packet = parse_packet(&mut bits);
    eval(&packet)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse3() {
        let mut bits = [0xD2].view_bits::<Msb0>();
        assert_eq!(parse3(&mut bits), 6);
        assert_eq!(parse3(&mut bits), 4);
    }

    #[test]
    fn test_parse_literal() {
        let mut bits = [0xD2, 0xFE, 0x28].view_bits::<Msb0>();
        assert_eq!(
            parse_packet(&mut bits),
            Packet {
                version: 6,
                contents: PacketContents::Literal(2021)
            }
        );
    }

    fn hex_to_packet(hex: &str) -> Packet {
        let bytes = hex::decode(hex).unwrap();
        let mut bits = bytes.view_bits::<Msb0>();
        parse_packet(&mut bits)
    }

    #[test]
    fn test_length_type_id_0() {
        let packet = hex_to_packet("38006F45291200");
        assert_eq!(packet.version, 1);

        let (op, subpackets) = match packet.contents {
            PacketContents::Operator(op, subs) => (op, subs),
            _ => panic!("Not an operator"),
        };
        assert_eq!(op, 6);
        assert_eq!(subpackets.len(), 2);
        assert_eq!(
            subpackets[0],
            Packet {
                version: 6,
                contents: PacketContents::Literal(10)
            }
        );
        assert_eq!(
            subpackets[1],
            Packet {
                version: 2,
                contents: PacketContents::Literal(20)
            }
        );
    }

    #[test]
    fn test_length_type_id_1() {
        let packet = hex_to_packet("EE00D40C823060");
        assert_eq!(packet.version, 7);

        let (op, subpackets) = match packet.contents {
            PacketContents::Operator(op, subs) => (op, subs),
            _ => panic!("Not an operator"),
        };
        assert_eq!(op, 3);
        assert_eq!(subpackets.len(), 3);
        assert_eq!(
            subpackets[0],
            Packet {
                version: 2,
                contents: PacketContents::Literal(1)
            }
        );
        assert_eq!(
            subpackets[1],
            Packet {
                version: 4,
                contents: PacketContents::Literal(2)
            }
        );
        assert_eq!(
            subpackets[2],
            Packet {
                version: 1,
                contents: PacketContents::Literal(3)
            }
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_sums() {
        assert_eq!(sum_versions(&hex_to_packet("8A004A801A8002F478")), 16);
        assert_eq!(sum_versions(&hex_to_packet("620080001611562C8802118E34")), 12);
        assert_eq!(sum_versions(&hex_to_packet("C0015000016115A2E0802F182340")), 23);
        assert_eq!(sum_versions(&hex_to_packet("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_eval() {
        // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
        assert_eq!(eval(&hex_to_packet("C200B40A82")), 3);

        // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
        assert_eq!(eval(&hex_to_packet("04005AC33890")), 54);

        // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
        assert_eq!(eval(&hex_to_packet("880086C3E88112")), 7);

        // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
        assert_eq!(eval(&hex_to_packet("CE00C43D881120")), 9);

        // D8005AC2A8F0 produces 1, because 5 is less than 15.
        assert_eq!(eval(&hex_to_packet("D8005AC2A8F0")), 1);

        // F600BC2D8F produces 0, because 5 is not greater than 15.
        assert_eq!(eval(&hex_to_packet("F600BC2D8F")), 0);

        // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
        assert_eq!(eval(&hex_to_packet("9C005AC2F8F0")), 0);

        // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
        assert_eq!(eval(&hex_to_packet("9C0141080250320F1802104A08")), 1);
    }
}
