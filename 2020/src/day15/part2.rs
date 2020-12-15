use super::game::Game;

/// Finds the 30000000th number, starting from the specified starting numbers
/// and proceeding according to the rules of the elves' memory game.
pub fn solve(starting_numbers: &[usize]) -> usize {
    const COUNT: usize = 30000000;
    Game::start(starting_numbers)
        .nth(COUNT - starting_numbers.len() - 1)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_samples() {
        assert_eq!(175594, solve(&[0, 3, 6]));
        assert_eq!(2578, solve(&[1, 3, 2]));
        assert_eq!(3544142, solve(&[2, 1, 3]));
        assert_eq!(261214, solve(&[1, 2, 3]));
        assert_eq!(6895259, solve(&[2, 3, 1]));
        assert_eq!(18, solve(&[3, 2, 1]));
        assert_eq!(362, solve(&[3, 1, 2]));
    }
}
