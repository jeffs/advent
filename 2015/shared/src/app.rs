use crux_core::{
    App, Command,
    macros::effect,
    operation::{self, Operation, OperationKind},
    render::{RenderOperation, render},
};

#[derive(Clone, Debug)]
pub enum Event {
    Input(String),
    // The shell is about to repaint.
    Frame,
}

/// Asks the shell to resolve the request just before its next repaint.
#[derive(Debug)]
pub struct AnimationFrame;

impl Operation for AnimationFrame {
    type Output = ();
    const KIND: Option<OperationKind> = Some(OperationKind::Request);
}

impl operation::Request for AnimationFrame {}

#[effect]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    AnimationFrame(AnimationFrame),
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

impl Model {
    /// Returns whether directions remain to be processed.
    ///
    /// An animation frame request is outstanding exactly when this is true.
    fn is_animating(&self) -> bool {
        self.index < self.directions.len()
    }
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

/// Renders, and requests another frame if directions remain.
fn animate(model: &Model) -> Command<Effect, Event> {
    if model.is_animating() {
        render().and(Command::request_from_shell(AnimationFrame).then_send(|()| Event::Frame))
    } else {
        render()
    }
}

fn update_input(input: &str, model: &mut Model) -> Command<Effect, Event> {
    let directions: Option<Vec<_>> = input.chars().map(Direction::from_char).collect();
    let Some(directions) = directions else {
        // TODO: Show errors.
        log::warn!("input should contain only parentheses");
        return Command::done();
    };
    let frame_pending = model.is_animating();
    model.directions = directions;
    model.index = 0;
    model.floor = 0;
    if frame_pending {
        render()
    } else {
        animate(model)
    }
}

fn update_frame(model: &mut Model) -> Command<Effect, Event> {
    if model.is_animating() {
        model.index += 1;
    }
    animate(model)
}

impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Input(s) => update_input(&s, model),
            Event::Frame => update_frame(model),
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

    /// Processes the event, resolving every animation frame request, until
    /// no commands remain.
    fn run(event: Event, model: &mut Model) {
        let mut commands = vec![Counter.update(event, model)];
        while let Some(mut command) = commands.pop() {
            let effects: Vec<_> = command.effects().collect();
            for effect in effects {
                if let Effect::AnimationFrame(mut request) = effect {
                    request.resolve(()).expect("resolving animation frame");
                }
            }
            let events: Vec<_> = command.events().collect();
            commands.extend(events.into_iter().map(|event| Counter.update(event, model)));
        }
    }

    fn solve(input: String) -> i32 {
        let mut model = Model::default();
        run(Event::Input(input), &mut model);
        Counter.view(&model).floor
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
        let mut first = Counter.update(Event::Input("((".to_string()), &mut model);
        first.expect_render().expect_animation_frame();
        Counter
            .update(Event::Input("()".to_string()), &mut model)
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
                assert_eq!(solve(input.to_string()), want, "{input}");
            }
        }
    }
}
