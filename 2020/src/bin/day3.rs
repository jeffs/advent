use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

fn load_rows<P: AsRef<Path>>(input: P) -> io::Result<Vec<String>> {
    BufReader::new(File::open(input)?).lines().collect()
}

fn solve(rows: &Vec<String>, right: usize, down: usize) -> i32 {
    let mut iter = rows.iter();
    let row = iter.next().expect("empty table");
    let width = row.len();
    let mut x = 0;
    let mut count = if row.as_bytes()[0] == b'#' { 1 } else { 0 };
    while let Some(row) = iter.nth(down - 1) {
        x = (x + right) % width;
        if row.as_bytes()[x] == b'#' {
            count += 1
        }
    }
    return count;
}

fn solve_part1(rows: &Vec<String>) -> i32 {
    solve(rows, 3, 1)
}

fn solve_part2(rows: &Vec<String>) -> i32 {
    solve(rows, 1, 1)
        * solve(rows, 3, 1)
        * solve(rows, 5, 1)
        * solve(rows, 7, 1)
        * solve(rows, 1, 2)
}

fn main() {
    let input = "tests/day3/input";
    let rows = load_rows(input).expect("can't read input");
    println!("{}", solve_part1(&rows));
    println!("{}", solve_part2(&rows));
}
