//! The table of days the site can solve.

use leptos::prelude::*;
use puzzle::Puzzle;

use crate::{
    elevator::elevator,
    page::{Visual, page},
};

/// A day whose puzzle the site can solve.
pub struct Day {
    pub year: u16,
    pub number: u8,
    page: fn(Option<Visual>) -> AnyView,
    visual: Option<Visual>,
}

impl Day {
    const fn of<P: Puzzle>() -> Self {
        Day {
            year: P::YEAR,
            number: P::DAY,
            page: any_page::<P>,
            visual: None,
        }
    }

    /// Adds a visualization of the puzzle input to the day's page.
    const fn with_visual(self, visual: Visual) -> Self {
        Day {
            visual: Some(visual),
            ..self
        }
    }

    pub fn page(&self) -> AnyView {
        (self.page)(self.visual)
    }

    pub fn path(&self) -> String {
        format!("{}/day/{}", year_path(self.year), self.number)
    }
}

fn any_page<P: Puzzle>(visual: Option<Visual>) -> AnyView {
    page::<P>(visual).into_any()
}

/// Every solvable day, ordered by year and then by day.
const DAYS: &[Day] = &[
    Day::of::<day1::Day1>().with_visual(elevator),
    Day::of::<day2::Day2>(),
];

pub fn year_path(year: u16) -> String {
    format!("/{year}")
}

/// Returns each year that has a solvable day, in ascending order.
pub fn years() -> impl Iterator<Item = u16> {
    let mut years: Vec<u16> = DAYS.iter().map(|day| day.year).collect();
    years.dedup();
    years.into_iter()
}

pub fn in_year(year: u16) -> impl Iterator<Item = &'static Day> {
    DAYS.iter().filter(move |day| day.year == year)
}

pub fn find(year: u16, number: u8) -> Option<&'static Day> {
    in_year(year).find(|day| day.number == number)
}
