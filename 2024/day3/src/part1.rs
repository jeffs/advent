const PREFIX: &str = "mul(";
const SUFFIX: &str = ")";

pub fn solve(input: &str) -> u32 {
    (0..input.len() - PREFIX.len() + 1)
        .flat_map(|start| {
            input[start..]
                .starts_with(PREFIX)
                .then_some(&input[start + PREFIX.len()..])
        })
        .flat_map(|tail| tail.find(SUFFIX).map(|len| &tail[..len]))
        .flat_map(|args| args.split_once(','))
        .flat_map(|(x, y)| Some(x.parse::<u32>().ok()? * y.parse::<u32>().ok()?))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_solve() {
        assert_eq!(solve(SAMPLE), 161);
    }
}
