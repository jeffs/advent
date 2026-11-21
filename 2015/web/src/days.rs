//! The table of days the site can solve.

use leptos::prelude::AnyView;

use crate::{day1, day2};

/// A day whose puzzle the site can solve.
pub struct Day {
    pub number: u8,
    pub page: fn() -> AnyView,
}

impl Day {
    pub fn path(&self) -> String {
        format!("/day/{}", self.number)
    }
}

/// Every solvable day, in order.
pub const DAYS: &[Day] = &[
    Day {
        number: 1,
        page: day1::page,
    },
    Day {
        number: 2,
        page: day2::page,
    },
];

pub fn find(number: u8) -> Option<&'static Day> {
    DAYS.iter().find(|day| day.number == number)
}
