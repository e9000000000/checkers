use std::time;
use std::thread;
use crate::board::Board;

mod board;

fn main() {
    let sleep_time = time::Duration::from_millis(1000);
    let mut board = Board::new();
    loop {
        println!("{}", "-".repeat(20));
        board.print();
        let mvs = board.all_awailable_moves();
        let mv_i = rand::random::<usize>() % mvs.len();
        let mv = mvs[mv_i];
        board.do_move(mv).unwrap();

        if board.is_ended() {
            break;
        }

        thread::sleep(sleep_time);
    }
    board.print();
}
