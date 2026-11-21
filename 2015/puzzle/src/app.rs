//! The Crux app: its events, effects, model, and view model.

use std::marker::PhantomData;

use crux_core::{
    Command,
    macros::effect,
    render::{RenderOperation, render},
};

use crate::{Key, Load, Puzzle, Save};

#[derive(Debug, PartialEq)]
pub enum Event {
    /// Restores the input saved by an earlier visit.
    Restore,
    /// Carries the saved input, if any, back from the shell.
    Restored(Option<String>),
    /// Replaces the input with what the user typed or pasted.
    Edit(String),
    /// Solves both parts for the current input.
    Solve,
}

#[effect]
pub enum Effect {
    Render(RenderOperation),
    Load(Load),
    Save(Save),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Outcome {
    #[default]
    Unsolved,
    Solved {
        part1: String,
        part2: String,
    },
    Failed(String),
}

impl Outcome {
    fn solve<P: Puzzle>(input: &str) -> Self {
        match input.parse::<P::Input>() {
            Ok(input) => Outcome::Solved {
                part1: P::part1(&input).to_string(),
                part2: P::part2(&input).to_string(),
            },
            Err(error) => Outcome::Failed(error.to_string()),
        }
    }
}

#[derive(Default)]
pub struct Model {
    input: String,
    outcome: Outcome,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewModel {
    pub input: String,
    pub outcome: Outcome,
}

/// The Crux app for the puzzle `P`.
pub struct App<P>(PhantomData<fn() -> P>);

impl<P> Default for App<P> {
    fn default() -> Self {
        App(PhantomData)
    }
}

impl<P: Puzzle> App<P> {
    fn input_key() -> Key {
        Key::new(format!("advent/{}/day/{}/input", P::YEAR, P::DAY))
    }
}

impl<P: Puzzle> crux_core::App for App<P> {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Restore => {
                Command::request_from_shell(Load(Self::input_key())).then_send(Event::Restored)
            }
            Event::Restored(None) => Command::done(),
            Event::Restored(Some(input)) => {
                if !input.is_empty() {
                    model.outcome = Outcome::solve::<P>(&input);
                }
                model.input = input;
                render()
            }
            Event::Edit(input) => {
                model.input.clone_from(&input);
                model.outcome = Outcome::Unsolved;
                let save = Save {
                    key: Self::input_key(),
                    value: input,
                };
                Command::notify_shell(save).build().and(render())
            }
            Event::Solve => {
                model.outcome = Outcome::solve::<P>(&model.input);
                render()
            }
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            input: model.input.clone(),
            outcome: model.outcome.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use crux_core::App as _;

    use super::*;

    /// A puzzle whose input is a number and whose answers are small sums.
    struct Sums;

    impl Puzzle for Sums {
        const YEAR: u16 = 2015;

        const DAY: u8 = 7;

        type Input = u32;

        fn part1(input: &u32) -> impl std::fmt::Display {
            input + 1
        }

        fn part2(input: &u32) -> impl std::fmt::Display {
            input + 2
        }
    }

    fn key() -> Key {
        Key::new("advent/2015/day/7/input")
    }

    fn solved() -> Outcome {
        Outcome::Solved {
            part1: "41".to_owned(),
            part2: "42".to_owned(),
        }
    }

    #[test]
    fn restore_loads_the_saved_input() {
        let app = App::<Sums>::default();
        let mut model = Model::default();

        let event = app
            .update(Event::Restore, &mut model)
            .resolve_load(|load| {
                assert_eq!(load, &Load(key()));
                Some("40".to_owned())
            })
            .expect_event();

        assert_eq!(event, Event::Restored(Some("40".to_owned())));
    }

    #[test]
    fn restored_input_is_solved() {
        let app = App::<Sums>::default();
        let mut model = Model::default();

        app.update(Event::Restored(Some("40".to_owned())), &mut model)
            .expect_only_render();

        let expected = ViewModel {
            input: "40".to_owned(),
            outcome: solved(),
        };
        assert_eq!(app.view(&model), expected);
    }

    #[test]
    fn nothing_restored_leaves_the_puzzle_unsolved() {
        let app = App::<Sums>::default();
        let mut model = Model::default();

        app.update(Event::Restored(None), &mut model)
            .expect_no_effects();

        assert_eq!(app.view(&model), ViewModel::default());
    }

    #[test]
    fn edit_saves_the_input_and_discards_the_answers() {
        let app = App::<Sums>::default();
        let mut model = Model {
            input: "40".to_owned(),
            outcome: solved(),
        };

        let save = Save {
            key: key(),
            value: "4".to_owned(),
        };
        app.update(Event::Edit("4".to_owned()), &mut model)
            .expect_save_with(|actual| assert_eq!(actual, &save))
            .expect_only_render();

        let expected = ViewModel {
            input: "4".to_owned(),
            outcome: Outcome::Unsolved,
        };
        assert_eq!(app.view(&model), expected);
    }

    #[test]
    fn solve_reports_a_parse_error() {
        let app = App::<Sums>::default();
        let mut model = Model {
            input: "forty".to_owned(),
            outcome: Outcome::Unsolved,
        };

        app.update(Event::Solve, &mut model).expect_only_render();

        let error: ParseIntError = "forty".parse::<u32>().unwrap_err();
        assert_eq!(app.view(&model).outcome, Outcome::Failed(error.to_string()));
    }
}
