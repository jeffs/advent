mod core;

use leptos::{
    ev,
    html::{button, div, p, section},
    prelude::*,
};

use shared::{Event, ViewModel};

fn event_button(
    class: &'static str,
    label: &'static str,
    event: Event,
    core: core::Core,
    render: WriteSignal<ViewModel>,
) -> impl IntoView {
    button()
        .class(class)
        .on(ev::click, move |_| {
            core::update(&core, event.clone(), render);
        })
        .child(label)
}

fn root() -> impl IntoView {
    let core = core::new();
    let (view, render) = signal(core.view());

    section()
        .class("box container has-text-centered m-5")
        .child((
            p().class("is-size-5").child(move || view.get().count),
            div().class("buttons section is-centered").child((
                event_button(
                    "button is-primary is-danger",
                    "Reset",
                    Event::Reset,
                    core.clone(),
                    render,
                ),
                event_button(
                    "button is-primary is-success",
                    "Increment",
                    Event::Increment,
                    core.clone(),
                    render,
                ),
                event_button(
                    "button is-primary is-warning",
                    "Decrement",
                    Event::Decrement,
                    core.clone(),
                    render,
                ),
            )),
        ))
}

fn main() {
    leptos::mount::mount_to_body(root);
}
