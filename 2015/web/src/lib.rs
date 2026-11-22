mod core;

use leptos::{
    ev,
    html::{label, progress, textarea},
    prelude::*,
};

use shared::Event;

use crate::core::Context;

fn input_textarea(ctx: Context) -> impl IntoView {
    textarea().on(ev::input, move |event| {
        let input = event_target_value(&event);
        ctx.update(Event::Input(input));
    })
}

pub fn root() -> impl IntoView {
    let ctx = Context::new();
    let view = ctx.view();

    let progress_bar = progress()
        .value(move || view.get().index)
        .max(move || view.get().input_len);

    leptos::html::main()
        .child(input_textarea(ctx))
        .child(progress_bar)
        .child(label().child(("Floor: ", move || view.get().floor)))
}
