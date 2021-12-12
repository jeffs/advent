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
    type CavePath = Vec<Cave>;
    type CaveSet = HashSet<Cave>;

    // All edges are mirrored, are except for those from START or to END.
    type CaveGraph = HashMap<Cave, CaveSet>;

    fn cave_is_big(cave: &str) -> bool {
        cave.as_bytes()[0].is_ascii_uppercase()
    }

    type CanAdd = fn(&CavePath, &Cave) -> bool;

    pub struct CavePaths<'a> {
        kids: &'a CaveGraph,
        can_add_small: CanAdd,
        paths: Vec<CavePath>,
    }

    impl CavePaths<'_> {
        fn from_graph(kids: &CaveGraph, start: Cave, can_add_small: CanAdd) -> CavePaths {
            CavePaths {
                kids,
                can_add_small,
                paths: vec![vec![start]],
            }
        }
    }

    impl Iterator for CavePaths<'_> {
        type Item = CavePath;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(path) = self.paths.pop() {
                let last = path.last().expect("empty path");
                if END == last {
                    return Some(path);
                }
                self.kids[last]
                    .iter()
                    .filter(|next| cave_is_big(next) || (self.can_add_small)(&path, next))
                    .for_each(|next| {
                        let mut next_path = Vec::new();
                        next_path.reserve_exact(path.len() + 1);
                        next_path.extend(path.iter().cloned());
                        next_path.push(next.clone());
                        self.paths.push(next_path);
                    });
            }
            None
        }
    }

    #[derive(Clone, Debug)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct CaveMap {
        kids: CaveGraph,
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
                    (START, END) | (END, START) => map.connect(START, END),
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

        pub fn paths(&self, can_add_small: CanAdd) -> CavePaths {
            CavePaths::from_graph(&self.kids, START.to_string(), can_add_small)
        }

        pub fn paths_from(&self, start: Cave, can_add_small: CanAdd) -> CavePaths {
            CavePaths::from_graph(&self.kids, start, can_add_small)
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

        pub fn solve(caves: &CaveMap) -> usize {
            caves.paths(|path, cave| !path.contains(cave)).count()
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

    pub mod part2 {
        use super::*;

        fn can_add_small(path: &CavePath, new: &Cave) -> bool {
            let mut seen = HashSet::new();
            for old in path.iter().filter(|cave| !cave_is_big(cave)).cloned() {
                if seen.contains(&old) {
                    return !path.contains(new);
                }
                seen.insert(old);
            }
            true
        }

        pub fn solve(caves: &CaveMap) -> usize {
            let mut threads = Vec::new();
            for entrance in &caves.kids[START] {
                let entrance = entrance.clone();
                let caves = caves.clone();
                threads.push(std::thread::spawn(move || {
                    caves.paths_from(entrance, can_add_small).count()
                }));
            }
            threads
                .into_iter()
                .map(|t| t.join().expect("thread panic"))
                .sum()
        }

        #[cfg(test)]
        mod tests {
            use super::super::CaveMap;
            use super::solve;

            #[test]
            fn test_solve() {
                let caves = CaveMap::from_file("tests/day12/sample1").unwrap();
                assert_eq!(36, solve(&caves));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cave_map_from_file() {
            #[rustfmt::skip]
            let want = CaveMap::from_iter([
                (START, ["A", "b"     ].iter()),
                (  "c", ["A"          ].iter()),
                (  "d", ["b"          ].iter()),
                (  "A", ["b", "c", END].iter()),
                (  "b", ["A", "d", END].iter()),
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
    println!("{}", day12::part2::solve(&caves));
}
