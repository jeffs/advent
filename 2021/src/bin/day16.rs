use advent2021::ParseError;
use std::fs::File;
use std::io::Read as _;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

pub struct Puzzle {
    bits: Vec<bool>,
}

impl Puzzle {
    pub fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut text = String::new();
        File::open(input)?.read_to_string(&mut text)?;
        Ok(text.trim_end().parse()?)
    }

    #[cfg(test)]
    fn bit_string(&self) -> String {
        let bytes: Vec<_> = self.bits.iter().map(|&b| b'0' + b as u8).collect();
        String::from_utf8_lossy(&bytes).to_string()
    }
}

impl FromStr for Puzzle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::new();
        for (i, _) in s.chars().enumerate() {
            let byte = u8::from_str_radix(&s[i..(i + 1)], 16)?;
            for j in [3, 2, 1, 0] {
                bits.push(byte >> j & 1 != 0);
            }
        }
        Ok(Puzzle { bits })
    }
}

fn try_parse_bits<I>(bits: &mut I, count: usize) -> Option<usize>
where
    I: Iterator<Item = bool>,
{
    let mut result = 0;
    for _ in 0..count {
        if let Some(bit) = bits.next() {
            result = result << 1 | bit as usize;
        } else {
            return None;
        }
    }
    Some(result)
}

fn parse_bits<I>(bits: &mut I, count: usize) -> Result<usize, ParseError>
where
    I: Iterator<Item = bool>,
{
    try_parse_bits(bits, count).ok_or_else(|| ParseError::new("expected more bits"))
}

fn take_bits<I>(bits: &mut I, count: usize) -> Result<Vec<bool>, ParseError>
where
    I: Iterator<Item = bool>,
{
    let mut result = Vec::new();
    for _ in 0..count {
        result.push(parse_bits(bits, 1)? != 0);
    }
    Ok(result)
}

pub mod part1 {
    use super::*;

    fn skip_literal<I>(bits: &mut I) -> Result<(), ParseError>
    where
        I: Iterator<Item = bool>,
    {
        while 0b10000 <= parse_bits(bits, 5)? {}
        Ok(())
    }

    fn packet_version_sum<I>(bits: &mut I, version: usize) -> Result<usize, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        let type_id = parse_bits(bits, 3)?;
        if 0b100 == type_id {
            // Type ID 4 means "literal." We skip past the literal without
            // interpreting it, because we care only about the sum of packet
            // version numbers.
            skip_literal(bits)?;
            Ok(version)
        } else if 0 == parse_bits(bits, 1)? {
            // Length type ID 0 means take a number of bits.
            let size = parse_bits(bits, 15)?;
            let kids = take_bits(bits, size)?;
            Ok(version + subpackets_version_sum(&mut kids.into_iter())?)
        } else {
            // Length type ID 1 means take a number of sub-packets.
            let mut sum = version;
            let count = parse_bits(bits, 11)?;
            for _ in 0..count {
                let subversion = parse_bits(bits, 3)?;
                sum += packet_version_sum(bits, subversion)?;
            }
            Ok(sum)
        }
    }

    fn subpackets_version_sum<I>(bits: &mut I) -> Result<usize, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        let mut sum = 0;
        while let Some(version) = try_parse_bits(bits, 3) {
            sum += packet_version_sum(bits, version)?;
        }
        Ok(sum)
    }

    pub fn solve(puzzle: &Puzzle) -> Result<usize, ParseError> {
        let bits = &mut puzzle.bits.iter().cloned();
        let version = parse_bits(bits, 3)?;
        packet_version_sum(bits, version)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            for (want, text) in [
                //(9, "38006F45291200"),
                //(14, "EE00D40C823060"),
                //(16, "8A004A801A8002F478"),
                (12, "620080001611562C8802118E34"),
                //(23, "C0015000016115A2E0802F182340"),
                //(31, "A0016C880162017C3686B18A3D4780"),
            ] {
                let puzzle: Puzzle = text.parse().expect("bad test data");
                assert_eq!(Ok(want), solve(&puzzle));
            }
        }
    }
}

pub mod part2 {
    use super::*;

    fn eval_literal<I>(bits: &mut I) -> Result<usize, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        let mut nybble = parse_bits(bits, 5)?;
        let mut result = nybble & 0xF;
        while 0b10000 <= nybble {
            nybble = parse_bits(bits, 5)?;
            result = result << 4 | nybble & 0xF;
        }
        Ok(result)
    }

    fn eval_operands<I>(bits: &mut I) -> Result<Vec<usize>, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        if 0 == parse_bits(bits, 1)? {
            // Length type ID 0 means take a number of bits.
            let size = parse_bits(bits, 15)?;
            let kids = take_bits(bits, size)?;
            eval_packets(&mut kids.into_iter())
        } else {
            // Length type ID 1 means take a number of sub-packets.
            let count = parse_bits(bits, 11)?;
            let mut operands = Vec::new();
            for _ in 0..count {
                let _version = parse_bits(bits, 3)?;
                operands.push(eval_packet(bits)?);
            }
            Ok(operands)
        }
    }

    /// The next three bits must be a type ID.  The 3-bit version number
    /// preceding each type ID must **already be consumed** before this
    /// function is called.
    fn eval_packet<I>(bits: &mut I) -> Result<usize, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        let type_id = parse_bits(bits, 3)?;
        if 4 == type_id {
            return eval_literal(bits);
        }
        let operands = eval_operands(bits)?;
        match type_id {
            // 0 are sum packets
            // 1 are product packets
            // 2 are minimum packets
            // 3 are maximum packets
            // 5 are greater than packets
            // 6 are less than packets
            // 7 are equal to packets
            0 => Ok(operands.iter().sum()),
            1 => Ok(operands.iter().product()),
            2 => Ok(*operands
                .iter()
                .min()
                .ok_or_else(|| ParseError::new("min requires at least one operand"))?),
            3 => Ok(*operands
                .iter()
                .max()
                .ok_or_else(|| ParseError::new("min requires at least one operand"))?),
            5 => match &operands[..] {
                [a, b] => Ok((a > b) as usize),
                _ => Err(ParseError::new(
                    "greater-than requires exactly two operands",
                )),
            },
            6 => match &operands[..] {
                [a, b] => Ok((a < b) as usize),
                _ => Err(ParseError::new("less-than requires exactly two operands")),
            },
            7 => match &operands[..] {
                [a, b] => Ok((a == b) as usize),
                _ => Err(ParseError::new("equal-to requires exactly two operands")),
            },
            _ => {
                let what = format!("bad operator type ID: {}", type_id);
                Err(ParseError::new(what))
            }
        }
    }

    fn eval_packets<I>(bits: &mut I) -> Result<Vec<usize>, ParseError>
    where
        I: Iterator<Item = bool>,
    {
        let mut results = Vec::new();
        while let Some(_version) = try_parse_bits(bits, 3) {
            results.push(eval_packet(bits)?);
        }
        Ok(results)
    }

    pub fn solve(puzzle: &Puzzle) -> Result<usize, ParseError> {
        let bits = &mut puzzle.bits.iter().cloned();
        let _version = parse_bits(bits, 3)?;
        eval_packet(bits)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            for (want, text) in [
                (3, "C200B40A82"),
                (54, "04005AC33890"),
                (7, "880086C3E88112"),
                (9, "CE00C43D881120"),
                (1, "D8005AC2A8F0"),
                (0, "F600BC2D8F"),
                (0, "9C005AC2F8F0"),
                (1, "9C0141080250320F1802104A08"),
            ] {
                let puzzle: Puzzle = text.parse().expect("bad test data");
                assert_eq!(Ok(want), solve(&puzzle));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        for (want, text) in [
            ("110100101111111000101000", "D2FE28"),
            (
                "00111000000000000110111101000101001010010001001000000000",
                "38006F45291200",
            ),
            (
                "11101110000000001101010000001100100000100011000001100000",
                "EE00D40C823060",
            ),
            (
                "011000100000000010000000000000000001011000010001010101100\
                 01011001000100000000010000100011000111000110100",
                "620080001611562C8802118E34",
            ),
        ] {
            let puzzle: Puzzle = text.parse().expect("bad test string");
            assert_eq!(want, puzzle.bit_string());
        }
    }
}

fn main() {
    let input = "tests/day16/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match part1::solve(&puzzle) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
    match part2::solve(&puzzle) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
