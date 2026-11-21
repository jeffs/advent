//! The days the site can solve.

use leptos::prelude::AnyView;

use crate::{day1, day2};

/// The page for each solvable day, starting with day 1.
const PAGES: &[fn() -> AnyView] = &[day1::page, day2::page];

/// Returns the number of every solvable day, in order.
pub fn numbers() -> impl Iterator<Item = usize> {
    1..=PAGES.len()
}

/// Returns the page for the day numbered `day`, if the site can solve it.
pub fn page(day: usize) -> Option<fn() -> AnyView> {
    PAGES.get(day.checked_sub(1)?).copied()
}
