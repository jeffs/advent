/// Returns the value of the page's query parameter with this name.
///
/// Returns `None` if the query lacks the parameter, or the browser can't
/// read the query.
fn url_param(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    web_sys::UrlSearchParams::new_with_str(&search)
        .ok()?
        .get(name)
}

fn url_log_level() -> Option<log::Level> {
    url_param("log")?.parse().ok()
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

fn main() {
    console_log::init_with_level(log_level()).expect("initializing console logger");
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(web::root);
}
