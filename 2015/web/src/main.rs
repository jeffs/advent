/// Returns the value of the named query parameter.
fn url_param(name: &str) -> Option<String> {
    use web_sys::{UrlSearchParams, window};
    let search = window()?.location().search().ok()?;
    UrlSearchParams::new_with_str(&search).ok()?.get(name)
}

/// Returns the level named by the `log` URL parameter, e.g. `?log=trace`.
///
/// Defaults to `Debug` in debug builds, and `Info` in release builds.
///
/// # Errors
///
/// If the parameter has an invalid value, returns the value.
fn log_level() -> Result<log::Level, String> {
    match url_param("log") {
        None if cfg!(debug_assertions) => Ok(log::Level::Debug),
        None => Ok(log::Level::Info),
        Some(value) => value.parse().map_err(|_| value),
    }
}

/// Iniializes the `console_log` crate.
fn init_console_log(level: log::Level) {
    if let Err(err) = console_log::init_with_level(level) {
        // Scream into the void.
        log::warn!("initializing logger: {err}");
    }
}

fn main() {
    console_error_panic_hook::set_once();
    init_console_log(log_level().expect("bad log level"));
    leptos::mount::mount_to_body(web::root);
}
