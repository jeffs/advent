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
    /// Returns boxes representing portions of self not overlapped by that.
    pub fn receive(self, new: &Cuboid) -> impl Iterator<Item = Cuboid> {
        #[rustfmt::skip]
        let Cuboid { xs, ys, zs } = new;
        let rx = range::relation(&self.xs, xs);
        let ry = range::relation(&self.ys, ys);
        let rz = range::relation(&self.zs, zs);
        let index = rx * 100 + ry + 10 + rz;
        RECEIVERS[index](self, new).into_iter()
    }

    pub fn volume(&self) -> usize {
        let dx = (self.xs.end - self.xs.start) as usize;
        let dy = (self.ys.end - self.ys.start) as usize;
        let dz = (self.zs.end - self.zs.start) as usize;
        dx * dy * dz
    }
}
