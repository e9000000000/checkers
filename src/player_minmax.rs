use crate::board::{Board, Move, Side, Cell};


#[derive(Debug)]
struct ScoredMove {
    mv: Move,
    score: i8,
}

/// positive numbers is good position, negative numbers is bad position
fn count_score(board: &Board, side: Side, king_multiplier: i8) -> i8 {
    match side {
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
    }
}


fn moves_with_scores(board: &Board, depth: usize, king_multiplier: i8) -> Vec<ScoredMove> {
    let mut scored_moves = vec![];
    let mvs = board.all_available_moves();
    if mvs.len() == 0 {
        return scored_moves;
    }

    for i in 0..mvs.len() {
        let mv = mvs[i];
        let mut test_board = board.clone();
        test_board.do_move(mv).unwrap();
        let score = match test_board.who_win() {
            Some(x) if x == board.who_turn() => 127,
            Some(x) if x != board.who_turn() => -128,
            None => {
                match depth {
                    0 => count_score(&test_board, board.who_turn(), king_multiplier),
                    _ => {
                        let new_depth = match mvs.len() {
                            1 => depth,
                            _ => depth - 1,
                        };
                        let enemy_moves = moves_with_scores(&test_board, new_depth, king_multiplier);
                        let mut best_enemy_move_score = -128;
                        for j in 0..enemy_moves.len() {
                            if enemy_moves[j].score > best_enemy_move_score {
                                best_enemy_move_score = enemy_moves[j].score;
                            }
                        }
                        match test_board.who_turn() == board.who_turn() {
                            true => best_enemy_move_score,
                            false => best_enemy_move_score * -1,
                        }
                    }
                }
            },
            Some(_) => unreachable!(),
        };
        scored_moves.push(ScoredMove {mv, score});
    }

    return scored_moves;
}


/// dir can be -1 or 1 it is for best or words move chousing (1 for best, -1 for worst)
fn best_move(board: &Board, depth: usize, king_multiplier: i8) -> Option<Move> {
    let scored_moves = moves_with_scores(&board, depth, king_multiplier);
    if scored_moves.len() == 0 {
        return None;
    } else {
        let mut best_move_index = 0;
        let mut best_move_score = -128;
        for i in 0..scored_moves.len() {
            if scored_moves[i].score > best_move_score {
                best_move_score = scored_moves[i].score;
                best_move_index = i;
            }
        }
        return Some(scored_moves[best_move_index].mv);
    }
}

pub fn chouse_move5(board: &Board) -> Option<Move> {
    return best_move(board, 5, 3);
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
