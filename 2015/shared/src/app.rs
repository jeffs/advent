use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};

#[derive(Clone)]
pub enum Event {
    Input(String),
}

#[effect]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
}

pub enum Direction {
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '(' => Self::Up,
            ')' => Self::Down,
            _ => return None,
        })
    }
}

#[derive(Default)]
pub struct Model {
    // Instructions to be processed.
    directions: Vec<Direction>,
    // How many directions (input characters) have been processed.
    index: usize,
    // Floor after processing `index` instructions.
    //
    // This is the answer to part 1 once `index == input.len()`.
    floor: i32,
}

#[derive(Clone, Default)]
pub struct ViewModel {
    // How many instructions have been processed.
    pub index: usize,
    // Total number of instructions to be processed.
    pub input_len: usize,
    // Current floor.
    //
    // When `index == input.len()`, this is the answer to part 1.
    pub floor: i32,
}

#[derive(Default)]
pub struct Counter;

fn update_input(input: &str, model: &mut Model) -> Command<Effect, Event> {
    let directions: Option<Vec<_>> = input.chars().map(Direction::from_char).collect();
    let Some(directions) = directions else {
        // TODO: Show errors.
        return Command::done();
    };
    model.directions = directions;
    render()
}

impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Input(s) => update_input(&s, model),
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            index: model.index,
            input_len: model.directions.len(),
            floor: model.floor,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn solve(input: String) -> i32 {
        let mut model = Model::default();
        Counter
            .update(Event::Input(input), &mut model)
            .expect_only_render();
        Counter.view(&model).floor
    }

    #[test]
    fn part1_examples_get_correct_answers() {
        for (inputs, want) in [
            (&[""][..], 0), // Empty input produces the answer 0.
            (&["(())", "()()"][..], 0),
            (&["(((", "(()(()(", "))((((("], 3),
            (&["())", "))("], -1),
            (&[")))", ")())())"], -3),
        ] {
            for input in inputs {
                assert_eq!(solve(input.to_string()), want, "{input}");
            }
        }
    }
}
