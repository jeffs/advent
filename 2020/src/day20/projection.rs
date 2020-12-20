#![allow(dead_code)]

use super::abutment::Abutment;
use std::collections::HashSet;

/// Rotation and/or reflection of a Tile.
#[derive(Clone)]
pub struct Projection {
    pub tile_id: u64,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub left: String,
}

impl Projection {
    fn abuts(&self, other: &Projection) -> Option<Abutment> {
        if self.top == other.bottom {
            Some(Abutment::Top)
        } else if self.right == other.left {
            Some(Abutment::Right)
        } else if self.bottom == other.top {
            Some(Abutment::Bottom)
        } else if self.left == other.right {
            Some(Abutment::Left)
        } else {
            None
        }
    }

    pub fn is_corner(&self, others: &[Projection]) -> bool {
        use Abutment::*;
        let abutments: HashSet<_> = others.iter().flat_map(|t| self.abuts(t)).collect();
        abutments.len() == 2
            && (abutments.contains(&Top) || abutments.contains(&Bottom))
            && (abutments.contains(&Left) || abutments.contains(&Right))
    }
}
