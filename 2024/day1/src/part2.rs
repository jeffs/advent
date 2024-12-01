use std::collections::HashMap;

fn count(ys: impl IntoIterator<Item = u32>) -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    for y in ys {
        *counts.entry(y).or_insert(0) += 1;
    }
    counts
}

pub fn similarity(xs: impl IntoIterator<Item = u32>, ys: impl IntoIterator<Item = u32>) -> usize {
    let counts = count(ys);
    xs.into_iter()
        .map(|x| x as usize * counts.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity() {
        assert_eq!(similarity([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]), 31);
    }
}
