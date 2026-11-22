//! The day 1 page: a walk through the instructions, a step per frame.

use std::rc::Rc;

use ::day1::{Core, Day1, Effect, Event, ViewModel};
use leptos::{
    ev,
    html::{button, div, label, p, span, textarea},
    prelude::*,
};

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
        // The document outlives this page: once the router shows another day,
        // the frame still fires, but the page's signals are gone.
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
fn passage(view: &ViewModel) -> impl IntoView + use<> {
    let passage = view.passage.clone();
    div().class("passage").child((
        span().class("before").child(passage.before),
        span().class("next").child(passage.next.map(String::from)),
        span().class("after").child(passage.after),
    ))
}

fn toggle_button(
    core: Rc<Core<Day1>>,
    playing: RwSignal<bool>,
    render: WriteSignal<ViewModel>,
) -> impl IntoView {
    let on_click = move |_| {
        if playing.get_untracked() {
            playing.set(false);
            return;
        }
        if core.view().finished {
            // Just when I thought I was out, they pull me back in.
            update(&core, Event::Reset, render);
        }
        playing.set(true);
        play(Rc::clone(&core), render, playing);
    };
    button()
        .on(ev::click, on_click)
        .child(move || if playing.get() { "Pause" } else { "Play" })
}

fn reset_button(
    core: Rc<Core<Day1>>,
    playing: RwSignal<bool>,
    render: WriteSignal<ViewModel>,
) -> impl IntoView {
    button()
        .on(ev::click, move |_| {
            playing.set(false);
            update(&core, Event::Reset, render);
        })
        .child("Reset")
}

fn input(
    core: Rc<Core<Day1>>,
    playing: RwSignal<bool>,
    render: WriteSignal<ViewModel>,
) -> impl IntoView {
    textarea().rows("6").on(ev::input, move |event| {
        playing.set(false);
        update(&core, Event::SetInput(event_target_value(&event)), render);
    })
}

fn status(view: &ViewModel) -> impl IntoView + use<> {
    let part1 = {
        let (floor, position) = (view.floor, view.position);
        let s = if position == 1 { "" } else { "s" };
        p().class("status").child(format!(
            "Part1: Floor {floor} after {position} instruction{s}"
        ))
    };
    let part2 = view.basement.map(|position| {
        let s = if position == 1 { "" } else { "s" };
        p().class("status").child(format!(
            "Part2: Found basement after {position} instruction{s}"
        ))
    });
    (part1, part2)
}

pub fn page() -> impl IntoView {
    log::debug!("Mounting day 1");

    let core = Rc::new(Core::<Day1>::new());
    let (view, render) = signal(core.view());
    let playing = RwSignal::new(false);

    let input = input(Rc::clone(&core), playing, render);
    let toggle = toggle_button(Rc::clone(&core), playing, render);
    let reset = reset_button(core, playing, render);

    (
        label().child(("Paste your input here:", input)),
        div().class("controls").child((toggle, reset)),
        move || (view.with(status), view.with(passage)),
    )
}
