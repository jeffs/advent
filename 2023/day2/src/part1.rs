use crate::game::Game;

pub fn solve(text: &str) -> u32 {
    text.lines()
        .map(Game::parse)
        .filter_map(|game| game.is_possible().then_some(game.id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 8);
    }
}
