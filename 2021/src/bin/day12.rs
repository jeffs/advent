#![allow(dead_code)]

use advent2021::ParseError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day12 {
    use super::*;

    const START: &str = "start";
    const END: &str = "end";

    type Cave = String;
    type CaveSet = HashSet<Cave>;

    #[derive(Debug)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct CaveMap {
        kids: HashMap<Cave, CaveSet>,
    }

    impl CaveMap {
        fn connect(&mut self, source: &str, target: &str) {
            // In principle, we shouldn't need the source to be a
            // real String, unless this is the first time we're seeing it;
            // i.e., unless it's not already a key in the map.  However, we
            // won't have a good way to avoid allocating a redundant source
            // String until HashMap::raw_entry_mut is stabilized.  See also:
            // https://stackoverflow.com/a/51542367/3116635
            self.kids
                .entry(source.to_string())
                .and_modify(|set: &mut CaveSet| {
                    set.insert(target.to_string());
                })
                .or_insert_with(|| CaveSet::from_iter([target.to_string()]));
        }

        pub fn from_file<P>(input: P) -> Result<Self, Box<dyn Error>>
        where
            P: AsRef<Path>,
        {
            let mut map = CaveMap {
                kids: HashMap::new(),
            };
            for line in BufReader::new(File::open(input)?).lines() {
                let line = line?;
                let caves = line.split_once('-').ok_or_else(|| {
                    let what = format!("bad line: {}", line);
                    ParseError::new(what)
                })?;
                match caves {
                    (START, END) | (END, START) => {
                        map.connect(START, END);
                    }
                    (START, cave) => map.connect(START, cave),
                    (cave, START) => map.connect(START, cave),
                    (cave, END) => map.connect(cave, END),
                    (END, cave) => map.connect(cave, END),
                    (first, second) => {
                        map.connect(first, second);
                        map.connect(second, first);
                    }
                }
            }
            Ok(map)
        }
    }

    // CaveMapLiteralItem probably should be CaveMap::LiteralItem, but generic
    // associated types are unstable as of this writing.
    #[cfg(test)]
    type CaveMapLiteralItem<'a> = (&'a str, std::slice::Iter<'a, &'a str>);

    #[cfg(test)]
    impl<'a> FromIterator<CaveMapLiteralItem<'a>> for CaveMap {
        fn from_iter<I>(entries: I) -> Self
        where
            I: IntoIterator<Item = CaveMapLiteralItem<'a>>,
        {
            let kids = entries
                .into_iter()
                .map(|(key, values)| {
                    let key = key.to_string();
                    let values = values.map(|v| v.to_string()).collect();
                    (key, values)
                })
                .collect();
            CaveMap { kids }
        }
    }

    pub mod part1 {
        use super::*;

        pub fn solve(_caves: &CaveMap) -> usize {
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::super::CaveMap;
            use super::solve;

            #[test]
            fn test_solve() {
                let wants = [10, 19, 226];
                for (index, want) in wants.into_iter().enumerate() {
                    let sample = format!("tests/day12/sample{}", index + 1);
                    let caves = CaveMap::from_file(sample).unwrap();
                    assert_eq!(want, solve(&caves));
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cave_map_from_file() {
            let want = CaveMap::from_iter([
                ("A", ["c", "end", "b"].iter()),
                ("b", ["end", "A", "d"].iter()),
                ("c", ["A"].iter()),
                ("d", ["b"].iter()),
                ("start", ["b", "A"].iter()),
            ]);
            let got = CaveMap::from_file("tests/day12/sample1").unwrap();
            assert_eq!(want, got);
        }
    }
}

fn main() {
    let input = "tests/day12/input";
    let caves = day12::CaveMap::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day12::part1::solve(&caves));
}
