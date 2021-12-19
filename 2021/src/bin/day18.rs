use advent2021::ParseError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::str::FromStr;

/// Snailfish numbers are sequences of symbols.  Technically, they're valid
/// JSON, and snailfish homework is JSON-LD.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Symbol {
    Open,       // [
    Close,      // ]
    Scalar(u8), // "regular number"
}

impl Symbol {
    fn value(&self) -> Option<u8> {
        match self {
            Symbol::Scalar(value) => Some(*value),
            _ => None,
        }
    }

    fn value_mut(&mut self) -> Option<&mut u8> {
        match self {
            Symbol::Scalar(ref mut value) => Some(value),
            _ => None,
        }
    }

    fn from_byte(byte: u8) -> Result<Option<Symbol>, ParseError> {
        Ok(match byte {
            b'[' => Some(Symbol::Open),
            b']' => Some(Symbol::Close),
            b'0'..=b'9' => Some(Symbol::Scalar(byte - b'0')),
            b',' => None,
            _ => {
                let what = format!("bad symbol; byte value: {}", byte);
                return Err(ParseError::new(what));
            }
        })
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Open => write!(f, "["),
            Symbol::Close => write!(f, "]"),
            Symbol::Scalar(n) => write!(f, "{}", n),
        }
    }
}

fn magnitude(symbols: &[Symbol]) -> u64 {
    if let Symbol::Scalar(n) = symbols[0] {
        return n as u64;
    }
    let mut depth = 0;
    for (i, sym) in symbols.iter().enumerate() {
        if matches!(sym, Symbol::Open) {
            depth += 1;
        } else {
            if matches!(sym, Symbol::Close) {
                depth -= 1;
            }
            if depth == 1 {
                let sep = i + 1;
                let left = &symbols[1..sep];
                let right = &symbols[sep..(symbols.len() - 1)];
                return 3 * magnitude(left) + 2 * magnitude(right);
            }
        }
    }
    panic!("bad number")
}

fn max_depth(symbols: &[Symbol]) -> usize {
    let (mut depth, mut result) = (0, 0);
    for sym in symbols {
        match sym {
            Symbol::Open => {
                depth += 1;
                result = result.max(depth);
            }
            Symbol::Close => {
                depth -= 1;
            }
            _ => (),
        }
    }
    result
}

#[derive(Eq, PartialEq)]
enum IsDone {
    No,
    Yes,
}

struct ReducerOnce<'a> {
    old: &'a [Symbol],  // number to be reduced
    new: Vec<Symbol>,   // result number being built
    is_explosive: bool, // whether old should explode
    depth: u8,          // nesting level; explode if > 4
}

impl ReducerOnce<'_> {
    fn explode_left(&mut self) {
        let carry = self
            .pop()
            .value()
            .expect("non-scalar where there ought to be a carry-left");
        if let Some(sym) = self
            .new
            .iter_mut()
            .rev()
            .filter_map(|sym| sym.value_mut())
            .next()
        {
            *sym += carry;
        }
    }

    fn explode_right(&mut self) {
        let mut carry = self
            .pop()
            .value()
            .expect("non-scalar where there ought to be a carry-right");
        if Symbol::Close != self.pop() {
            panic!("expected close to match open");
        }
        while !self.old.is_empty() {
            let sym = match self.pop() {
                Symbol::Scalar(n) if carry != 0 => Symbol::Scalar(n + std::mem::take(&mut carry)),
                sym => sym,
            };
            self.new.push(sym);
        }
    }

    fn explode(&mut self) {
        self.explode_left();
        self.new.push(Symbol::Scalar(0));
        self.explode_right();
    }

    pub fn new(old: &[Symbol]) -> ReducerOnce {
        ReducerOnce {
            old,
            new: Vec::new(),
            is_explosive: max_depth(old) > 4,
            depth: 0,
        }
    }

    /// Removes and returns the next symbol in the supplied old number.  Panics
    /// if there is no such symbol.
    fn pop(&mut self) -> Symbol {
        let (head, tail) = self.old.split_first().expect("incomplete number");
        self.old = tail;
        *head
    }

    pub fn reduce_once(mut self) -> Number {
        while self.step() == IsDone::No {}
        Number { symbols: self.new }
    }

    fn split(&mut self, n: u8) {
        let half = n / 2;
        self.new.extend([
            Symbol::Open,
            Symbol::Scalar(half),
            Symbol::Scalar(n - half),
            Symbol::Close,
        ]);
        self.new.extend(self.old.iter());
    }

    fn step_open(&mut self) -> IsDone {
        if self.depth < 4 {
            self.new.push(Symbol::Open);
            self.depth += 1;
            IsDone::No
        } else {
            assert_eq!(self.depth, 4);
            self.explode();
            IsDone::Yes
        }
    }

    fn step_close(&mut self) -> IsDone {
        assert!(self.depth > 0);
        self.new.push(Symbol::Close);
        self.depth -= 1;
        if self.depth > 0 {
            IsDone::No
        } else {
            IsDone::Yes
        }
    }

    fn step_value(&mut self, n: u8) -> IsDone {
        if n < 10 || self.is_explosive {
            self.new.push(Symbol::Scalar(n));
            IsDone::No
        } else {
            self.split(n);
            IsDone::Yes
        }
    }

    /// Processes the next scalar or pair in the supplied 'old' number.
    fn step(&mut self) -> IsDone {
        match self.pop() {
            Symbol::Open => self.step_open(),
            Symbol::Close => self.step_close(),
            Symbol::Scalar(n) => self.step_value(n),
        }
    }
}

/// Snailfish number reduction depends on left/right spatial relationships of
/// "regular" (scalar) numbers.  To represent those relationships as directly
/// as possible, we store each snailfish number as a sequence of left-to-right
/// symbols corresponding to its textual representation, rather than as an
/// expression/syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Number {
    symbols: Vec<Symbol>,
}

impl Number {
    fn add(&self, right: &Number) -> Number {
        let mut symbols = vec![Symbol::Open];
        symbols.extend(self.symbols.iter());
        symbols.extend(right.symbols.iter());
        symbols.push(Symbol::Close);
        Number { symbols }.reduce()
    }

    fn magnitude(&self) -> u64 {
        magnitude(&self.symbols)
    }

    fn reduce(&self) -> Number {
        let mut old = self.clone();
        let mut new = old.reduce_once();
        while new != old {
            std::mem::swap(&mut old, &mut new);
            new = old.reduce_once()
        }
        new
    }

    fn reduce_once(&self) -> Number {
        ReducerOnce::new(&self.symbols).reduce_once()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for sym in &self.symbols {
            write!(f, "{}", sym)?;
        }
        Ok(())
    }
}

impl FromStr for Number {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Validate bracket matching and max nesting of 4.
        let mut symbols = Vec::new();
        for byte in s.bytes() {
            if let Some(symbol) = Symbol::from_byte(byte)? {
                symbols.push(symbol);
            }
        }
        Ok(Number { symbols })
    }
}

pub struct Homework {
    numbers: Vec<Number>, // Each line of input is a snailfish number.
}

impl Homework {
    fn from_file<P>(input: P) -> Result<Homework, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut numbers = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            numbers.push(line?.parse()?);
        }
        Ok(Homework { numbers })
    }

    fn sum(&self) -> Number {
        self.numbers
            .iter()
            .cloned()
            .reduce(|a, b| a.add(&b))
            .expect("empty homework")
    }
}

pub mod part1 {
    use super::Homework;

    pub fn solve(hw: &Homework) -> u64 {
        hw.sum().magnitude()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Homework};

        #[test]
        fn test_solve() {
            let homework = Homework::from_file("tests/day18/sample").unwrap();
            assert_eq!(4140, solve(&homework));
        }
    }
}

mod part2 {
    use super::Homework;

    pub fn solve(hw: &Homework) -> u64 {
        let mut result = 0;
        for (i, a) in hw.numbers.iter().enumerate() {
            for (_, b) in hw.numbers.iter().enumerate().filter(|&(j, _)| i != j) {
                result = result.max(a.add(b).magnitude());
                result = result.max(b.add(a).magnitude());
            }
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Homework};

        #[test]
        fn test_solve() {
            let homework = Homework::from_file("tests/day18/sample").unwrap();
            assert_eq!(3993, solve(&homework));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_explode() {
        for (orig, want) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            let orig: Number = orig.parse().expect("bad orig");
            let want: Number = want.parse().expect("bad want");
            assert_eq!(want.to_string(), orig.reduce_once().to_string());
        }
    }

    #[test]
    fn test_reduce_once() {
        let mut number: Number = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]" // after addition
            .parse()
            .expect("bad number");
        for want in [
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",     // after explode
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",       // after explode
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",    // after split
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", // after split
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",     // after explode
        ] {
            let want: Number = want.parse().expect("bad want");
            number = number.reduce_once();
            assert_eq!(want.to_string(), number.to_string());
        }
    }

    #[test]
    fn test_add() {
        let mut number: Number = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse()
            .expect("bad number");
        for (addend, want) in [
            (
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            ("[2,9]", "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"),
            (
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ] {
            let want: Number = want.parse().expect("bad want");
            number = number.add(&addend.parse().expect("bad addend"));
            assert_eq!(want.to_string(), number.to_string());
        }
    }

    #[test]
    fn test_sum() {
        let hw = Homework {
            numbers: ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
                .iter()
                .map(|s| s.parse().expect("bad addend"))
                .collect(),
        };
        let want: Number = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().expect("bad sum");
        assert_eq!(want.to_string(), hw.sum().to_string());
    }

    #[test]
    fn test_sum_larger() {
        let hw = Homework {
            numbers: [
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ]
            .iter()
            .map(|s| s.parse().expect("bad addend"))
            .collect(),
        };
        let want: Number = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .expect("bad sum");
        assert_eq!(want.to_string(), hw.sum().to_string());
    }
}

fn main() {
    let input = "tests/day18/input";
    let homework = Homework::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&homework));
    println!("{}", part2::solve(&homework));
}
