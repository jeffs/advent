//! The web shell: a client-side Leptos app that routes to each year and day.

mod day1;
mod day2;
mod days;

use days::Day;
use leptos::{
    html::{a, h1, header, li, main as main_element, nav, p, ul},
    prelude::*,
};
use leptos_router::{
    ParamSegment, StaticSegment,
    components::{A, AProps, Route, RouteProps, Router, RouterProps, Routes, RoutesProps},
    hooks::use_params_map,
};

/// The path where the site is served.
///
/// Must match `public_url` in `Trunk.toml`.
const BASE: &str = "/advent";

const TITLE: &str = "Advent of Code";

fn set_title(page: &str) {
    document().set_title(&format!("{page} · {TITLE}"));
}

/// Returns a client-side link to a site-relative path such as `/2015/day/1`.
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

fn year_link(year: u16) -> impl IntoView + use<> {
    link(&days::year_path(year), year.to_string())
}

fn day_link(day: &Day) -> impl IntoView + use<> {
    link(&day.path(), format!("Day {}", day.number))
}

/// Lists the years that have solvable days.
fn index() -> impl IntoView {
    document().set_title(TITLE);

    (
        h1().child(TITLE),
        ul().child(
            days::years()
                .map(|year| li().child(year_link(year)))
                .collect_view(),
        ),
    )
}

fn not_found() -> impl IntoView {
    set_title("Page not found");

    h1().child("Page not found")
}

/// Lists the solvable days of `year`.
fn year_page(year: u16) -> impl IntoView {
    set_title(&year.to_string());

    (
        h1().child(format!("{TITLE} {year}")),
        ul().child(
            days::in_year(year)
                .map(|day| li().child(day_link(day)))
                .collect_view(),
        ),
    )
}

/// Returns links to the year of `day` and to each of that year's days.
fn day_links(day: &Day) -> impl IntoView {
    nav().class("days").child((
        year_link(day.year),
        days::in_year(day.year).map(day_link).collect_view(),
    ))
}

/// Returns the day links, puzzle link, and puzzle page for `day`.
fn day_page(day: &'static Day) -> impl IntoView {
    set_title(&format!("{} Day {}", day.year, day.number));

    let puzzle = a()
        .href(format!("https://adventofcode.com{}", day.path()))
        .child("Puzzle description");

    (day_links(day), p().child(puzzle), (day.page)())
}

/// Returns the route parameter `name`, parsed, if the route has it and it
/// parses.
fn param<T: std::str::FromStr>(params: &leptos_router::params::ParamsMap, name: &str) -> Option<T> {
    params.get(name)?.parse().ok()
}

/// Shows the year named by the route's `year` parameter.
fn year_route() -> impl IntoView {
    let params = use_params_map();

    move || {
        let year =
            param(&params.read(), "year").filter(|&year| days::in_year(year).next().is_some());
        match year {
            Some(year) => year_page(year).into_any(),
            None => not_found().into_any(),
        }
    }
}

/// Shows the day named by the route's `year` and `day` parameters.
fn day_route() -> impl IntoView {
    let params = use_params_map();

    move || {
        let params = params.read();
        let day = param(&params, "year")
            .zip(param(&params, "day"))
            .and_then(|(year, number)| days::find(year, number));
        match day {
            Some(day) => day_page(day).into_any(),
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
                            .path(ParamSegment("year"))
                            .view(year_route)
                            .build(),
                    ),
                    Route(
                        RouteProps::builder()
                            .path((
                                ParamSegment("year"),
                                StaticSegment("day"),
                                ParamSegment("day"),
                            ))
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
                    header().child(nav().child(link("/", TITLE.to_owned()))),
                    main_element().child(routes()),
                )
            }))
            .build(),
    )
}

fn main() {
    leptos::mount::mount_to_body(app);
}
