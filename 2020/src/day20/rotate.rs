pub fn clockwise(lines: &[Vec<u8>]) -> Vec<Vec<u8>> {
    assert!(!lines.is_empty() && !lines[0].is_empty());
    let m = lines.len();
    let n = lines[0].len();
    let mut result = vec![vec![0; m]; n];
    for (i, line) in lines.iter().enumerate() {
        for (j, &b) in line.iter().enumerate() {
            result[j][m - i - 1] = b;
        }
    }
    result
}
