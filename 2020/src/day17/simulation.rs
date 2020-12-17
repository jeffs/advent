use super::grid::Grid;

#[allow(dead_code)]
pub struct Simulation {
    grid: Grid,
}

impl Simulation {
    pub fn from_grid(grid: Grid) -> Simulation {
        Simulation { grid }
    }

    pub fn advance(mut self, n: usize) -> Simulation {
        for _ in 0..n {
            self = self.next();
        }
        self
    }

    pub fn next(self) -> Simulation {
        todo!()
    }

    pub fn population(&self) -> usize {
        self.grid.population()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let grid = "
            .#.
            ..#
            ###
        ";
        let sim = Simulation::from_grid(grid.parse().unwrap());
        assert_eq!(112, sim.advance(6).population());
    }
}
