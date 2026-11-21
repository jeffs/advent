use leptos::prelude::*;
use puzzle::Puzzle;

use crate::page::page;

/// A day whose puzzle the site can solve.
pub struct Day {
    pub number: u8,
    pub page: fn() -> AnyView,
}

impl Day {
    const fn of<P: Puzzle>() -> Self {
        Day {
            number: P::DAY,
            page: any_page::<P>,
        }
    }

    pub fn path(&self) -> String {
        format!("/2015/day/{}", self.number)
    }
}

fn any_page<P: Puzzle>() -> AnyView {
    page::<P>().into_any()
}

pub const DAYS: &[Day] = &[Day::of::<day1::Day1>(), Day::of::<day2::Day2>()];

pub fn find(number: u8) -> Option<&'static Day> {
    DAYS.iter().find(|day| day.number == number)
}
