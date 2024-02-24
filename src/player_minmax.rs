use crate::board::{Board, Move, Side, Cell};


#[derive(Debug)]
struct ScoredMove {
    mv: Option<Move>,
    score: i8,
}

/// positive numbers is good position, negative numbers is bad position
#[inline(always)]
fn count_score(board: &Board) -> i8 {
    let who_turn = board.who_turn();

    match board.who_win() {
        Some(x) if x == who_turn => 100,
        Some(x) if x != who_turn => -100,
        None => (board.count(Cell::White) - board.count(Cell::Black)) as i8,
        Some(_) => unreachable!(),
    }
}


fn compute_best_move(board: &mut Board, depth: usize, incoming_alpha: i8, incoming_beta: i8, play_as_white: bool) -> ScoredMove {
    let board_score = count_score(&board);
    if depth == 0 {
        return ScoredMove {mv: None, score: board_score};
    }

    let mut best_mv = ScoredMove {
        mv: None,
        score: if play_as_white {
            -127
        } else {
            127
        },
    };

    let mvs = board.all_available_moves();
    let mvs_amount = mvs.len();
    if mvs_amount == 0 {
        return ScoredMove {mv: None, score: board_score};
    }
    
    let mut alpha = incoming_alpha;
    let mut beta = incoming_beta;
        
    for i in 0..mvs_amount {
        let mv = mvs[i];
        let mut test_board = board.clone();
        test_board.do_move_without_checks(mv);

        let next_best_move;
        if test_board.who_turn() == board.who_turn() {
            next_best_move = compute_best_move(&mut test_board, depth - 1, alpha, beta, play_as_white);
        } else {
            next_best_move = compute_best_move(&mut test_board, depth - 1, alpha, beta, !play_as_white);
        
            if play_as_white {
                if next_best_move.score > alpha {
                    alpha = next_best_move.score;
                }
            } else {
                if next_best_move.score < beta {
                    beta = next_best_move.score;
                }
            }
        }

        if (play_as_white && next_best_move.score > best_mv.score) || (!play_as_white && next_best_move.score < best_mv.score) {
            best_mv.score = next_best_move.score;
            best_mv.mv = Some(mv);
        }

        if beta <= alpha {
            break;
        }
    }

    return best_mv;
}


/// dir can be -1 or 1 it is for best or words move chousing (1 for best, -1 for worst)
pub fn best_move(board: &mut Board, depth: usize) -> Option<Move> {
    match board.who_turn() {
        Side::White => return compute_best_move(board, depth, -127, 127, true).mv,
        Side::Black => return compute_best_move(board, depth, -127, 127, false).mv,
    }
}

pub fn chouse_move5(board: &mut Board) -> Option<Move> {
    best_move(board, 5)
}

pub fn chouse_move10(board: &mut Board) -> Option<Move> {
    best_move(board, 10)
}

pub fn chouse_move15(board: &mut Board) -> Option<Move> {
    best_move(board, 15)
}