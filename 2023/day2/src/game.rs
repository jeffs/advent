use core::panic;

/// Counts of red, green, and blue marbles, respectively.
#[derive(Debug)]
struct Sample(u32, u32, u32);

impl Sample {
    /// Returns true if this sample contains no more than 12 red marbles, 13
    /// green ones, and 14 blue ones.
    fn is_possible(&self) -> bool {
        self.0 <= 12 && self.1 <= 13 && self.2 <= 14
    }

    fn parse(s: &str) -> Option<Sample> {
        let mut sample = Sample(0, 0, 0);
        for part in s.trim().split(',') {
            let (count, color) = part.trim().split_once(' ').expect("space");
            let count: u32 = count.parse().expect("count");
            match color {
                "red" => sample.0 += count,
                "green" => sample.1 += count,
                "blue" => sample.2 += count,
                _ => panic!("{color}: bad color"),
            }
        }
        Some(sample)
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    samples: Vec<Sample>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.samples.iter().all(|sample| sample.is_possible())
    }

    pub fn parse(line: &str) -> Game {
        let (game, samples) = line.split_once(':').expect("colon");
        let (_, id) = game.split_once(' ').expect("space");
        Game {
            id: id.parse().expect("id"),
            samples: samples.split(';').filter_map(Sample::parse).collect(),
        }
    }

    pub fn product(&self) -> u32 {
        let maxes = self.samples.iter().fold((0, 0, 0), |old, new| {
            (old.0.max(new.0), old.1.max(new.1), old.2.max(new.2))
        });
        maxes.0 * maxes.1 * maxes.2
    }
}
