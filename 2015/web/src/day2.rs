use std::rc::Rc;

use ::day2::{Core, Day2, Effect, Event, ViewModel};
use leptos::prelude::*;

/// Sends `event` to the core and performs the effects the core requests.
fn update(core: &Core<Day2>, event: Event, render: WriteSignal<ViewModel>) {
    for effect in core.process_event(event) {
        match effect {
            Effect::Render(_) => render.set(core.view()),
        }
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let core = Rc::new(Core::<Day2>::new());
    let (view, render) = signal(core.view());
    let input = RwSignal::new(String::new());

    view! {
        <h1>"Day 2"</h1>
        <textarea rows="12" placeholder="Paste your puzzle input" bind:value=input />
        <p>
            <button on:click=move |_| update(&core, Event::Solve(input.get()), render)>
                "Solve"
            </button>
        </p>
        <p class="error">{move || view.get().error}</p>
        <dl>
            <dt>"Part 1"</dt>
            <dd>{move || view.get().part1}</dd>
            <dt>"Part 2"</dt>
            <dd>{move || view.get().part2}</dd>
        </dl>
    }
}
