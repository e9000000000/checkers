use crate::board::{Board, Move, Side, Cell};


#[derive(Debug)]
struct ScoredMove {
    mv: Option<Move>,
    score: i8,
}

/// positive numbers is good position, negative numbers is bad position
#[inline(always)]
fn count_score(board: &Board, side: Side) -> i8 {
    let who_turn = board.who_turn();

    match board.who_win() {
        Some(x) if x == who_turn => 127,
        Some(x) if x != who_turn => -127,
        None => match side {
            Side::Black => {
                let our_checkers_amount = board.count(Cell::Black) as i8;
                let enemy_checkers_amount = board.count(Cell::White) as i8;
                return our_checkers_amount - enemy_checkers_amount;
            },
            Side::White => {
                let our_checkers_amount = board.count(Cell::White) as i8;
                let enemy_checkers_amount = board.count(Cell::Black) as i8;
                return our_checkers_amount - enemy_checkers_amount;
            },
        },
        Some(_) => unreachable!(),
    }
}


fn compute_best_move(board: &mut Board, depth: usize, start_score: i8) -> ScoredMove {
    let board_score = count_score(&board, board.who_turn());
    if depth == 0 {
        return ScoredMove {mv: None, score: board_score};
    }

    let mut best_mv = ScoredMove {
        mv: None,
        score: -127
    };

    let mvs = board.all_available_moves();
    let mvs_amount = mvs.len();
    if mvs_amount == 0 {
        return ScoredMove {mv: None, score: board_score};
    }

    for i in 0..mvs_amount {
        let mv = mvs[i];
        let mut test_board = board.clone();
        test_board.do_move_without_checks(mv);

        let best_enemy_move = compute_best_move(&mut test_board, depth - 1, start_score);

        let score = match test_board.who_turn() == board.who_turn() {
            true => best_enemy_move.score,
            false => best_enemy_move.score * -1,
        };

        if score > best_mv.score {
            best_mv.score = score;
            best_mv.mv = Some(mv);
        }
    }

    return best_mv;
}


/// dir can be -1 or 1 it is for best or words move chousing (1 for best, -1 for worst)
fn best_move(board: &mut Board, depth: usize) -> Option<Move> {
    let start_score = count_score(&board, board.who_turn());
    return compute_best_move(board, depth, start_score).mv;
}

pub fn chouse_move5(board: &mut Board) -> Option<Move> {
    return best_move(board, 5);
}


pub fn chouse_move10(board: &mut Board) -> Option<Move> {
    return best_move(board, 10);
}
