//! The core shared by every day: a Crux app that takes puzzle input, saves it,
//! and solves it. Each day supplies its solution by implementing [`Puzzle`].

mod app;
mod storage;

use std::{fmt::Display, str::FromStr};

pub use app::{App, Effect, Event, Model, Outcome, ViewModel};
pub use crux_core::Core;
pub use storage::{Key, Load, Save};

/// One day's puzzle: how to parse its input and compute both answers.
pub trait Puzzle: 'static {
    /// The year in which the puzzle was released.
    const YEAR: u16;

    /// The day of December on which the puzzle was released.
    const DAY: u8;

    type Input: FromStr<Err: Display>;

    fn part1(input: &Self::Input) -> impl Display;

    fn part2(input: &Self::Input) -> impl Display;
}
