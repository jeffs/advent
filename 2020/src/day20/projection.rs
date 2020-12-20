/// Rotation and/or reflection of a Tile.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Projection {
    pub tile_id: u64,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub left: String,
}
