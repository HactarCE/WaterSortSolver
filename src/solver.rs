use priority_queue::PriorityQueue;
use std::collections::HashSet;

use crate::puzzle::{Move, Puzzle};

pub fn solve(initial: Puzzle) -> Result<Vec<Move>, &'static str> {
    let mut seen = HashSet::new();
    let mut queue = PriorityQueue::new();
    seen.insert(initial.clone());
    queue.push((initial, vec![]), 0);

    let mut depth = 0;
    let mut vials_solved = 0;

    while let Some(((puzzle, moves), vs)) = queue.pop() {
        if puzzle.is_solved() {
            return Ok(moves);
        }

        if moves.len() == depth {
            depth += 1;
            println!("... to depth {:?}", depth);
        }

        if vs > vials_solved {
            vials_solved = vs;
            println!("... solved {} vials!", vials_solved);
            let mut first = true;
            for m in &moves {
                if first {
                    first = false;
                } else {
                    print!(", ");
                }
                print!("{}", m);
            }
            println!();
        }

        for (additional_move, new_puzzle) in puzzle.gen_all_moves() {
            if !seen.contains(&new_puzzle) {
                let mut new_moves = moves.clone();
                new_moves.push(additional_move);
                seen.insert(new_puzzle.clone());
                let priority = new_puzzle.vials_solved();
                queue.push((new_puzzle, new_moves), priority);
            }
        }
    }

    Err("no solution")
}
