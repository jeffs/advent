//! The app: it computes both answers whenever the input changes.

use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};

pub enum Event {
    SetInput(String),
}

#[effect]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct Model {
    input: String,
}

/// Placeholder: counts the input's characters.
fn part1(input: &str) -> usize {
    input.chars().count()
}

/// Placeholder: counts the input's lines.
fn part2(input: &str) -> usize {
    input.lines().count()
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewModel {
    pub part1: String,
    pub part2: String,
}

#[derive(Default)]
pub struct Day2;

impl App for Day2 {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::SetInput(input) => model.input = input,
        }
        render()
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            part1: part1(&model.input).to_string(),
            part2: part2(&model.input).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_input_renders_both_answers() {
        let mut model = Model::default();

        Day2.update(Event::SetInput("ab\ncd".to_owned()), &mut model)
            .expect_only_render();

        let expected = ViewModel {
            part1: "5".to_owned(),
            part2: "2".to_owned(),
        };
        assert_eq!(Day2.view(&model), expected);
    }
}
