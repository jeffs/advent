mod day1;
mod day2;

use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

/// The path under which the site is served; it must match `public_url` in
/// `Trunk.toml`.
const BASE: &str = "/advent";

/// Prefixes a site-relative path such as `/2015/day/1` with [`BASE`]. The
/// router does not apply its base to absolute link targets.
fn href(path: &str) -> String {
    format!("{BASE}{path}")
}

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>"Advent of Code 2015"</h1>
        <ul>
            <li><A href=href("/2015/day/1")>"Day 1"</A></li>
            <li><A href=href("/2015/day/2")>"Day 2"</A></li>
        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router base=BASE>
            <nav><A href=href("/")>"All days"</A></nav>
            <main>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Index />
                    <Route path=path!("/2015/day/1") view=day1::Page />
                    <Route path=path!("/2015/day/2") view=day2::Page />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
