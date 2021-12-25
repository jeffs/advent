use crate::beacon::Beacon;

fn fall_left(beacons: &[Beacon]) -> Vec<Beacon> {
    beacons.iter().map(|b| b.orbit_y_right()).collect()
}

fn fall_right(beacons: &[Beacon]) -> Vec<Beacon> {
    beacons.iter().map(|b| b.orbit_y_left()).collect()
}

fn turn_left(beacons: &[Beacon]) -> Vec<Beacon> {
    beacons.iter().map(|b| b.orbit_z_right()).collect()
}

pub struct Rotations {
    beacons: Vec<Beacon>,
    step: i32,
}

impl Rotations {
    fn fall_left(&mut self) {
        self.beacons = fall_left(&self.beacons);
    }

    fn fall_right(&mut self) {
        self.beacons = fall_right(&self.beacons);
    }

    pub fn of(beacons: &[Beacon]) -> Rotations {
        Rotations {
            beacons: Vec::from(beacons),
            step: 0,
        }
    }

    fn turn_left(&mut self) {
        self.beacons = turn_left(&self.beacons);
    }
}

impl Iterator for Rotations {
    type Item = Vec<Beacon>;

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::zero_prefixed_literal)]
        match self.step {
            00 => (),                // head up,    face north
            01 => self.turn_left(),  // head up,    face west
            02 => self.turn_left(),  // head up,    face south
            03 => self.turn_left(),  // head up,    face east
            04 => self.fall_left(),  // head north, face east
            05 => self.turn_left(),  // head north, face down
            06 => self.turn_left(),  // head north, face west
            07 => self.turn_left(),  // head north, face up
            08 => self.fall_left(),  // head east,  face up
            09 => self.turn_left(),  // head east,  face south
            10 => self.turn_left(),  // head east,  face down
            11 => self.turn_left(),  // head east,  face north
            12 => self.fall_right(), // head down,  face north
            13 => self.turn_left(),  // head down,  face east
            14 => self.turn_left(),  // head down,  face south
            15 => self.turn_left(),  // head down,  face west
            16 => self.fall_right(), // head south, face west
            17 => self.turn_left(),  // head south, face down
            18 => self.turn_left(),  // head south, face east
            19 => self.turn_left(),  // head south, face up
            20 => self.fall_left(),  // head west,  face up
            21 => self.turn_left(),  // head west,  face north
            22 => self.turn_left(),  // head west,  face down
            23 => self.turn_left(),  // head west,  face south
            _ => return None,
        }
        self.step += 1;
        Some(self.beacons.clone())
    }
}
