#![allow(dead_code)]

use std::ops::Range;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cuboid {
    pub xs: Range<i32>,
    pub ys: Range<i32>,
    pub zs: Range<i32>,
}

impl Cuboid {
    /// Inclusive
    fn above(&self, z: i32) -> Cuboid {
        let zs = z..self.zs.end;
        Cuboid { zs, ..self.clone() }
    }

    /// Exclusive
    fn below(&self, z: i32) -> Cuboid {
        let zs = self.zs.start..z;
        Cuboid { zs, ..self.clone() }
    }

    /// Returns boxes representing portions of self not overlapped by that.
    pub fn minus(self, new: &Cuboid) -> impl Iterator<Item = Cuboid> {
        let top = self.above(new.zs.end);
        let bottom = self.below(new.zs.start);
        let zs = bottom.zs.end..top.zs.start;
        let east = Cuboid {
            xs: new.xs.end..self.xs.end,
            ys: self.ys.clone(),
            zs: zs.clone(),
        };
        let west = Cuboid {
            xs: self.xs.start..new.xs.start,
            ys: self.ys.clone(),
            zs: zs.clone(),
        };
        let north = Cuboid {
            xs: west.xs.end..east.xs.start,
            ys: new.ys.end..self.ys.end,
            zs: zs.clone(),
        };
        let south = Cuboid {
            xs: west.xs.end..east.xs.start,
            ys: self.ys.start..new.ys.start,
            zs,
        };
        vec![top, bottom, east, north, south, west]
            .into_iter()
            .filter(|cuboid| cuboid.volume() > 0)
    }

    pub fn volume(&self) -> usize {
        let dx = self.xs.clone().count() as usize;
        let dy = self.ys.clone().count() as usize;
        let dz = self.zs.clone().count() as usize;
        dx * dy * dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INNER: Cuboid = Cuboid {
        xs: -100..100,
        ys: -200..200,
        zs: -300..300,
    };

    const OUTER: Cuboid = Cuboid {
        xs: -101..101,
        ys: -201..201,
        zs: -301..301,
    };

    #[test]
    fn minus() {
        assert_eq!(0, INNER.minus(&INNER).count());
        assert_eq!(0, INNER.minus(&OUTER).count());
        assert_eq!(6, OUTER.minus(&INNER).count());
    }
}
