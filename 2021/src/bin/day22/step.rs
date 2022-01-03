use crate::cuboid::Cuboid;
use crate::range;
use crate::state::State;

use advent2021::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Step {
    pub state: State,
    pub block: Cuboid,
}

impl Step {
    pub fn constrain(&self, max: i32) -> Step {
        Step {
            state: self.state.clone(),
            block: Cuboid {
                xs: range::constrain(&self.block.xs, max),
                ys: range::constrain(&self.block.ys, max),
                zs: range::constrain(&self.block.zs, max),
            },
        }
    }

    pub fn cubes(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        let xs = self.block.xs.clone();
        xs.flat_map(move |x| {
            let ys = self.block.ys.clone();
            ys.flat_map(move |y| {
                let zs = self.block.zs.clone();
                zs.map(move |z| (x, y, z))
            })
        })
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < "on x=1..1,y=1..1,z=1..1".len() {
            return Err(ParseError::new("bad step: too short"));
        }
        let ranges: Vec<_> = s[3..].split(',').collect();
        if ranges.len() != 3 {
            return Err(ParseError::new("bad step: expected three ranges"));
        }
        let step = Step {
            state: s[..3].parse()?,
            block: Cuboid {
                xs: range::parse(ranges[0].trim())?,
                ys: range::parse(ranges[1].trim())?,
                zs: range::parse(ranges[2].trim())?,
            },
        };
        Ok(step)
    }
}
