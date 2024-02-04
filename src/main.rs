use std::time;
use std::thread;

mod board;
mod player_random;
mod player_minmax;

fn main() {
    let sleep_time = time::Duration::from_millis(1000);
    let mut bd = board::Board::new();
    loop {
        println!("{}", "-".repeat(20));
        let move_result = match bd.who_turn() {
            board::Side::White => player_random::chouse_move(&bd),
            board::Side::Black => player_minmax::chouse_move(&bd),
        };
        match move_result {
            Some(mv) => bd.do_move(mv).unwrap(),
            None => break,
        }
        bd.print();

        thread::sleep(sleep_time);
    }
    bd.print();
}
