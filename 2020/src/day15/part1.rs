use super::game::Game;

/// Finds the 2020th number, starting from the specified starting numbers and
/// proceeding according to the rules of the elves' memory game.
pub fn solve(starting_numbers: &[usize]) -> usize {
    const COUNT: usize = 2020;
    Game::start(starting_numbers)
        .nth(COUNT - starting_numbers.len() - 1)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_samples() {
        assert_eq!(436, solve(&[0, 3, 6]));
        assert_eq!(1, solve(&[1, 3, 2]));
        assert_eq!(10, solve(&[2, 1, 3]));
        assert_eq!(27, solve(&[1, 2, 3]));
        assert_eq!(78, solve(&[2, 3, 1]));
        assert_eq!(438, solve(&[3, 2, 1]));
        assert_eq!(1836, solve(&[3, 1, 2]));
    }
}
