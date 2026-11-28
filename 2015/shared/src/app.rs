use crux_core::{
    App, Command,
    macros::effect,
    operation::{self, Operation, OperationKind},
    render::{RenderOperation, render},
};

#[derive(Clone, Debug)]
pub enum Event {
    /// Resets the sequence of directions to be followed.
    Input(String),
    /// Advances the solver.
    ///
    /// Triggered by resolution of [`RenderFrameOperation`].
    Step,
    /// Sets the number of steps to take per frame.
    Speed(usize),
}

/// Requests resolution in an animation frame.
///
/// The term "animation frame" here means a regularly scheduled UI update.
/// Resolving the effect may trigger a subsequent `Step` event, which in turn
/// may emit another `RenderFrameOperation`, and so on until work is complete.
#[derive(Debug)]
pub struct RenderFrameOperation;

impl Operation for RenderFrameOperation {
    type Output = ();
    const KIND: Option<OperationKind> = Some(OperationKind::Request);
}

impl operation::Request for RenderFrameOperation {}

#[effect]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    RenderFrame(RenderFrameOperation),
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

    fn to_char(&self) -> char {
        match self {
            Self::Up => '(',
            Self::Down => ')',
        }
    }
}

#[derive(Default)]
pub struct Model {
    /// Instructions to be processed.
    directions: Vec<Direction>,
    /// How many directions (input characters) have been processed.
    index: usize,
    /// Floor after processing `index` instructions.
    ///
    /// This is the answer to part 1 once `index == input.len()`.
    floor: i32,
    /// How many directions to process per animation frame.
    speed: usize,
}

impl Model {
    /// Returns true until we finish solving part 1.
    ///
    /// After that, the answer to part 1 is in [`Self::floor`].
    fn needs_step(&self) -> bool {
        self.index < self.directions.len()
    }

    /// Processes a single direction.
    fn step(&mut self) {
        let Some(direction) = self.directions.get(self.index) else {
            return;
        };
        self.index += 1;
        match direction {
            Direction::Up => self.floor += 1,
            Direction::Down => self.floor -= 1,
        }
    }

    /// Processes `self.speed` directions.
    fn advance(&mut self) {
        for _ in 0..self.speed {
            self.step()
        }
    }
}

pub const SPEED_MAX: usize = 100;

#[derive(Clone, Default)]
pub struct ViewModel {
    /// How many instructions have been processed.
    pub index: usize,
    /// Instructions being processed.
    pub input: String,
    /// Current floor.
    ///
    /// When `index == input.len()`, this is the answer to part 1.
    pub floor: i32,
    /// Directions to process per frame ([`Effect::RenderFrame`] resolution).
    pub speed: usize,
}

#[derive(Default)]
pub struct Day1;

/// Renders, and requests another frame if directions remain.
fn animate(model: &Model) -> Command<Effect, Event> {
    if model.needs_step() {
        render().and(Command::request_from_shell(RenderFrameOperation).then_send(|()| Event::Step))
    } else {
        render()
    }
}

fn update_input(input: &str, model: &mut Model) -> Command<Effect, Event> {
    let directions: Option<Vec<_>> = input.chars().map(Direction::from_char).collect();
    let Some(directions) = directions else {
        log::trace!("input should contain only parentheses");
        return Command::done();
    };
    let already_animating = model.needs_step();
    *model = Model {
        directions,
        index: 0,
        floor: 0,
        speed: 1,
    };
    if already_animating {
        render()
    } else {
        animate(model)
    }
}

impl App for Day1 {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Input(s) => update_input(&s, model),
            Event::Step => {
                model.advance();
                animate(model)
            }
            Event::Speed(n) => {
                model.speed = n;
                render()
            }
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            index: model.index,
            input: model.directions.iter().map(Direction::to_char).collect(),
            floor: model.floor,
            speed: model.speed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Resolves animation frames. Render is a no-op.
    fn process_effects(effects: impl Iterator<Item = Effect>) {
        for effect in effects {
            match effect {
                Effect::Render(_) => (),
                Effect::RenderFrame(mut request) => {
                    request.resolve(()).expect("resolving animation frame");
                }
            }
        }
    }

    /// Processes the event and resolves requests until no commands remain.
    fn run(event: Event, model: &mut Model) {
        let mut commands = vec![Day1.update(event, model)];
        while let Some(mut command) = commands.pop() {
            process_effects(command.effects());
            commands.extend(command.events().map(|event| Day1.update(event, model)));
        }
    }

    #[test]
    fn frames_advance_index_to_end() {
        let mut model = Model::default();
        run(Event::Input("(()".to_string()), &mut model);
        assert_eq!(model.index, 3);
    }

    #[test]
    fn input_while_animating_does_not_request_second_frame() {
        let mut model = Model::default();
        let mut first = Day1.update(Event::Input("((".to_string()), &mut model);
        first.expect_render().expect_render_frame();
        Day1.update(Event::Input("()".to_string()), &mut model)
            .expect_only_render();
        assert_eq!(model.index, 0);
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
                let mut model = Model::default();
                run(Event::Input(input.to_string()), &mut model);
                assert_eq!(Day1.view(&model).floor, want, "{input}");
            }
        }
    }
}
