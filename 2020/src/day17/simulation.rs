use super::grid::Grid;

#[allow(dead_code)]
pub struct Simulation {
    grid: Grid,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            grid: Grid::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let _grid: Grid = "
            .#.
            ..#
            ###
        ".parse().unwrap();
    }
}

