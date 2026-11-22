//! The app: it walks through the instructions one step at a time.

use std::mem;

use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};

pub enum Event {
    /// Replaces the instructions and returns to the start.
    SetInput(String),
    /// Follows the next instruction, if any remain.
    Step,
    /// Returns to the start.
    Reset,
}

#[effect]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct Model {
    input: String,
    /// How many instructions have been followed.
    position: usize,
    floor: i32,
}

impl Model {
    fn finished(&self) -> bool {
        self.position >= self.input.chars().count()
    }
}

/// Follows the next instruction.
fn update_step(model: &mut Model) {
    // The body is a placeholder that leaves the model as it is.
    let _ = model;
}

/// The instructions around the current position.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Passage {
    pub before: String,
    /// The instruction the next step follows, if any remain.
    pub next: Option<char>,
    pub after: String,
}

impl Passage {
    /// How many instructions the passage reaches on each side of the next one.
    const REACH: usize = 16;

    fn around(input: &str, position: usize) -> Self {
        let start = position.saturating_sub(Self::REACH);
        let mut chars = input.chars().skip(start);
        Passage {
            before: chars.by_ref().take(position - start).collect(),
            next: chars.next(),
            after: chars.take(Self::REACH).collect(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewModel {
    pub floor: i32,
    pub position: usize,
    pub passage: Passage,
    pub finished: bool,
}

#[derive(Default)]
pub struct Day1;

impl App for Day1 {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::SetInput(input) => {
                *model = Model {
                    input: input.trim().to_owned(),
                    ..Model::default()
                };
            }
            Event::Step => {
                if !model.finished() {
                    update_step(model);
                }
            }
            Event::Reset => {
                *model = Model {
                    input: mem::take(&mut model.input),
                    ..Model::default()
                };
            }
        }
        render()
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            floor: model.floor,
            position: model.position,
            passage: Passage::around(&model.input, model.position),
            finished: model.finished(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_input_trims_the_instructions_and_starts_over() {
        let mut model = Model {
            position: 2,
            floor: 2,
            ..Model::default()
        };

        Day1.update(Event::SetInput("(()\n".to_owned()), &mut model)
            .expect_only_render();

        let expected = ViewModel {
            passage: Passage {
                before: String::new(),
                next: Some('('),
                after: "()".to_owned(),
            },
            ..ViewModel::default()
        };
        assert_eq!(Day1.view(&model), expected);
    }

    #[test]
    fn reset_keeps_the_instructions_and_starts_over() {
        let mut model = Model {
            input: "(()".to_owned(),
            position: 3,
            floor: 1,
        };

        Day1.update(Event::Reset, &mut model).expect_only_render();

        let view = Day1.view(&model);
        assert_eq!((view.position, view.floor), (0, 0));
        assert_eq!(view.passage.next, Some('('));
    }

    #[test]
    fn no_instructions_is_finished() {
        assert!(Day1.view(&Model::default()).finished);
    }

    #[test]
    fn passage_shows_the_instructions_on_both_sides() {
        let passage = Passage::around("abcde", 2);

        let expected = Passage {
            before: "ab".to_owned(),
            next: Some('c'),
            after: "de".to_owned(),
        };
        assert_eq!(passage, expected);
    }
}
