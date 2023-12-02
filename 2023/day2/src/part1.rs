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
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.samples.iter().all(|sample| sample.is_possible())
    }

    fn parse(line: &str) -> Game {
        let (game, samples) = line.split_once(':').expect("colon");
        let (_, id) = game.split_once(' ').expect("space");
        Game {
            id: id.parse().expect("id"),
            samples: samples.split(';').filter_map(Sample::parse).collect(),
        }
    }
}

pub fn solve(text: &str) -> u32 {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(Game::parse)
        .filter_map(|game| game.is_possible().then_some(game.id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let text = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(solve(text), 8);
    }
}
