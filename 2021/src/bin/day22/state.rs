use advent2021::ParseError;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum State {
    Off,
    On,
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "off" => Ok(State::Off),
            "on" => Ok(State::On),
            _ => Err(ParseError::new("state must be off on or on")),
        }
    }
}
