//! The walk through the instructions, one step at a time.

/// The state of a walk, kept between steps.
#[derive(Debug, Default, PartialEq)]
pub struct Walk {
    /// How many instructions have been followed. The visualizer highlights
    /// the instruction at this index, and stops the walk once this reaches the
    /// number of instructions.
    pub position: usize,
    /// The current floor. The visualizer draws the building around it.
    pub floor: i32,
}

/// Follows the next instruction, advancing `walk` by one step.
pub fn update_step(instructions: &str, walk: &mut Walk) {
    // The body is a placeholder that leaves the walk where it is.
    let _ = (instructions, walk);
}
