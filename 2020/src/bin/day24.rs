use advent2020::day24::Floor;
use std::fs;

fn main() {
    let input_path = "tests/day24/input";
    let text = fs::read_to_string(input_path).unwrap();
    let floor: Floor = text.parse().unwrap();
    println!("{}", floor.count_black());
    println!("{}", floor.day(100).count_black());
}
