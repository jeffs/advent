use day11::Grid;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{}", grid.distance(2));
    println!("{}", grid.distance(1000000));
}
