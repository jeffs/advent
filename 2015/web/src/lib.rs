mod core;

use leptos::{
    ev,
    html::{self, label, progress, textarea},
    prelude::*,
};

use shared::Event;

use crate::core::Context;

fn input_textarea(ctx: Context, input: Memo<String>) -> impl IntoView {
    let node = NodeRef::<html::Textarea>::new();
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

pub fn root() -> impl IntoView {
    let ctx = Context::new();
    let view = ctx.view();
    let input = Memo::new(move |_| view.get().input);

    let progress_bar = progress()
        .value(move || view.get().index)
        .max(move || input.get().len());

    leptos::html::main()
        .child(input_textarea(ctx, input))
        .child(progress_bar)
        .child(label().child(("Floor: ", move || view.get().floor)))
}
