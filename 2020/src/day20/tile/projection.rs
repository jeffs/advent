use super::Tile;

/// Rotation and/or reflection of a Tile.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Projection {
    pub tile_id: u64,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub left: String,
}

impl Projection {
    pub fn collect(tiles: &[Tile]) -> Vec<Projection> {
        let mut projections = Vec::new();
        for tile in tiles {
            projections.extend(tile.projections().iter().cloned());
        }
        projections
    }
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

impl Tile {
    pub fn projections(&self) -> [Projection; 8] {
        let top = self.top.clone();
        let right = self.right.clone();
        let bottom = self.bottom.clone();
        let left = self.left.clone();
        let rev_top = rev(&top);
        let rev_right = rev(&right);
        let rev_bottom = rev(&bottom);
        let rev_left = rev(&left);
        let turn0 = Projection {
            tile_id: self.id,
            top: top.clone(),
            right: right.clone(),
            bottom: bottom.clone(),
            left: left.clone(),
        };
        let turn1 = Projection {
            tile_id: self.id,
            top: rev_left.clone(),
            right: top.clone(),
            bottom: rev_right.clone(),
            left: bottom.clone(),
        };
        let turn2 = Projection {
            // two turns: totally topsy turvy
            tile_id: self.id,
            top: rev_bottom.clone(),
            right: rev_left.clone(),
            bottom: rev_top.clone(),
            left: rev_right.clone(),
        };
        let turn3 = Projection {
            tile_id: self.id,
            top: right.clone(),
            right: rev_bottom.clone(),
            bottom: left.clone(),
            left: rev_top.clone(),
        };
        let (top, right, bottom, left, rev_top, rev_right, rev_bottom, rev_left) = (
            rev_top, left, rev_bottom, right, top, rev_left, bottom, rev_right,
        );
        let flip_turn0 = Projection {
            tile_id: self.id,
            top: top.clone(),
            right: right.clone(),
            bottom: bottom.clone(),
            left: left.clone(),
        };
        let flip_turn1 = Projection {
            tile_id: self.id,
            top: rev_left.clone(),
            right: top.clone(),
            bottom: rev_right.clone(),
            left: bottom.clone(),
        };
        let flip_turn2 = Projection {
            tile_id: self.id,
            top: rev_bottom.clone(),
            right: rev_left.clone(),
            bottom: rev_top.clone(),
            left: rev_right.clone(),
        };
        let flip_turn3 = Projection {
            tile_id: self.id,
            top: right.clone(),
            right: rev_bottom.clone(),
            bottom: left.clone(),
            left: rev_top.clone(),
        };
        [
            turn0, turn1, turn2, turn3, flip_turn0, flip_turn1, flip_turn2, flip_turn3,
        ]
    }
}
