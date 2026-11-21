//! The page for one puzzle: an input form and answers, driven by a Crux core.

use std::rc::Rc;

use leptos::{
    ev,
    html::{button, dd, dl, dt, form, label, p, textarea},
    prelude::*,
};
use puzzle::{App, Core, Effect, Event, Outcome, Puzzle, ViewModel};

use crate::storage;

/// Connects the core for the puzzle `P` to the page.
struct Shell<P: Puzzle> {
    core: Rc<Core<App<P>>>,
    render: WriteSignal<ViewModel>,
}

impl<P: Puzzle> Clone for Shell<P> {
    fn clone(&self) -> Self {
        Shell {
            core: Rc::clone(&self.core),
            render: self.render,
        }
    }
}

impl<P: Puzzle> Shell<P> {
    /// Sends `event` to the core and performs the effects the core requests.
    fn send(&self, event: Event) {
        self.perform(self.core.process_event(event));
    }

    fn perform(&self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::Render(_) => self.render.set(self.core.view()),
                Effect::Load(mut request) => {
                    let value = storage::load(&request.operation.0);
                    let effects = self
                        .core
                        .resolve(&mut request, value)
                        .expect("the core should accept the answer to its own request");
                    self.perform(effects);
                }
                Effect::Save(request) => {
                    storage::save(&request.operation.key, &request.operation.value);
                }
            }
        }
    }
}

fn outcome(outcome: Outcome) -> AnyView {
    match outcome {
        Outcome::Unsolved => ().into_any(),
        Outcome::Solved { part1, part2 } => dl()
            .child((
                dt().child("Part 1"),
                dd().child(part1),
                dt().child("Part 2"),
                dd().child(part2),
            ))
            .into_any(),
        Outcome::Failed(error) => p().class("error").child(error).into_any(),
    }
}

/// Draws a visualization of the puzzle input it is given.
pub type Visual = fn(Signal<String>) -> AnyView;

/// Returns the input form and answers for the puzzle `P`, followed by
/// `visual`, if any.
pub fn page<P: Puzzle>(visual: Option<Visual>) -> impl IntoView {
    let (view, render) = signal(ViewModel::default());
    let shell = Shell::<P> {
        core: Rc::new(Core::new()),
        render,
    };
    shell.send(Event::Restore);

    let input = textarea()
        .rows("12")
        .prop("value", move || view.get().input)
        .on(ev::input, {
            let shell = shell.clone();
            move |event| shell.send(Event::Edit(event_target_value(&event)))
        })
        .on(ev::keydown, {
            let shell = shell.clone();
            move |event| {
                if event.key() == "Enter" && (event.meta_key() || event.ctrl_key()) {
                    event.prevent_default();
                    shell.send(Event::Solve);
                }
            }
        });

    (
        form()
            .on(ev::submit, move |event| {
                event.prevent_default();
                shell.send(Event::Solve);
            })
            .child((
                label().child(("Paste your input here:", input)),
                p().child(button().attr("type", "submit").child("Solve")),
            )),
        move || outcome(view.get().outcome),
        visual.map(|visual| visual(Memo::new(move |_| view.get().input).into())),
    )
}
