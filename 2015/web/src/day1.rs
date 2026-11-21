//! The day 1 page: an elevator following the instructions, a step per frame.

use std::rc::Rc;

use ::day1::{Core, Day1, Effect, Event, ViewModel};
use leptos::{
    ev,
    html::{button, div, label, p, span, textarea},
    prelude::*,
};

/// How many floors to draw above, and below, the current one.
const FLOORS_EACH_WAY: i32 = 4;

/// Sends `event` to the core and performs the effects the core requests.
fn update(core: &Core<Day1>, event: Event, render: WriteSignal<ViewModel>) {
    for effect in core.process_event(event) {
        match effect {
            Effect::Render(_) => render.set(core.view()),
        }
    }
}

/// Steps the core once per animation frame, until `playing` turns false or
/// the walk finishes.
fn play(core: Rc<Core<Day1>>, render: WriteSignal<ViewModel>, playing: RwSignal<bool>) {
    request_animation_frame(move || {
        // The signal is gone once the user has left the page.
        if playing.try_get_untracked() != Some(true) {
            return;
        }
        update(&core, Event::Step, render);
        if core.view().finished {
            playing.set(false);
        } else {
            play(core, render, playing);
        }
    });
}

/// Draws the instructions around the next one, which stays centered.
fn strip(view: &ViewModel) -> impl IntoView + use<> {
    let strip = view.strip.clone();
    div().class("strip").child((
        span().class("before").child(strip.before),
        span().class("next").child(strip.next.map(String::from)),
        span().class("after").child(strip.after),
    ))
}

/// Draws the floors around `current`, highest first.
fn shaft(current: i32) -> impl IntoView {
    let floors = (-FLOORS_EACH_WAY..=FLOORS_EACH_WAY).rev().map(|offset| {
        let floor = current.saturating_add(offset);
        let mut class = String::from("floor");
        if offset == 0 {
            class.push_str(" car");
        }
        if floor == 0 {
            class.push_str(" ground");
        }
        div()
            .class(class)
            .child((span().class("number").child(floor), span().class("room")))
    });

    div().class("shaft").child(floors.collect_view())
}

pub fn page() -> AnyView {
    let core = Rc::new(Core::<Day1>::new());
    let (view, render) = signal(core.view());
    let playing = RwSignal::new(false);

    let input = textarea().rows("6").on(ev::input, {
        let core = Rc::clone(&core);
        move |event| {
            playing.set(false);
            update(&core, Event::SetInput(event_target_value(&event)), render);
        }
    });

    let toggle = button()
        .on(ev::click, {
            let core = Rc::clone(&core);
            move |_| {
                if playing.get_untracked() {
                    playing.set(false);
                    return;
                }
                if core.view().finished {
                    update(&core, Event::Reset, render);
                }
                playing.set(true);
                play(Rc::clone(&core), render, playing);
            }
        })
        .child(move || if playing.get() { "Pause" } else { "Play" });

    let reset = button()
        .on(ev::click, move |_| {
            playing.set(false);
            update(&core, Event::Reset, render);
        })
        .child("Reset");

    (
        label().child(("Paste your input here:", input)),
        div().class("controls").child((toggle, reset)),
        move || {
            let view = view.get();
            (
                p().class("status").child(format!(
                    "Floor {} after {} instructions",
                    view.floor, view.position
                )),
                strip(&view),
                shaft(view.floor),
            )
        },
    )
        .into_any()
}
