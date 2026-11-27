use std::rc::Rc;

use leptos::prelude::*;

use shared::{Day1, Effect, Event, ViewModel};

#[derive(Clone)]
pub struct Context {
    /// `Rc` to allow sharing by callbacks, particularly animation frames.
    core: Rc<shared::Core<Day1>>,
    /// Written on render, read via [`Self::view()`].
    view: RwSignal<ViewModel>,
}

impl Context {
    pub fn new() -> Self {
        let core = Rc::new(shared::Core::new());
        let view = RwSignal::new(core.view());
        Self { core, view }
    }

    fn process_effect(&self, effect: Effect) {
        log::debug!("{effect:?}");
        match effect {
            Effect::Render(_) => {
                self.view.update(|view| *view = self.core.view());
            }
            Effect::RenderFrame(mut request) => {
                let ctx = self.clone();
                request_animation_frame(move || {
                    let effects = ctx
                        .core
                        .resolve(&mut request, ())
                        .expect("resolving animation frame");
                    ctx.process_effects(effects);
                });
            }
        }
    }

    fn process_effects(&self, effects: impl IntoIterator<Item = Effect>) {
        for effect in effects {
            self.process_effect(effect);
        }
    }

    pub fn update(&self, event: Event) {
        log::debug!("{event:?}");
        self.process_effects(self.core.process_event(event));
    }

    pub fn view(&self) -> ReadSignal<ViewModel> {
        self.view.read_only()
    }
}
