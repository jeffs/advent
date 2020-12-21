pub fn clockwise(lines: &[Vec<u8>]) -> Vec<Vec<u8>> {
    assert!(!lines.is_empty() && !lines[0].is_empty());
    let m = lines.len();
    let n = lines[0].len();
    let mut result = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            result[j][m - i - 1] = lines[i][j];
        }
    }
    result
}
