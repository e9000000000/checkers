use crate::board::{Board, Move};


pub fn chouse_move(board: &mut Board) -> Option<Move> {
    let mvs = board.all_available_moves();
    if mvs.len() == 0 {
        return None;
    }
    let mv_i = rand::random::<usize>() % mvs.len();
    return Some(mvs[mv_i]);
}
