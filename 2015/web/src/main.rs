//! The web shell: a client-side Leptos app that routes to each day.

mod day1;
mod day2;
mod days;

use leptos::{
    html::{a, h1, header, li, main as main_element, nav, p, ul},
    prelude::*,
};
use leptos_router::{
    ParamSegment, StaticSegment,
    components::{A, AProps, Route, RouteProps, Router, RouterProps, Routes, RoutesProps},
    hooks::use_params_map,
};

/// The path of the page that lists every year.
const ALL_YEARS: &str = "/advent/";

/// The path where this year's app is served.
///
/// Must match `public_url` in `Trunk.toml`.
const BASE: &str = "/advent/2015";

const TITLE: &str = "Advent of Code 2015";

fn set_title(page: &str) {
    document().set_title(&format!("{page} · {TITLE}"));
}

/// Returns a client-side link to a site-relative path such as `/day/1`.
/// The router does not apply its base to absolute link targets, so this
/// prefixes [`BASE`]. The router marks the link with `aria-current` while its
/// target is the current page.
fn link(path: &str, text: String) -> impl IntoView + use<> {
    A(AProps::builder()
        .href(format!("{BASE}{path}"))
        .exact(true)
        .children(ToChildren::to_children(move || text))
        .build())
}

fn day_link(day: usize) -> impl IntoView + use<> {
    link(&format!("/day/{day}"), format!("Day {day}"))
}

/// Lists the solvable days.
fn index() -> impl IntoView {
    document().set_title(TITLE);

    (
        h1().child(TITLE),
        ul().child(
            days::numbers()
                .map(|day| li().child(day_link(day)))
                .collect_view(),
        ),
    )
}

fn not_found() -> impl IntoView {
    set_title("Page not found");

    h1().child("Page not found")
}

/// Returns the day links, puzzle link, and `page` for the day numbered `day`.
fn day_page(day: usize, page: fn() -> AnyView) -> impl IntoView {
    set_title(&format!("Day {day}"));

    let puzzle = a()
        .href(format!("https://adventofcode.com/2015/day/{day}"))
        .child("Puzzle description");

    (
        nav()
            .class("days")
            .child(days::numbers().map(day_link).collect_view()),
        p().child(puzzle),
        page(),
    )
}

/// Shows the day named by the route's `day` parameter.
fn day_route() -> impl IntoView {
    let params = use_params_map();

    move || {
        let day = params
            .read()
            .get("day")
            .and_then(|day| day.parse::<usize>().ok())
            .and_then(|day| Some((day, days::page(day)?)));
        match day {
            Some((day, page)) => day_page(day, page).into_any(),
            None => not_found().into_any(),
        }
    }
}

fn routes() -> impl IntoView {
    Routes(
        RoutesProps::builder()
            .fallback(not_found)
            .children(ToChildren::to_children(|| {
                (
                    Route(
                        RouteProps::builder()
                            .path(StaticSegment(""))
                            .view(index)
                            .build(),
                    ),
                    Route(
                        RouteProps::builder()
                            .path((StaticSegment("day"), ParamSegment("day")))
                            .view(day_route)
                            .build(),
                    ),
                )
            }))
            .build(),
    )
}

fn app() -> impl IntoView {
    Router(
        RouterProps::builder()
            .base(BASE)
            .children(ToChildren::to_children(|| {
                (
                    header().child(nav().child((
                        a().href(ALL_YEARS).child("Advent of Code"),
                        link("/", "2015".to_owned()),
                    ))),
                    main_element().child(routes()),
                )
            }))
            .build(),
    )
}

fn main() {
    leptos::mount::mount_to_body(app);
}
