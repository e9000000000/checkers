use crate::board::{Board, Move, Side, Cell};


#[derive(Debug)]
struct ScoredMove {
    mv: Option<Move>,
    score: i8,
}

/// positive numbers is good position, negative numbers is bad position
fn count_score(board: &Board, side: Side, king_multiplier: i8) -> i8 {
    let who_turn = board.who_turn();

    match board.who_win() {
        Some(x) if x == who_turn => 127,
        Some(x) if x != who_turn => -127,
        None => match side {
            Side::Black => {
                let our_checkers_amount = board.count(Cell::Black) as i8;
                let our_kings_amount = board.count(Cell::BlackKing) as i8;
                let enemy_checkers_amount = board.count(Cell::White) as i8;
                let enemy_kings_amount = board.count(Cell::WhiteKing) as i8;
                return (our_checkers_amount + our_kings_amount * king_multiplier) - (enemy_checkers_amount + enemy_kings_amount * king_multiplier);
            },
            Side::White => {
                let our_checkers_amount = board.count(Cell::White) as i8;
                let our_kings_amount = board.count(Cell::WhiteKing) as i8;
                let enemy_checkers_amount = board.count(Cell::Black) as i8;
                let enemy_kings_amount = board.count(Cell::BlackKing) as i8;
                return (our_checkers_amount + our_kings_amount * king_multiplier) - (enemy_checkers_amount + enemy_kings_amount * king_multiplier);
            },
        },
        Some(_) => unreachable!(),
    }
}


fn compute_best_move(board: &Board, depth: usize, king_multiplier: i8, start_score: i8, start_depth: usize) -> ScoredMove {
    let board_score = count_score(&board, board.who_turn(), king_multiplier);
    if board_score < -3 {
        return ScoredMove {mv: None, score: board_score};
    }
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
        test_board.do_move(mv).unwrap();

        let new_depth;
        let depth_substract = mvs_amount / 2;
        if depth_substract <= depth {
            new_depth = depth - mvs_amount / 2;
        } else {
            new_depth = 0;
        }

        let best_enemy_move = compute_best_move(&test_board, new_depth, king_multiplier, start_score, start_depth);

        let score = match test_board.who_turn() == board.who_turn() {
            true => best_enemy_move.score,
            false => best_enemy_move.score * -1,
        };

        if score > best_mv.score {
            best_mv.score = score;
            best_mv.mv = Some(mv);
        }

        if score > start_score {
            break
        }
    }

    return best_mv;
}


/// dir can be -1 or 1 it is for best or words move chousing (1 for best, -1 for worst)
fn best_move(board: &Board, depth: usize, king_multiplier: i8) -> Option<Move> {
    let start_score = count_score(&board, board.who_turn(), king_multiplier);
    return compute_best_move(&board, depth, king_multiplier, start_score, depth).mv;
}

pub fn chouse_move5(board: &Board) -> Option<Move> {
    return best_move(board, 5, 3);
}


pub fn chouse_move6(board: &Board) -> Option<Move> {
    return best_move(board, 30, 3);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::State;


    #[test]
    fn count_advantage_white() {
        let board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            ['w', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(count_score(&board, Side::White, 33), 1);
    }

    #[test]
    fn count_advantage_black() {
        let board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            ['w', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(count_score(&board, Side::Black, 33), -1);
    }

    #[test]
    fn avoid_be_jumped_over() {
        let board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            ['w', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(best_move(&board, 2, 1), Some(Move::new(5, 2, 4, 1)))
    }


    #[test]
    fn try_not_play_worst_as_black() {
        let board = Board::from_arr(State::BlackTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'w', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'B', ' '],
        ]);
        assert_ne!(best_move(&board, 3, 1), Some(Move::new(6, 7, 5, 6)))
    }

    #[test]
    fn try_not_play_worst_as_white() {
        let board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'W', ' '],
        ]);
        assert_ne!(best_move(&board, 3, 1), Some(Move::new(6, 7, 5, 6)))
    }
}
