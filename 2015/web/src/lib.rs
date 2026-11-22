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

pub fn root() -> impl IntoView {
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
