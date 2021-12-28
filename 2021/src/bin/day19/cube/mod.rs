mod parse;
mod rotation;

pub use parse::cubes_from_file;

use crate::beacon::{Beacon, Offset};
use rotation::Rotations;
use std::collections::HashSet;

/// How many beacons need identical relative offsets to prove cubes overlap.
const MIN_OVERLAP: usize = 12;

/// The detection cube of a single scanner.
#[derive(Clone, Eq, PartialEq)]
pub struct Cube {
    beacons: Vec<Beacon>,
    pub scanner: Offset,
}

impl Cube {
    pub fn beacons(&self) -> impl Iterator<Item = Beacon> + '_ {
        self.beacons.iter().cloned()
    }

    pub fn distance(&self, other: &Cube) -> usize {
        // As of this writing, i32::abs_diff remains experimental.
        // https://doc.rust-lang.org/std/primitive.i32.html#method.abs_diff
        let (x0, y0, z0) = self.scanner;
        let (x1, y1, z1) = other.scanner;
        let dx = (x1 - x0).abs() as usize;
        let dy = (y1 - y0).abs() as usize;
        let dz = (z1 - z0).abs() as usize;
        dx + dy + dz
    }

    fn has_min_overlap(&self, beacons: &HashSet<Beacon>) -> bool {
        self.beacons()
            .filter(|b| beacons.contains(b))
            .skip(MIN_OVERLAP - 1)
            .next()
            .is_some()
    }

    fn rotations(&self) -> impl Iterator<Item = Cube> {
        let scanner = self.scanner;
        Rotations::of(&self.beacons).map(move |beacons: Vec<Beacon>| Cube { beacons, scanner })
    }

    /// Returns a transformation of the other cube into this cube's frame of
    /// reference if the cubes have at least MIN_OVERLAP shared beacons, and
    /// None otherwise.
    pub fn transform(&self, other: &Cube) -> Option<Cube> {
        other.rotations().find_map(|cube| self.translate(cube))
    }

    /// Returns a translation of the other cube such that at least MIN_OVERLAP
    /// of its beacons have the same relative positions as corresponding
    /// beacons of this cube, or None if no such translation exists.
    fn translate(&self, other: Cube) -> Option<Cube> {
        let ours: HashSet<_> = self.beacons().collect();
        other
            .beacons()
            .flat_map(|theirs| self.beacons().map(move |ours| ours - theirs))
            .map(|offset| other.translated(offset))
            .find(|translated| translated.has_min_overlap(&ours))
    }

    fn translated(&self, offset: Offset) -> Cube {
        Cube {
            beacons: self.beacons().map(|b| b + offset).collect(),
            scanner: (
                self.scanner.0 + offset.0,
                self.scanner.1 + offset.1,
                self.scanner.2 + offset.2,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Beacons shared by scanners 0 and 1 in the sample rotations.
    const TEST_TRANSFORM01_WANT_BEACONS: [Beacon; MIN_OVERLAP] = [
        Beacon(-618, -824, -621),
        Beacon(-537, -823, -458),
        Beacon(-447, -329, 318),
        Beacon(404, -588, -901),
        Beacon(544, -627, -890),
        Beacon(528, -643, 409),
        Beacon(-661, -816, -575),
        Beacon(390, -675, -793),
        Beacon(423, -701, 434),
        Beacon(-345, -311, 381),
        Beacon(459, -707, 401),
        Beacon(-485, -357, 347),
    ];

    /// Beacons shared by scanners 1 and 4 in the sample rotations.
    const TEST_TRANSFORM14_WANT_BEACONS: [Beacon; MIN_OVERLAP] = [
        Beacon(459, -707, 401),
        Beacon(-739, -1745, 668),
        Beacon(-485, -357, 347),
        Beacon(432, -2009, 850),
        Beacon(528, -643, 409),
        Beacon(423, -701, 434),
        Beacon(-345, -311, 381),
        Beacon(408, -1815, 803),
        Beacon(534, -1912, 768),
        Beacon(-687, -1600, 576),
        Beacon(-447, -329, 318),
        Beacon(-635, -1737, 486),
    ];

    fn assert_intersection(a: &Cube, b: &Cube, want: [Beacon; MIN_OVERLAP]) {
        let a: HashSet<_> = a.beacons().collect();
        let b: HashSet<_> = b.beacons().collect();
        let want = HashSet::from(want);
        let got = HashSet::from_iter(a.intersection(&b).cloned());
        assert_eq!(want, got);
    }

    #[test]
    fn test_rotations_match() {
        let cubes = cubes_from_file("tests/day19/sample-rotations").unwrap();
        for i in 0..cubes.len() {
            let got = cubes[i].rotations().find(|cube| cube == &cubes[0]);
            assert!(got.is_some(), "expected matching rotation; i={}", i);
        }
    }

    #[test]
    fn test_rotations_do_not_match() {
        let cubes = cubes_from_file("tests/day19/sample-rotations").unwrap();
        let runt = Cube {
            beacons: cubes[0].beacons().skip(1).collect(),
            scanner: (0, 0, 0),
        };
        for i in 0..cubes.len() {
            let got = cubes[i].rotations().find(|cube| cube == &runt);
            assert!(got.is_none(), "unexpected matching rotation; i={}", i);
        }
    }

    #[test]
    fn test_transform01() {
        let cubes = cubes_from_file("tests/day19/sample").unwrap();
        let cube0 = &cubes[0];
        let cube1 = &cube0.transform(&cubes[1]).expect("failure to transform");
        assert_intersection(cube0, cube1, TEST_TRANSFORM01_WANT_BEACONS);
    }

    #[test]
    fn test_transform14() {
        let cubes = cubes_from_file("tests/day19/sample").unwrap();
        let cube0 = &cubes[0];
        let cube1 = &cube0.transform(&cubes[1]).expect("failure to transform");
        let cube4 = &cube1.transform(&cubes[4]).expect("failure to transform");
        assert_intersection(cube1, cube4, TEST_TRANSFORM14_WANT_BEACONS);
    }
}
