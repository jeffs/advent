pub fn add(i: usize, d: usize, n: usize) -> usize {
    assert!(i > 0);
    (i - 1 + d) % n + 1
}

pub fn inc(i: usize, n: usize) -> usize {
    add(i, 1, n)
}
