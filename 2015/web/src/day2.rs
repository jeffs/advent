//! The day 2 page: both answers, updated as the input changes.

use ::day2::{Core, Day2, Effect, Event, ViewModel};
use leptos::{
    ev,
    html::{dd, dl, dt, label, textarea},
    prelude::*,
};

/// Sends `event` to the core and performs the effects the core requests.
fn update(core: &Core<Day2>, event: Event, render: WriteSignal<ViewModel>) {
    for effect in core.process_event(event) {
        match effect {
            Effect::Render(_) => render.set(core.view()),
        }
    }
}

pub fn page() -> AnyView {
    log::debug!("Mounting day 2");

    let core = Core::<Day2>::new();
    let (view, render) = signal(core.view());

    let input = textarea().rows("6").on(ev::input, move |event| {
        update(&core, Event::SetInput(event_target_value(&event)), render);
    });

    (
        label().child(("Paste your input here:", input)),
        dl().child((
            dt().child("Part 1"),
            dd().child(move || view.get().part1),
            dt().child("Part 2"),
            dd().child(move || view.get().part2),
        )),
    )
        .into_any()
}
