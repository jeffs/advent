use leptos::prelude::window;
use puzzle::Key;
use web_sys::Storage;

/// Returns the browser's local storage, or `None` where the browser denies
/// access to it, as some do in private windows.
fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

pub fn load(key: &Key) -> Option<String> {
    local_storage()?.get_item(key.as_str()).ok().flatten()
}

/// Stores `value` under `key`. Storage is a convenience, so a failure, such as
/// exceeding the browser's quota, loses only the saved copy.
pub fn save(key: &Key, value: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(key.as_str(), value);
    }
}
