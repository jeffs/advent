#![allow(dead_code)]

use crate::image::{Image, Point};
use advent2021::ParseError;
use std::collections::HashMap;
use std::str::FromStr;

const TABLE_SIZE: usize = 512;

/// Image enhancement algorithm.
pub struct Algorithm {
    table: [bool; TABLE_SIZE], // false for dark (.), true for light (#)
}

impl Algorithm {
    pub fn enhance(&self, old: &Image) -> Image {
        let mut map: HashMap<Point, u16> = HashMap::new();
        for (i, j) in old {
            *map.entry((i - 1, j - 1)).or_default() |= 1 << 0;
            *map.entry((i - 1, j - 0)).or_default() |= 1 << 1;
            *map.entry((i - 1, j + 1)).or_default() |= 1 << 2;
            *map.entry((i - 0, j - 1)).or_default() |= 1 << 3;
            *map.entry((i - 0, j - 0)).or_default() |= 1 << 4;
            *map.entry((i - 0, j + 1)).or_default() |= 1 << 5;
            *map.entry((i + 1, j - 1)).or_default() |= 1 << 6;
            *map.entry((i + 1, j - 0)).or_default() |= 1 << 7;
            *map.entry((i + 1, j + 1)).or_default() |= 1 << 8;
        }
        let mut new = Image::new();
        for (k, v) in map {
            if self.table[v as usize] {
                new.insert(k);
            }
        }
        new
    }
}

impl FromStr for Algorithm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if TABLE_SIZE != s.len() {
            let what = format!("want {} bytes; got {}", TABLE_SIZE, s.len());
            return Err(ParseError::new(what));
        }
        let mut table = [false; TABLE_SIZE];
        let mut bytes = s.bytes();
        table.fill_with(|| bytes.next() == Some(b'#'));
        Ok(Algorithm { table })
    }
}
