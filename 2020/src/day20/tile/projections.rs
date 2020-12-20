use super::{Projection, Tile};

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

impl Tile {
    pub fn projections(&self) -> [Projection; 4] {  // TODO: Flip
        let rev_top = rev(&self.top);
        let rev_right = rev(&self.right);
        let rev_bottom = rev(&self.bottom);
        let rev_left = rev(&self.left);
        let turn0 = Projection {
            tile_id: self.id,
            top: self.top.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            left: self.left.clone(),
        };
        let turn1 = Projection {
            tile_id: self.id,
            top: rev_left.clone(),
            right: self.top.clone(),
            bottom: rev_right.clone(),
            left: self.bottom.clone(),
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
            top: self.right.clone(),
            right: rev_bottom.clone(),
            bottom: self.left.clone(),
            left: rev_top.clone(),
        };
        [turn0, turn1, turn2, turn3]
    }
}
