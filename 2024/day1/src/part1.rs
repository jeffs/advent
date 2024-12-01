#[derive(Debug)]
pub enum Error {
    BadLine,
}

type Result<T> = std::result::Result<T, Error>;

pub fn distance(xs: impl Into<Vec<u32>>, ys: impl Into<Vec<u32>>) -> u32 {
    let (mut xs, mut ys) = (xs.into(), ys.into());
    debug_assert_eq!(xs.len(), ys.len());
    xs.sort();
    ys.sort();
    xs.iter().zip(ys.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
}

pub fn solve(input: &str) -> Result<u32> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in input.lines() {
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
    Ok(distance(xs, ys))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]), 11);
    }
}
