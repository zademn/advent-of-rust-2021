use std::vec::IntoIter;

use crate::utils::read_challenge_data;

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType,
    //num_bits: u64,
}
impl Packet {
    /// First read the version and type ID then decide what to read next based on type_id
    fn read_packet(bv_iter: &mut IntoIter<u8>) -> Self {
        let version = n_bits(bv_iter, 3).unwrap() as u8; // bv_iter.n_bits(3) as u8;
        let type_id = n_bits(bv_iter, 3).unwrap() as u8; //bv_iter.n_bits(3) as u8;
        let packet_type = if type_id == 4 {
            PacketType::LiteralPacketEnum(LiteralPacket::from_iter(bv_iter))
        } else {
            PacketType::OperatorPacketEnum(OperatorPacket::from_iter(bv_iter))
        };
        Self {
            version,
            type_id,
            packet_type,
        }
    }

    fn version_sum(&self) -> u64 {
        let mut v = self.version as u64;
        if let PacketType::OperatorPacketEnum(op_packet) = &self.packet_type {
            for p in op_packet.subpackets.iter() {
                v += p.version_sum();
            }
        }
        v
    }

    /// Gets the number of bits from the current package. It recurses down if it's an operator package
    fn num_bits(&self) -> u64 {
        let mut n = 0;
        n += match &self.packet_type {
            PacketType::LiteralPacketEnum(p) => p.num_bits(),
            PacketType::OperatorPacketEnum(p) => p.num_bits(),
        };
        n
    }
    /// Computes the value as required
    fn value(&self) -> u64 {
        assert!(self.type_id <= 7);
        match &self.packet_type {
            PacketType::LiteralPacketEnum(p) => p.value,
            PacketType::OperatorPacketEnum(p) => match self.type_id {
                0 => p.subpackets.iter().fold(0, |acc, e| acc + e.value()),
                1 => p.subpackets.iter().fold(1, |acc, e| acc * e.value()),
                2 => p
                    .subpackets
                    .iter()
                    .fold(u64::MAX, |acc, e| acc.min(e.value())),
                3 => p.subpackets.iter().fold(0, |acc, e| acc.max(e.value())),
                5 => (p.subpackets[0].value() > p.subpackets[1].value()) as u64,
                6 => (p.subpackets[0].value() < p.subpackets[1].value()) as u64,
                7 => (p.subpackets[0].value() == p.subpackets[1].value()) as u64,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Clone, Debug)]
enum PacketType {
    LiteralPacketEnum(LiteralPacket),
    OperatorPacketEnum(OperatorPacket),
}

#[derive(Clone, Debug)]
struct LiteralPacket {
    value: u64,
    num_bits: u64,
}

impl LiteralPacket {
    fn from_iter(bv_iter: &mut IntoIter<u8>) -> Self {
        let mut num_bits = 6; // version and type_id
        let mut hex_digits = Vec::new();
        loop {
            let bits = n_bits(bv_iter, 5).unwrap(); // bv_iter.n_bits(5);
            hex_digits.push(bits & 0xf);
            num_bits += 5;
            if bits & 0x10 == 0 {
                break;
            }
        }
        let mut value = 0;
        for (i, &d) in hex_digits.iter().rev().enumerate() {
            value += (d as u64) << (i * 4);
        }
        Self { num_bits, value }
    }
    fn num_bits(&self) -> u64 {
        self.num_bits
    }
}

#[derive(Clone, Debug)]
struct OperatorPacket {
    length_type_id: u8,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    fn from_iter(bv_iter: &mut IntoIter<u8>) -> Self {
        let length_type_id = n_bits(bv_iter, 1).unwrap() as u8; //bv_iter.n_bits(1) as u8;
        let mut subpackets: Vec<Packet> = Vec::new();
        if length_type_id == 0 {
            let mut total_length = n_bits(bv_iter, 15).unwrap(); // bv_iter.n_bits(15);
            while total_length > 0 {
                let p = Packet::read_packet(bv_iter);
                total_length -= p.num_bits();
                subpackets.push(p);
            }
        } else {
            let mut num_sub_packets = n_bits(bv_iter, 11).unwrap(); // bv_iter.n_bits(11);
            while num_sub_packets > 0 {
                let p = Packet::read_packet(bv_iter);
                num_sub_packets -= 1;
                subpackets.push(p);
            }
        }
        Self {
            length_type_id,
            subpackets,
        }
    }

    fn num_bits(&self) -> u64 {
        let mut n = if self.length_type_id == 0 {
            7 + 15
        } else {
            7 + 11
        };
        for p in self.subpackets.iter() {
            n += match &p.packet_type {
                PacketType::LiteralPacketEnum(p) => p.num_bits(),
                PacketType::OperatorPacketEnum(p) => p.num_bits(),
            };
        }
        n
    }
}

#[derive(Clone, Debug)]
struct BitVec {
    inner: Vec<u8>,
}
impl BitVec {
    #[allow(unused)]
    fn from_hex(a: &str) -> Self {
        let a = a.chars().map(|e| e.to_digit(16).unwrap() as u8).collect();
        Self { inner: a }
    }
}
impl IntoIterator for BitVec {
    type Item = u8;
    type IntoIter = BitVecIter;
    fn into_iter(self) -> Self::IntoIter {
        BitVecIter {
            iter: self.inner.into_iter(),
            curr_bit: 4,
            current_value: None,
        }
    }
}
struct BitVecIter {
    iter: IntoIter<u8>,
    curr_bit: usize,
    current_value: Option<u8>,
}
impl BitVecIter {
    #[allow(unused)]
    fn n_bits(&mut self, n: usize) -> u64 {
        let mut v = 0;
        for (i, b) in self.take(n).enumerate() {
            v += (b as u64) << (n - 1 - i);
        }
        v
    }
}
impl Iterator for BitVecIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_bit == 4 {
            self.current_value = self.iter.next();
            self.curr_bit = 0;
        }
        if let Some(h) = self.current_value {
            let v = (h >> (3 - self.curr_bit)) & 1;
            self.curr_bit += 1;
            return Some(v);
        }
        None
    }
}

fn n_bits(iter: &mut IntoIter<u8>, n: usize) -> Option<u64> {
    if iter.size_hint().0 < n {
        return None;
    }
    let mut v = 0;
    for (i, b) in iter.take(n).enumerate() {
        v += (b as u64) << (n - 1 - i);
    }
    Some(v)
}
fn to_bit_vector(s: &str) -> Vec<u8> {
    let mut bv: Vec<u8> = Vec::new();
    let s: Vec<u8> = s.chars().map(|e| e.to_digit(16).unwrap() as u8).collect();
    for h in s {
        for i in 0..4 {
            let v = (h >> (3 - i)) & 1;
            bv.push(v);
        }
    }
    bv
}
/// first 3 bits = type ID
/// type ID 4 = litteral value -- single binary number.
/// -  Pad with 0 until length is multiple of 4
/// - Break up into groups of 4 bits
/// - Each groups is prefixed by 1 except the last one which is prefixed by 0
/// type ID != 4 => operator
/// - Has a length type id
///     - 0 -> 15 bits = total length of the subpackets
///     - 1 -> 11 bits = number of sub packets contained
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(16, run_example);

    let bv = to_bit_vector(&input);
    let mut bv_iter = bv.into_iter();
    //println!("{:?}", bv_iter);
    //let bv = BitVec::from_hex(&input);
    //println!("{:?}", bv);
    // let mut bv_iter = bv.into_iter();
    let p = Packet::read_packet(&mut bv_iter);
    println!("{:?}", p);
    if part1 {
        println!("{}", p.version_sum()); // 901
        p.version_sum() as usize
    } else {
        println!("{}", p.value()); // 110434737925
        p.value() as usize
    }

    //println!("After {:?}", bv_iter);
}
#[cfg(test)]
mod tests {
    use super::solve;
    // #[test]
    // // fn test_example() {
    // //     assert_eq!(solve(true, true), 31);
    // //     assert_eq!(solve(true, false), 31);
    // // }
    #[test]
    fn test_problem() {
        assert_eq!(solve(false, true), 901);
        assert_eq!(solve(false, false), 110434737925);
    }
}
