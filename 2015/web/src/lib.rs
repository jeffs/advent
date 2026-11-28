mod core;

use leptos::{
    ev,
    html::{self, div, h1, h2, label, progress, section, span, textarea},
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

fn speed_slider(ctx: Context, speed: Memo<usize>) -> impl IntoView {
    let node = NodeRef::new();
    html::input()
        .r#type("range")
        .node_ref(node)
        .attr("max", core::SPEED_MAX)
        .prop("value", move || speed.get().to_string())
        .on(ev::input, move |event| {
            let value = event_target_value(&event);
            let Ok(value) = value.parse() else {
                log::warn!("speed slider: bad value: {value}");
                return;
            };
            ctx.update(Event::Speed(value));
            if let Some(element) = node.get_untracked() {
                element.set_value(&speed.get_untracked().to_string());
            }
        })
}

fn row(label: &'static str, child: impl IntoView) -> impl IntoView {
    html::label()
        .class("row")
        .child((span().child((label, ":")), child))
}

pub fn root() -> impl IntoView {
    let ctx = Context::new();
    let view = ctx.view();
    let input = Memo::new(move |_| view.get().input);
    let speed = Memo::new(move |_| view.get().speed);

    let progress_bar = progress()
        .value(move || view.get().index)
        .max(move || input.get().len());

    let main = html::main()
        .child(
            section()
                .child(h2().child("Input"))
                .child(input_textarea(ctx.clone(), input))
                .child(row("Speed", speed_slider(ctx, speed))),
        )
        .child(
            section()
                .class("output")
                .child(h2().child("Output"))
                .child(row("Progress", progress_bar))
                .child(row("Floor", move || view.get().floor)),
        );

    (h1().child("2015 Day 1"), main)
}
