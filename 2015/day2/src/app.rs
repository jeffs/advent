use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};

use crate::solution::{self, Input, part1, part2};

pub enum Event {
    /// Solves both parts for the given puzzle input.
    Solve(String),
}

#[effect]
pub enum Effect {
    Render(RenderOperation),
}

struct Answers {
    part1: usize,
    part2: usize,
}

impl Answers {
    fn solve(input: &str) -> Result<Self, solution::Error> {
        let input: Input = input.parse()?;
        Ok(Answers {
            part1: part1(&input),
            part2: part2(&input),
        })
    }
}

#[derive(Default)]
pub struct Model {
    outcome: Option<Result<Answers, solution::Error>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewModel {
    pub part1: Option<String>,
    pub part2: Option<String>,
    pub error: Option<String>,
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
            Event::Solve(input) => model.outcome = Some(Answers::solve(&input)),
        }
        render()
    }

    fn view(&self, model: &Model) -> ViewModel {
        match &model.outcome {
            None => ViewModel::default(),
            Some(Ok(answers)) => ViewModel {
                part1: Some(answers.part1.to_string()),
                part2: Some(answers.part2.to_string()),
                error: None,
            },
            Some(Err(error)) => ViewModel {
                error: Some(error.to_string()),
                ..ViewModel::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_renders_answers() {
        let app = Day2;
        let mut model = Model::default();

        app.update(Event::Solve("ab\ncd".to_owned()), &mut model)
            .expect_only_render();

        assert_eq!(
            app.view(&model),
            ViewModel {
                part1: Some("5".to_owned()),
                part2: Some("2".to_owned()),
                error: None,
            }
        );
    }

    #[test]
    fn solve_renders_parse_error() {
        let app = Day2;
        let mut model = Model::default();

        app.update(Event::Solve(String::new()), &mut model)
            .expect_only_render();

        assert_eq!(app.view(&model).error.as_deref(), Some("input is empty"));
    }
}
