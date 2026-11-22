mod core;

use leptos::{
    ev,
    html::{label, progress, textarea},
    prelude::*,
};

use shared::{Event, ViewModel};

fn input_textarea(core: core::Core, render: WriteSignal<ViewModel>) -> impl IntoView {
    textarea().on(ev::input, move |event| {
        let input = event_target_value(&event);
        core::update(&core, Event::Input(input), render);
    })
}

fn root() -> impl IntoView {
    let core = core::new();
    let (view, render) = signal(core.view());

    let progress_bar = progress()
        .value(move || view.get().index)
        .max(move || view.get().input_len);

    leptos::html::main()
        .child(input_textarea(core, render))
        .child(progress_bar)
        .child(label().child(("Floor: ", move || view.get().floor)))
}

/// Returns the level named by the `log` URL parameter (e.g., `?log=trace`).
///
/// Without the parameter, returns a default that depends on the build
/// profile.
fn log_level() -> log::Level {
    url_log_level().unwrap_or(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    })
}

fn url_log_level() -> Option<log::Level> {
    url_param("log")?.parse().ok()
}

/// Returns the value of the page's query parameter with this name.
///
/// Returns `None` if the query lacks the parameter, or the browser can't
/// read the query.
fn url_param(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    web_sys::UrlSearchParams::new_with_str(&search).ok()?.get(name)
}

fn main() {
    console_log::init_with_level(log_level()).expect("initializing console logger");
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(root);
}
