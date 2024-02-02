use std::fmt;
use std::time;
use std::thread;
use std::process;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    White,
    Black,
    WhiteKing,
    BlackKing,
    Empty,
}

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq)]
struct Move {
    from: Point,
    to: Point,
}

impl Move {
    fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        Move {
            from: Point {x: from_x, y: from_y},
            to: Point {x: to_x, y: to_y},
        }
    }
}

type Moves = Vec<Move>;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::White => write!(f, "\x1b[38;5;252m◯\x1b[0m"),
            Cell::Black => write!(f, "\x1b[38;5;196m◯\x1b[0m"),
            Cell::WhiteKing => write!(f, "\x1b[38;5;252◷\x1b[0m"),
            Cell::BlackKing => write!(f, "\x1b[38;5;196m◷\x1b[0m"),
            Cell::Empty => write!(f, "\x1b[38;5;60m.\x1b[0m"),
        }
    }
}

enum State {
    WhiteTurn,
    BlackTurn,
    WhiteWin,
    BlackWin,
    Draw,
}

struct Board {
    field: [[Cell; 8]; 8],
    state: State,
    move_amount: usize,
    prev_turn_jump: Option<Point>,
}

impl Board {
    fn new() -> Self {
        Board {
            state: State::WhiteTurn,
            move_amount: 0,
            prev_turn_jump: None,
            field: [
                [Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black],
                [Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty],
                [Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black, Cell::Empty, Cell::Black],
                [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty],
                [Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White],
                [Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty, Cell::White, Cell::Empty],
            ],
        }
    }

    fn from_arr(state: State, arr: [[char; 8]; 8]) -> Self {
        Board {
            state,
            move_amount: 0,
            prev_turn_jump: None,
            field: arr.map(|row| row.map(|x| match x {
                'b' => Cell::Black,
                'w' => Cell::White,
                'W' => Cell::WhiteKing,
                'B' => Cell::BlackKing,
                _ => Cell::Empty,
            }))
        }
    }

    fn print(&self) {
        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                print!(" {}", self.field[y][x]);
            }
            println!();
        }
    }

    fn add_checker_jump_move_if_awailabel(&self, moves: &mut Moves, mv: Move) {
        match self.field[mv.to.y][mv.to.x] {
            Cell::Empty => match self.field[mv.from.y][mv.from.x] {
                Cell::White => match self.field[(mv.from.y + mv.to.y) / 2][(mv.from.x + mv.to.x) / 2] {
                    Cell::Black => moves.push(mv),
                    _ => (),
                },
                Cell::Black => match self.field[(mv.from.y + mv.to.y) / 2][(mv.from.x + mv.to.x) / 2] {
                    Cell::White => moves.push(mv),
                    _ => (),
                },
                _ => (),
            },
            _ => (),

        }
    }

    fn add_jump_moves_for_checker(&self, moves: &mut Moves, x: usize, y: usize) {
        if x > 1 && y > 1 {
            self.add_checker_jump_move_if_awailabel(moves, Move::new(x, y, x - 2, y - 2));
        }
        if x > 1 && y < 6 {
            self.add_checker_jump_move_if_awailabel(moves, Move::new(x, y, x - 2, y + 2));
        }
        if x < 6 && y > 1 {
            self.add_checker_jump_move_if_awailabel(moves, Move::new(x, y, x + 2, y - 2));
        }
        if x < 6 && y < 6 {
            self.add_checker_jump_move_if_awailabel(moves, Move::new(x, y, x + 2, y + 2));
        }
    }

    // return true if jump possible awailable in same direction, otherwise false
    fn add_jump_move_for_king(&self, moves: &mut Moves, x: usize, y: usize, check_x: usize, check_y: usize, dir_x: i32, dir_y: i32) -> bool {
        let enemy_checker = match self.field[y][x] {
            Cell::White => Cell::Black,
            Cell::WhiteKing => Cell::Black,
            Cell::Black => Cell::White,
            Cell::BlackKing => Cell::White,
            Cell::Empty => unreachable!(),
        };
        let enemy_king = match self.field[y][x] {
            Cell::White => Cell::BlackKing,
            Cell::WhiteKing => Cell::BlackKing,
            Cell::Black => Cell::WhiteKing,
            Cell::BlackKing => Cell::WhiteKing,
            Cell::Empty => unreachable!(),
        };

        match self.field[check_y][check_x] {
            Cell::Empty => true,
            cell if cell == enemy_checker || cell == enemy_king => {
                let mut jump_end_x = check_x;
                let mut jump_end_y = check_y;
                loop {
                    if (dir_x > 0 && jump_end_x >= 7) ||
                       (dir_y > 0 && jump_end_y >= 7) ||
                       (dir_x < 0 && jump_end_x <= 0) ||
                       (dir_y < 0 && jump_end_y <= 0) {
                        break;
                    }
                    jump_end_x = (jump_end_x as i32 + dir_x) as usize;
                    jump_end_y = (jump_end_y as i32 + dir_y) as usize;

                    match self.field[jump_end_y][jump_end_x] {
                        Cell::Empty => {
                            moves.push(Move::new(x, y, jump_end_x, jump_end_y));
                        },
                        _ => break,
                    };
                }

                false
            },
            _ => false,
        }
    }

    fn add_jump_moves_for_king(&self, moves: &mut Moves, x: usize, y: usize) {
        let mut check_x = x;
        let mut check_y = y;
        loop {
            if check_x >= 6 || check_y >= 6 {
                break;
            }
            check_x += 1;
            check_y += 1;
            self.add_jump_move_for_king(moves, x, y, check_x, check_y, 1, 1);
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x <= 1 || check_y <= 1 {
                break;
            }
            check_x -= 1;
            check_y -= 1;
            self.add_jump_move_for_king(moves, x, y, check_x, check_y, -1, -1);
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x >= 6 || check_y <= 1 {
                break;
            }
            check_x += 1;
            check_y -= 1;
            self.add_jump_move_for_king(moves, x, y, check_x, check_y, 1, -1);
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x <= 1 || check_y >= 6 {
                break;
            }
            check_x -= 1;
            check_y += 1;
            self.add_jump_move_for_king(moves, x, y, check_x, check_y, -1, 1);
        }
    }

    fn add_jump_moves_for_checker_or_king(&self, moves: &mut Moves, x: usize, y: usize) {
        match self.state {
            State::WhiteTurn => match self.field[y][x] {
                Cell::White => {
                    self.add_jump_moves_for_checker(moves, x, y);
                },
                Cell::WhiteKing => {
                    self.add_jump_moves_for_king(moves, x, y);
                },
                _ => (),
            },
            State::BlackTurn => match self.field[y][x] {
                Cell::Black => {
                    self.add_jump_moves_for_checker(moves, x, y);
                },
                Cell::BlackKing => {
                    self.add_jump_moves_for_king(moves, x, y);
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn add_normal_moves_for_checker(&self, moves: &mut Moves, x: usize, y: usize, to_y: usize) {
        if x > 0 {  // not beside left border
            let to_x = x - 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => (),
            }
        }

        if x < 7 {  // not beside right border
            let to_x = x + 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => (),
            }
        }
    }

    fn add_normal_moves_for_king(&self, moves: &mut Moves, x: usize, y: usize) {
        let mut to_x = x;
        let mut to_y = y;
        loop {
            if to_x >= 7 || to_y >= 7 {
                break;
            }
            to_x += 1;
            to_y += 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => break,
            }
        }

        to_x = x;
        to_y = y;
        loop {
            if to_x == 0 || to_y == 0 {
                break;
            }
            to_x -= 1;
            to_y -= 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => break,
            }
        }

        to_x = x;
        to_y = y;
        loop {
            if to_x >= 7 || to_y == 0 {
                break;
            }
            to_x += 1;
            to_y -= 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => break,
            }
        }

        to_x = x;
        to_y = y;
        loop {
            if to_x == 0 || to_y >= 7 {
                break;
            }
            to_x -= 1;
            to_y += 1;
            match self.field[to_y][to_x] {
                Cell::Empty => moves.push(Move::new(x, y, to_x, to_y)),
                _ => break,
            }
        }
    }

    fn add_normal_moves_for_checker_or_king(&self, moves: &mut Moves, x: usize, y: usize) {
        match self.state {
            State::WhiteTurn => match self.field[y][x] {
                Cell::White => {
                    self.add_normal_moves_for_checker(moves, x, y, y - 1);
                },
                Cell::WhiteKing => {
                    self.add_normal_moves_for_king(moves, x, y);
                },
                _ => (),
            },
            State::BlackTurn => match self.field[y][x] {
                Cell::Black => {
                    self.add_normal_moves_for_checker(moves, x, y, y + 1);
                },
                Cell::BlackKing => {
                    self.add_normal_moves_for_king(moves, x, y);
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn add_forced_moves_for_all_checkers_and_kings(&self, moves: &mut Moves) {
        match self.prev_turn_jump {
            Some(p) => self.add_jump_moves_for_checker_or_king(moves, p.x, p.y),
            None => {
                for y in 0..self.field.len() {
                    for x in 0..self.field[y].len() {
                        self.add_jump_moves_for_checker_or_king(moves, x, y)
                    }
                }
            },
        }
    }

    fn update_after_move(&mut self) {
        let mut is_white_on_board = false;
        let mut is_black_on_board = false;

        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                match self.field[y][x] {
                    Cell::White => is_white_on_board = true,
                    Cell::WhiteKing => is_white_on_board = true,
                    Cell::Black => is_black_on_board = true,
                    Cell::BlackKing => is_black_on_board = true,
                    Cell::Empty => (),
                }
            }
        }

        if !is_white_on_board {
            self.state = State::BlackWin;
        }
        if !is_black_on_board {
            self.state = State::WhiteWin;
        }

        if self.move_amount > 100 {
            self.state = State::Draw;
        }

        if self.all_awailable_moves().len() == 0 {
            match self.state {
                State::WhiteTurn => self.state = State::BlackWin,
                State::BlackTurn => self.state = State::WhiteWin,
                _ => (),
            }
        }

        for x in 0..self.field[0].len() {
            match self.field[0][x] {
                Cell::White => self.field[0][x] = Cell::WhiteKing,
                _ => (),
            }
        }
        for x in 0..self.field[7].len() {
            match self.field[7][x] {
                Cell::Black => self.field[7][x] = Cell::BlackKing,
                _ => (),
            }
        }
    }

    fn all_awailable_moves(&self) -> Moves {
        let mut awailable_moves = vec![];
        self.add_forced_moves_for_all_checkers_and_kings(&mut awailable_moves);
        if awailable_moves.len() != 0 {
            return awailable_moves
        }

        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                self.add_normal_moves_for_checker_or_king(&mut awailable_moves, x, y);
            }
        }

        return awailable_moves
    }

    fn do_move(&mut self, mv: Move) -> Result<(), &'static str> {
        let mut awailable_moves = vec![];
        self.add_forced_moves_for_all_checkers_and_kings(&mut awailable_moves);
        if awailable_moves.len() == 0 {
            self.add_normal_moves_for_checker_or_king(&mut awailable_moves, mv.from.x, mv.from.y);
        }

        match awailable_moves.contains(&mv) {
            true => {
                self.field[mv.to.y][mv.to.x] = self.field[mv.from.y][mv.from.x];

                let mut is_it_was_jump = false;
                let dir_y = match mv.to.y > mv.from.y {
                    true => 1,
                    false => -1,
                };
                let dir_x = match mv.to.x > mv.from.x {
                    true => 1,
                    false => -1,
                };
                let mut y = mv.from.y;
                let mut x = mv.from.x;
                loop {
                    if y == mv.to.y || x == mv.to.x {
                        break;
                    }

                    if self.field[y][x] != Cell::Empty && (y != mv.from.y || x != mv.from.x) {
                        is_it_was_jump = true;
                    }
                    self.field[y][x] = Cell::Empty;

                    x = (x as i32 + dir_x) as usize;
                    y = (y as i32 + dir_y) as usize;
                }

                let mut forced_to_jump_on_next_turn = false;
                if is_it_was_jump {
                    self.prev_turn_jump = Some(mv.to);
                    let mut jump_moves = vec![];
                    self.add_jump_moves_for_checker_or_king(&mut jump_moves, mv.to.x, mv.to.y);

                    if jump_moves.len() != 0 {
                        forced_to_jump_on_next_turn = true;
                    }
                } else {
                    self.prev_turn_jump = None;
                }

                if !forced_to_jump_on_next_turn {
                    self.state = match self.state {
                        State::WhiteTurn => State::BlackTurn,
                        State::BlackTurn => State::WhiteTurn,
                        _ => unreachable!(),
                    };
                }

                self.move_amount += 1;
                self.update_after_move();
                Ok(())
            },
            false => Err("move unawailable"),
        }
    }
}

fn main() {
    let sleep_time = time::Duration::from_millis(1000);
    let mut board = Board::new();
    loop {
        process::Command::new("clear").status().unwrap();
        board.print();
        let mvs = board.all_awailable_moves();
        let mv_i = rand::random::<usize>() % mvs.len();
        let mv = mvs[mv_i];
        board.do_move(mv).unwrap();

        match board.state {
            State::WhiteWin => break,
            State::BlackWin => break,
            State::Draw => break,
            _ => (),
        }

        thread::sleep(sleep_time);
    }
    board.print();

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_first_move() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(0, 5, 1, 4)), Ok(()));
    }

    #[test]
    fn black_first_move() {
        let mut board = Board::new();
        assert!(board.do_move(Move::new(0, 2, 1, 3)).is_err());
    }

    #[test]
    fn empty_cell_move() {
        let mut board = Board::new();
        assert!(board.do_move(Move::new(0, 3, 1, 4)).is_err());
    }

    #[test]
    fn move_to_white_cells() {
        let mut board = Board::new();
        assert!(board.do_move(Move::new(0, 5, 0, 4)).is_err());
    }

    #[test]
    fn move_to_ally_checker_position() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert!(board.do_move(Move::new(2, 5, 1, 4)).is_err());
    }

    #[test]
    fn move_to_enemy_checker_position() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert!(board.do_move(Move::new(1, 4, 2, 3)).is_err());
    }

    #[test]
    fn move_to_self_position() {
        let mut board = Board::new();
        assert!(board.do_move(Move::new(0, 5, 0, 5)).is_err());
    }

    #[test]
    fn move_back() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert!(board.do_move(Move::new(1, 4, 0, 5)).is_err());
    }

    #[test]
    fn move_two_cells_forward() {
        let mut board = Board::new();
        assert!(board.do_move(Move::new(0, 5, 2, 3)).is_err());
    }

    #[test]
    fn can_jump() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', 'w', ' ', ' ', ' ', ' '],
            ['w', ' ', ' ', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(3, 4, 1, 2)), Ok(()));
    }

    #[test]
    fn forced_to_jump() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', 'w', ' ', ' ', ' ', ' '],
            ['w', ' ', ' ', ' ', 'w', ' ', 'w', ' '],
            [' ', 'w', ' ', 'w', ' ', 'w', ' ', 'w'],
            ['w', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert!(board.do_move(Move::new(6, 5, 7, 4)).is_err());
    }


    #[test]
    fn jump_two_in_a_row() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'b', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'b', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(5, 5, 3, 3)), Ok(()));
        assert_eq!(board.do_move(Move::new(3, 3, 5, 1)), Ok(()));
    }

    #[test]
    fn jump_backwards() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'w', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'b', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(4, 4, 6, 6)), Ok(()));
    }

    #[test]
    fn king_moves() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            ['W', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(0, 0, 6, 6)), Ok(()));
        assert_eq!(board.do_move(Move::new(7, 0, 6, 1)), Ok(()));
        assert_eq!(board.do_move(Move::new(6, 6, 0, 0)), Ok(()));
    }

    #[test]
    fn king_cant_moves() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            ['W', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert!(board.do_move(Move::new(0, 0, 6, 7)).is_err());
    }

    #[test]
    fn king_long_jump_one() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'b', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(0, 7, 7, 0)), Ok(()));
    }

    #[test]
    fn king_force_jump_one() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'b', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert!(board.do_move(Move::new(0, 7, 1, 6)).is_err());
    }

    #[test]
    fn king_jump_two() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'B', ' '],
            [' ', ' ', ' ', 'b', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(0, 7, 5, 2)), Ok(()));
        assert_eq!(board.do_move(Move::new(5, 2, 7, 4)), Ok(()));
    }

    #[test]
    fn chear_cell_after_jump_over_it() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'b', ' ', ' ', ' ', ' ', ' ', ' '],
            ['w', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.field[6][1], Cell::Black);
        assert_eq!(board.do_move(Move::new(0, 7, 2, 5)), Ok(()));
        assert_eq!(board.field[6][1], Cell::Empty);
    }

    #[test]
    fn after_jump_with_one_piece_cant_eat_with_another_in_a_row() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            [' ', 'b', ' ', ' ', ' ', ' ', ' ', ' '],
            ['w', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'w'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(7, 6, 5, 4)), Ok(()));
        assert!(board.do_move(Move::new(0, 5, 2, 3)).is_err());
    }

    #[test]
    fn cant_jump_after_normal_move() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', 'b', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', 'w', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(3, 4, 2, 3)), Ok(()));
        assert!(board.do_move(Move::new(2, 3, 4, 1)).is_err());
    }

    // TODO it can not jump when should be forsed sometimes
}
