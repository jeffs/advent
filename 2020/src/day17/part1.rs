use super::simulation::Simulation;
use std::error::Error;
use std::fs;

pub fn solve(input_path: &str) -> Result<usize, Box<dyn Error>> {
    let sim = Simulation::from_grid(fs::read_to_string(input_path)?.parse()?);
    Ok(sim.advance(6).population())
}
