pub fn solve(_text: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        for (text, want) in include_str!("samples.txt").split("\n\n").zip([3, 3, 8, 8]) {
            assert_eq!(solve(text), want);
        }
    }
}
