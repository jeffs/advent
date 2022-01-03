use super::Cuboid;

impl Cuboid {
    fn empty(self) -> Vec<Cuboid> {
        vec![]
    }

    fn identity(self) -> Vec<Cuboid> {
        vec![self]
    }
}

pub const RECEIVERS: &[fn(Cuboid) -> Vec<Cuboid>] = &[
    Cuboid::identity, // 000
    Cuboid::empty,    // 001
];
