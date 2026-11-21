mod app;
mod storage;

use std::{fmt::Display, str::FromStr};

pub use app::{App, Effect, Event, Model, Outcome, ViewModel};
pub use crux_core::Core;
pub use storage::{Key, Load, Save};

/// One day's puzzle: how to parse its input and compute both answers.
pub trait Puzzle: 'static {
    /// The day of December on which the puzzle was released.
    const DAY: u8;

    type Input: FromStr<Err: Display>;

    fn part1(input: &Self::Input) -> impl Display;

    fn part2(input: &Self::Input) -> impl Display;
}
