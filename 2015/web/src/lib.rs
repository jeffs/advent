mod core;

use leptos::{
    ev,
    html::{self, label, progress, textarea},
    prelude::*,
};

use shared::Event;

use crate::core::Context;

fn input_textarea(ctx: Context, input: Memo<String>) -> impl IntoView {
    let node = NodeRef::new();
    textarea()
        .node_ref(node)
        .autofocus(true)
        .prop("value", input)
        .on(ev::input, move |event| {
            // Send the input to the core, and Don't let the controlled
            // component update itself.
            ctx.update(Event::Input(event_target_value(&event)));
            if let Some(element) = node.get_untracked() {
                element.set_value(&input.get_untracked());
            }
        })
}

fn speed_slider(ctx: Context) -> impl IntoView {
    html::input()
        .r#type("range")
        .attr("max", "100")
        .attr("value", "1")
        .on(ev::input, move |event| {
            let value = event_target_value(&event);
            let Ok(speed) = value.parse() else {
                log::warn!("speed slider: bad value: {value}");
                return;
            };
            ctx.update(Event::Speed(speed));
        })
}

pub fn root() -> impl IntoView {
    let ctx = Context::new();
    let view = ctx.view();
    let input = Memo::new(move |_| view.get().input);

    let progress_bar = progress()
        .value(move || view.get().index)
        .max(move || input.get().len());

    html::main()
        .child(input_textarea(ctx.clone(), input))
        .child(progress_bar)
        .child(label().child(("Speed:", speed_slider(ctx))))
        .child(label().child(("Floor: ", move || view.get().floor)))
}
