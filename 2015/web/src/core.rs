use std::rc::Rc;

use leptos::prelude::{Update as _, WriteSignal, request_animation_frame};
use shared::{Day1, Effect, Event, ViewModel};

pub type Core = Rc<shared::Core<Day1>>;

pub fn new() -> Core {
    Rc::new(shared::Core::new())
}

fn process_effect(core: &Core, effect: Effect, render: WriteSignal<ViewModel>) {
    log::debug!("{effect:?}");
    match effect {
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
        Effect::RenderFrame(mut request) => {
            let core = Rc::clone(core);
            request_animation_frame(move || {
                let effects = core
                    .resolve(&mut request, ())
                    .expect("resolving animation frame");
                for effect in effects {
                    process_effect(&core, effect, render);
                }
            });
        }
    }
}

pub fn update(core: &Core, event: Event, render: WriteSignal<ViewModel>) {
    log::debug!("{event:?}");
    for effect in core.process_event(event) {
        process_effect(core, effect, render);
    }
}
