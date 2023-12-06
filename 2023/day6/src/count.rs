/// Returns the number of ways to beat the specified record distance in the specified time.
pub fn count(time: i64, distance: i64) -> i64 {
    // Use the quadratic formula to solve for the start time that would achieve the record.
    let (t, d) = (time as f64, distance as f64);
    let s = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
    time + 1 - (s as i64 + 1) * 2
}
