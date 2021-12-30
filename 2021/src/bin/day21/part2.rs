use crate::player::Player;
use crate::puzzle::Puzzle;
use std::collections::HashMap;

type Tally = [usize; 2]; // scores

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    turn: usize, // 0 or 1
    players: [Player; 2],
}

fn roll3() -> impl Iterator<Item = usize> {
    (1..=3).flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
}

fn successors(old: &State) -> impl Iterator<Item = State> {
    let old = old.clone();
    let turn = (old.turn + 1) % 2;
    roll3().map(move |sum| {
        let mut players = old.players.clone();
        players[old.turn].advance(sum);
        State { turn, players }
    })
}

fn solve_score(puzzle: &Puzzle, score: usize) -> usize {
    let init = State {
        turn: 0,
        players: puzzle.new_players(),
    };
    let mut memo: HashMap<State, Tally> = HashMap::new();
    let mut stack = vec![init.clone()];
    while let Some(old) = stack.pop() {
        if memo.contains_key(&old) {
            // Nothing to do.  We already solved the old state.
        } else {
            // Compute all immediate successor states ("kids").
            // Detect and memoize any game-over kids (leaf states).
            // If all immediate successor states have been solved,
            //  Compute and memoize the tally for the old state.
            // Else,
            //  Push the old state back onto the stack.
            //  Push the unsolved successor states onto the stack.
            let kids: Vec<_> = successors(&old).collect();
            assert_eq!(27, kids.len()); // pow(3 universes per roll, 3 rolls per turn)
            for kid in &kids {
                assert!(kid.players[kid.turn].score < score);
                if kid.players[old.turn].score >= score {
                    memo.insert(kid.clone(), [[1, 0], [0, 1]][old.turn]);
                }
            }
            if kids.iter().all(|new| memo.contains_key(new)) {
                let tally = kids
                    .iter()
                    .map(|kid| memo[kid])
                    .fold([0, 0], |tally, kid| [tally[0] + kid[0], tally[1] + kid[1]]);
                memo.insert(old, tally);
            } else {
                stack.push(old);
                stack.extend(kids.into_iter().filter(|new| !memo.contains_key(new)));
            }
        }
    }
    let tally = memo[&init];
    tally[0].max(tally[1])
}

pub fn solve(puzzle: &Puzzle) -> usize {
    solve_score(puzzle, 21)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(444_356_092_776_315, solve(&Puzzle::new(4, 8)));
    }
}
