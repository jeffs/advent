//! The day 1 visualization: an elevator following the instructions.

use std::{rc::Rc, time::Duration};

use day1::elevator::{self, Elevator, Event, Speed, Strip, ViewModel};
use leptos::{
    ev,
    html::{button, div, label, option, p, section, select, span},
    prelude::*,
};
use puzzle::Core;

/// How often the page tells the core that time has passed.
const TICK: Duration = Duration::from_millis(50);

/// How many floors to draw above, and below, the current one.
const FLOORS_EACH_WAY: i32 = 4;

/// Connects the elevator's core to the page.
#[derive(Clone)]
struct Shell {
    core: Rc<Core<Elevator>>,
    render: WriteSignal<ViewModel>,
}

impl Shell {
    /// Sends `event` to the core and performs the effects the core requests.
    fn send(&self, event: Event) {
        for effect in self.core.process_event(event) {
            match effect {
                elevator::Effect::Render(_) => self.render.set(self.core.view()),
            }
        }
    }
}

fn controls(shell: &Shell, view: ReadSignal<ViewModel>) -> impl IntoView + use<> {
    let play = button()
        .attr("type", "button")
        .disabled(move || view.with(|view| view.length == 0))
        .on(ev::click, {
            let shell = shell.clone();
            move |_| {
                let running = view.with_untracked(|view| view.running);
                shell.send(if running { Event::Pause } else { Event::Play });
            }
        })
        .child(move || {
            if view.with(|view| view.running) {
                "Pause"
            } else {
                "Play"
            }
        });

    let reset = button()
        .attr("type", "button")
        .on(ev::click, {
            let shell = shell.clone();
            move |_| shell.send(Event::Reset)
        })
        .child("Reset");

    let speed = select()
        .on(ev::change, {
            let shell = shell.clone();
            move |event| {
                let chosen = event_target_value(&event)
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| Speed::ALL.get(index).copied());
                if let Some(speed) = chosen {
                    shell.send(Event::SetSpeed(speed));
                }
            }
        })
        .child(
            Speed::ALL
                .iter()
                .enumerate()
                .map(|(index, &speed)| {
                    option()
                        .value(index.to_string())
                        .selected(move || view.with(|view| view.speed == speed))
                        .child(format!("{} per second", speed.steps_per_second()))
                })
                .collect_view(),
        );

    div()
        .class("controls")
        .child((play, reset, label().child(("Speed ", speed))))
}

fn status(view: &ViewModel) -> impl IntoView + use<> {
    p().class("status").child(format!(
        "Floor {} after {} of {} instructions",
        view.floor, view.position, view.length
    ))
}

/// Draws the instructions around the next one, which stays centered.
fn strip(strip: Strip) -> impl IntoView {
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

/// Returns the elevator visualization for the instructions in `input`.
pub fn elevator(input: Signal<String>) -> AnyView {
    let (view, render) = signal(ViewModel::default());
    let shell = Shell {
        core: Rc::new(Core::new()),
        render,
    };

    Effect::new({
        let shell = shell.clone();
        move |_| shell.send(Event::Load(input.get()))
    });

    let ticker = set_interval_with_handle(
        {
            let shell = shell.clone();
            move || shell.send(Event::Tick(TICK))
        },
        TICK,
    );
    on_cleanup(move || {
        if let Ok(ticker) = ticker {
            ticker.clear();
        }
    });

    section()
        .class("elevator")
        .child((controls(&shell, view), move || {
            let view = view.get();
            (status(&view), strip(view.strip), shaft(view.floor))
        }))
        .into_any()
}
