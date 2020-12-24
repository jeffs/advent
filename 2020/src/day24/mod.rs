//! We'll model our hex grid as a square grid by mapping two of the
//! semicardinal directions to north and south.  There's a little wackiness
//! though, because hex grid axes aren't orthogonal like square grids are:
//! We'll have to alternate which pair of semicardinals to remap (either
//! westerly or easterly) from one row to the next.  Assuming the reference
//! tile is at the origin (0, 0), we'll map NE/SE to north and south in even
//! numbered rows, but map NW/SW in odd ones.
//!
//! Note that unmapped semicardinals mean the same thing they would in any
//! rectangular grid.  For example, going SW from a tile in an even numbered
//! row means going diagonally south and west.

mod direction;
mod floor;
mod latlon;

pub use floor::Floor;
