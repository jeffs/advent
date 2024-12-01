pub fn distance(xs: impl IntoIterator<Item = u32>, ys: impl IntoIterator<Item = u32>) -> u32 {
    let (mut xs, mut ys) = (
        xs.into_iter().collect::<Vec<_>>(),
        ys.into_iter().collect::<Vec<_>>(),
    );
    debug_assert_eq!(xs.len(), ys.len());
    xs.sort_unstable();
    ys.sort_unstable();
    xs.iter().zip(ys.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]), 11);
    }
}
