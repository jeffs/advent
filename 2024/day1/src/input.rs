use std::str::FromStr;

pub struct Input(pub Vec<u32>, pub Vec<u32>);

#[derive(Debug)]
pub enum Error {
    BadLine,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for line in s.lines() {
            let mut parts = line.split_ascii_whitespace();
            let (Some(x), Some(y), None) = (parts.next(), parts.next(), parts.next()) else {
                return Err(Error::BadLine);
            };
            let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) else {
                return Err(Error::BadLine);
            };
            xs.push(x);
            ys.push(y);
        }
        Ok(Input(xs, ys))
    }
}
