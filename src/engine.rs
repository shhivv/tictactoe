use crate::{Eval, Marker, Move, Position};

pub fn find_best_move(position: Position) -> (Eval, Move) {
    minimax(position, 9, Eval::MIN, Eval::MAX, true)
}

fn minimax(
    position: Position,
    depth: u8,
    mut alpha: Eval,
    mut beta: Eval,
    maximize: bool,
) -> (Eval, Move) {
    let mut best_move = 0;

    if depth == 0 || position.winning().is_some() || position.draw() {
        return (eval(position, depth, maximize), best_move);
    }

    if maximize {
        let mut max_eval = Eval::MIN;

        for (i, possible_move) in position.0.iter().enumerate() {
            if possible_move == &Marker::Empty {
                let new_eval =
                    minimax(position.clone().make_move(i), depth - 1, alpha, beta, false);
                if new_eval.0 > max_eval {
                    max_eval = new_eval.0 as Eval;
                    best_move = i;
                    if new_eval.0 > alpha {
                        alpha = new_eval.0 as Eval;
                    }
                }

                if beta <= alpha {
                    break;
                }
            }
        }

        return (max_eval, best_move);
    }
    let mut min_eval = Eval::MAX;
    for (i, possible_move) in position.0.iter().enumerate() {
        if possible_move == &Marker::Empty {
            let new_eval = minimax(position.clone().make_move(i), depth - 1, alpha, beta, true);
            if new_eval.0 < min_eval {
                min_eval = new_eval.0 as Eval;
                best_move = i;
                if new_eval.0 < beta {
                    beta = new_eval.0 as Eval;
                }
            }

            if beta <= alpha {
                break;
            }
        }
    }

    (min_eval, best_move)
}

fn eval(position: Position, depth: u8, maximize: bool) -> Eval {
    if position.winning().is_some() {
        let score = 100 - i64::from(depth);
        if maximize {
            return -score;
        }
        return score;
    }
    0
}
