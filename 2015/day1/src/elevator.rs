//! The Crux app that animates a walk through the instructions.

use std::time::Duration;

use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};

use crate::walk::{Walk, update_step};

/// How fast the walk runs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Speed {
    steps_per_second: u16,
}

impl Speed {
    /// The speeds a user can choose, slowest first.
    pub const ALL: [Speed; 4] = [
        Speed::new(10),
        Speed::new(100),
        Speed::new(1_000),
        Speed::new(10_000),
    ];

    const fn new(steps_per_second: u16) -> Self {
        Speed { steps_per_second }
    }

    pub fn steps_per_second(self) -> u16 {
        self.steps_per_second
    }
}

impl Default for Speed {
    fn default() -> Self {
        Speed::ALL[0]
    }
}

#[derive(Debug, PartialEq)]
pub enum Event {
    /// Replaces the instructions and returns to the start.
    Load(String),
    /// Starts or resumes the walk; from the end, starts over.
    Play,
    Pause,
    /// Returns to the start and stops.
    Reset,
    SetSpeed(Speed),
    /// Reports that this much time has passed.
    Tick(Duration),
}

#[effect]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct Model {
    instructions: String,
    /// The number of instructions, counted once per load.
    length: usize,
    walk: Walk,
    running: bool,
    speed: Speed,
    /// Steps owed for time that has passed; a fraction carries to the next
    /// tick.
    owed: f32,
}

impl Model {
    fn finished(&self) -> bool {
        self.walk.position >= self.length
    }

    fn restart(&mut self) {
        self.walk = Walk::default();
        self.owed = 0.0;
    }

    fn stop(&mut self) {
        self.running = false;
        self.owed = 0.0;
    }

    /// Takes the steps owed for `elapsed`, stopping at the end of the
    /// instructions.
    fn advance(&mut self, elapsed: Duration) {
        self.owed += f32::from(self.speed.steps_per_second) * elapsed.as_secs_f32();
        while self.owed >= 1.0 && !self.finished() {
            self.owed -= 1.0;
            update_step(&self.instructions, &mut self.walk);
        }
        if self.finished() {
            self.stop();
        }
    }
}

/// The instructions around the current position.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Strip {
    pub before: String,
    /// The instruction the next step follows, if any remain.
    pub next: Option<char>,
    pub after: String,
}

impl Strip {
    /// How many instructions to show on each side of the next one.
    const RADIUS: usize = 16;

    fn around(instructions: &str, position: usize) -> Self {
        let start = position.saturating_sub(Self::RADIUS);
        let mut window = instructions.chars().skip(start);
        Strip {
            before: window.by_ref().take(position - start).collect(),
            next: window.next(),
            after: window.take(Self::RADIUS).collect(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewModel {
    pub floor: i32,
    pub position: usize,
    pub length: usize,
    pub strip: Strip,
    pub running: bool,
    pub speed: Speed,
}

#[derive(Default)]
pub struct Elevator;

impl App for Elevator {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Load(instructions) => {
                model.instructions = instructions.trim().to_owned();
                model.length = model.instructions.chars().count();
                model.restart();
                model.stop();
            }
            Event::Play => {
                if model.finished() {
                    model.restart();
                }
                model.running = !model.finished();
            }
            Event::Pause => model.stop(),
            Event::Reset => {
                model.restart();
                model.stop();
            }
            Event::SetSpeed(speed) => model.speed = speed,
            Event::Tick(elapsed) => {
                if !model.running {
                    return Command::done();
                }
                model.advance(elapsed);
            }
        }
        render()
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            floor: model.walk.floor,
            position: model.walk.position,
            length: model.length,
            strip: Strip::around(&model.instructions, model.walk.position),
            running: model.running,
            speed: model.speed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loaded(instructions: &str) -> Model {
        let mut model = Model::default();
        Elevator
            .update(Event::Load(instructions.to_owned()), &mut model)
            .expect_only_render();
        model
    }

    #[test]
    fn load_trims_the_instructions_and_waits_at_the_start() {
        let model = loaded("(()\n");

        let expected = ViewModel {
            length: 3,
            strip: Strip {
                before: String::new(),
                next: Some('('),
                after: "()".to_owned(),
            },
            ..ViewModel::default()
        };
        assert_eq!(Elevator.view(&model), expected);
    }

    #[test]
    fn play_runs_and_pause_stops() {
        let mut model = loaded("(()");

        Elevator
            .update(Event::Play, &mut model)
            .expect_only_render();
        assert!(Elevator.view(&model).running);

        Elevator
            .update(Event::Pause, &mut model)
            .expect_only_render();
        assert!(!Elevator.view(&model).running);
    }

    #[test]
    fn play_without_instructions_stays_stopped() {
        let mut model = loaded("");

        Elevator
            .update(Event::Play, &mut model)
            .expect_only_render();

        assert!(!Elevator.view(&model).running);
    }

    #[test]
    fn tick_while_stopped_requests_nothing() {
        let mut model = loaded("(()");

        Elevator
            .update(Event::Tick(Duration::from_secs(1)), &mut model)
            .expect_no_effects();
    }

    #[test]
    fn strip_shows_the_instructions_on_both_sides() {
        let strip = Strip::around("abcde", 2);

        let expected = Strip {
            before: "ab".to_owned(),
            next: Some('c'),
            after: "de".to_owned(),
        };
        assert_eq!(strip, expected);
    }

    #[test]
    fn strip_has_no_next_instruction_at_the_end() {
        let strip = Strip::around("abc", 3);

        let expected = Strip {
            before: "abc".to_owned(),
            next: None,
            after: String::new(),
        };
        assert_eq!(strip, expected);
    }
}
