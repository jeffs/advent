use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::Path;

pub type Point = (i32, i32); // (i, j) coordinates

pub struct Image {
    default: bool,           // false for dark, true for light
    special: HashSet<Point>, // pixels that don't have the default state
}

impl Image {
    pub fn at(&self, i: i32, j: i32) -> bool {
        self.special.contains(&(i, j)) ^ self.default
    }

    pub fn background(&self) -> bool {
        self.default
    }

    pub fn count_lights(&self) -> Result<usize, &'static str> {
        if self.default {
            return Err("can't count infinite lights");
        }
        Ok(self.special.len())
    }

    pub fn max_i(&self) -> i32 {
        self.special
            .iter()
            .map(|&(i, _)| i)
            .max()
            .unwrap_or_default()
    }

    pub fn max_j(&self) -> i32 {
        self.special
            .iter()
            .map(|&(_, j)| j)
            .max()
            .unwrap_or_default()
    }

    pub fn min_i(&self) -> i32 {
        self.special
            .iter()
            .map(|&(i, _)| i)
            .min()
            .unwrap_or_default()
    }

    pub fn min_j(&self) -> i32 {
        self.special
            .iter()
            .map(|&(_, j)| j)
            .min()
            .unwrap_or_default()
    }

    pub fn new(default: bool, special: HashSet<Point>) -> Image {
        Image { default, special }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        fs::write(path, format!("{}", self)).expect("can't write image file");
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let c = if self.default { '.' } else { '#' };
        let d = if self.default { '#' } else { '.' };
        for i in self.min_i()..=self.max_i() {
            for j in self.min_j()..=self.max_j() {
                let c = if self.special.contains(&(i, j)) { c } else { d };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
