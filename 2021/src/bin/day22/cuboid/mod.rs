#![allow(dead_code)]

mod receivers;

use self::receivers::RECEIVERS;
use crate::range;
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
    pub fn receive(self, that: &Cuboid) -> impl Iterator<Item = Cuboid> {
        #[rustfmt::skip]
        let Cuboid { xs, ys, zs } = that;
        let rx = range::relation(&self.xs, xs);
        let ry = range::relation(&self.ys, ys);
        let rz = range::relation(&self.zs, zs);
        let index = rx * 100 + ry + 10 + rz;
        RECEIVERS[index](self).into_iter()
    }

    pub fn volume(&self) -> usize {
        let dx = (self.xs.end - self.xs.start) as usize;
        let dy = (self.ys.end - self.ys.start) as usize;
        let dz = (self.zs.end - self.zs.start) as usize;
        dx * dy * dz
    }
}
