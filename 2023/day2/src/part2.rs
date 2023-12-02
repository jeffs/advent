use crate::game::Game;

pub fn solve(text: &str) -> u32 {
    text.lines()
        .map(Game::parse)
        .map(|game| game.product())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 2286);
    }
}
