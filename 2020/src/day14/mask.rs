#![allow(dead_code, unused_imports, unused_variables)]

use crate::error::ParseError;

const LEN: usize = 36;

fn slice_text(line: &str) -> Result<&str, ParseError> {
    let mut parts = line.splitn(3, ' ');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("mask"), Some("="), Some(text)) => Ok(text),
        _ => Err(ParseError::new(format!("{}: expected mask", line))),
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Leave,
    Clear,
    Set,
}

#[derive(Debug)]
pub struct Mask {
    ops: [Op; LEN],
}

impl Mask {
    pub fn parse_line<S: AsRef<str>>(line: S) -> Result<Mask, ParseError> {
        let text = slice_text(line.as_ref())?;
        if text.len() != LEN {
            let what = format!("{}: bad mask: expected {} bits", text, LEN);
            return Err(ParseError::new(what));
        }
        let mut mask = Mask {
            ops: [Op::Leave; LEN],
        };
        for (i, c) in text.bytes().enumerate() {
            mask.ops[i] = match c {
                b'X' => Op::Leave,
                b'0' => Op::Clear,
                b'1' => Op::Set,
                _ => {
                    let what = format!("{}: bad mask bit", c);
                    return Err(ParseError::new(what));
                }
            };
        }
        Ok(mask)
    }
}
