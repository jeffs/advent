use day11::Grid;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{}", grid.distance_with_expansion(2));
    println!("{}", grid.distance_with_expansion(1000000));
}
