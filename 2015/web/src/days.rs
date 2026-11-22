//! Provides a page representing each daily solution.

use leptos::prelude::*;

/// Daily solutions in chronological order.
const PAGES: &[fn() -> AnyView] = &[
    || crate::day1::page().into_any(),
    || crate::day2::page().into_any(),
];

/// Returns the number of every solvable day, in order.
pub fn numbers() -> impl Iterator<Item = usize> {
    1..=PAGES.len()
}

/// Returns the page for the day numbered `day`, if the site can solve it.
pub fn page(day: usize) -> Option<fn() -> AnyView> {
    PAGES.get(day.checked_sub(1)?).copied()
}
