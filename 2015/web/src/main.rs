mod days;
mod page;
mod storage;

use days::{DAYS, Day};
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

const TITLE: &str = "Advent of Code 2015";

fn set_title(page: &str) {
    document().set_title(&format!("{page} · {TITLE}"));
}

/// Returns a client-side link to a site-relative path such as `/2015/day/1`.
/// The router does not apply its base to absolute link targets, so this
/// prefixes [`BASE`].
fn link(path: &str, text: String) -> impl IntoView + use<> {
    A(AProps::builder()
        .href(format!("{BASE}{path}"))
        .children(ToChildren::to_children(move || text))
        .build())
}

fn day_link(day: &Day) -> impl IntoView + use<> {
    link(&day.path(), format!("Day {}", day.number))
}

fn index() -> impl IntoView {
    document().set_title(TITLE);

    (
        h1().child(TITLE),
        ul().child(
            DAYS.iter()
                .map(|day| li().child(day_link(day)))
                .collect_view(),
        ),
    )
}

fn not_found() -> impl IntoView {
    set_title("Page not found");

    h1().child("Page not found")
}

/// Returns links to every day. The router marks the link to the current
/// page with `aria-current`.
fn day_links() -> impl IntoView {
    nav()
        .class("days")
        .child(DAYS.iter().map(day_link).collect_view())
}

/// Returns the day links, puzzle link, and puzzle page for `day`.
fn day_page(day: &'static Day) -> impl IntoView {
    set_title(&format!("Day {}", day.number));

    let puzzle = a()
        .href(format!("https://adventofcode.com{}", day.path()))
        .child("Puzzle description");

    (day_links(), p().child(puzzle), (day.page)())
}

/// Shows the day named by the route's `day` parameter.
fn day_route() -> impl IntoView {
    let params = use_params_map();

    move || {
        let day = params
            .read()
            .get("day")
            .and_then(|number| number.parse().ok())
            .and_then(days::find);
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
                            .path((
                                StaticSegment("2015"),
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
