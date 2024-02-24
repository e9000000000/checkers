use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    White,
    Black,
    WhiteKing,
    BlackKing,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        return Self {x, y}
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Move {
    pub from: Point,
    pub to: Point,
}

impl Move {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
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
            Cell::White => write!(f, "w"),
            Cell::Black => write!(f, "b"),
            Cell::WhiteKing => write!(f, "W"),
            Cell::BlackKing => write!(f, "B"),
            Cell::Empty => write!(f, " "),
        }
    }
}

#[derive(Copy, Clone)]
pub enum State {
    WhiteTurn,
    BlackTurn,
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Side {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Board {
    field: [[Cell; 8]; 8],
    state: State,
    pub move_amount: usize,
    prev_turn_jump: Option<Point>,
    available_moves_exists: Option<bool>,
    white_amount: usize,
    black_amount: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: State::WhiteTurn,
            move_amount: 0,
            prev_turn_jump: None,
            available_moves_exists: None,
            white_amount: 12,
            black_amount: 12,
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

    #[allow(dead_code)]
    pub fn from_arr(state: State, arr: [[char; 8]; 8]) -> Self {
        Board {
            state,
            move_amount: 0,
            prev_turn_jump: None,
            available_moves_exists: None,
            white_amount: 12,
            black_amount: 12,
            field: arr.map(|row| row.map(|x| match x {
                'b' => Cell::Black,
                'w' => Cell::White,
                'W' => Cell::WhiteKing,
                'B' => Cell::BlackKing,
                _ => Cell::Empty,
            }))
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.field[y][x]
    }

    pub fn is_playable_cell(&self, x: usize, y: usize) -> bool {
        (9 * y + x) % 2 != 0
    }

    fn add_checker_jump_move_if_awailabel(&self, moves: &mut Moves, mv: Move) {
        match self.field[mv.to.y][mv.to.x] {
            Cell::Empty => match self.field[mv.from.y][mv.from.x] {
                Cell::White => match self.field[(mv.from.y + mv.to.y) / 2][(mv.from.x + mv.to.x) / 2] {
                    Cell::Black => moves.push(mv),
                    Cell::BlackKing => moves.push(mv),
                    _ => (),
                },
                Cell::Black => match self.field[(mv.from.y + mv.to.y) / 2][(mv.from.x + mv.to.x) / 2] {
                    Cell::White => moves.push(mv),
                    Cell::WhiteKing => moves.push(mv),
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

    // return true if jump possible available in same direction, otherwise false
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
            if !(self.add_jump_move_for_king(moves, x, y, check_x, check_y, 1, 1)) {
                break;
            }
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x <= 1 || check_y <= 1 {
                break;
            }
            check_x -= 1;
            check_y -= 1;
            if !(self.add_jump_move_for_king(moves, x, y, check_x, check_y, -1, -1)) {
                break;
            }
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x >= 6 || check_y <= 1 {
                break;
            }
            check_x += 1;
            check_y -= 1;
            if !(self.add_jump_move_for_king(moves, x, y, check_x, check_y, 1, -1)) {
                break;
            }
        }

        check_x = x;
        check_y = y;
        loop {
            if check_x <= 1 || check_y >= 6 {
                break;
            }
            check_x -= 1;
            check_y += 1;
            if !(self.add_jump_move_for_king(moves, x, y, check_x, check_y, -1, 1)) {
                break;
            }
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
        if self.white_amount == 0 {
            self.state = State::BlackWin;
        }
        if self.black_amount == 0 {
            self.state = State::WhiteWin;
        }

        if self.move_amount > 100 {
            self.state = State::Draw;
        }

        match self.available_moves_exists {
            Some(exists) => {
                if !exists {
                    match self.state {
                        State::WhiteTurn => self.state = State::BlackWin,
                        State::BlackTurn => self.state = State::WhiteWin,
                        _ => (),
                    }
                }
            },
            None => {
                let all_mvs = self.all_available_moves();
                if all_mvs.len() == 0 {
                    match self.state {
                        State::WhiteTurn => self.state = State::BlackWin,
                        State::BlackTurn => self.state = State::WhiteWin,
                        _ => (),
                    }
                }
            },
        }

        for x in 0..self.field[0].len() {
            match self.field[0][x] {
                Cell::White => {
                    self.field[0][x] = Cell::WhiteKing;
                },
                _ => (),
            }
        }
        for x in 0..self.field[7].len() {
            match self.field[7][x] {
                Cell::Black => {
                    self.field[7][x] = Cell::BlackKing;
                }
                _ => (),
            }
        }
    }

    pub fn available_moves_for_cell(&self, x: usize, y: usize) -> Moves {
        let mut all_forced_moves = Vec::with_capacity(10);
        let mut available_moves = Vec::with_capacity(20);
        self.add_forced_moves_for_all_checkers_and_kings(&mut all_forced_moves);
        if all_forced_moves.len() == 0 {
            self.add_normal_moves_for_checker_or_king(&mut available_moves, x, y);
        } else {
            for i in 0..all_forced_moves.len() {
                let mv = all_forced_moves[i];
                if mv.from == Point::new(x, y) {
                    available_moves.push(mv);
                }
            }
        }

        available_moves

    }

    pub fn all_available_moves(&mut self) -> Moves {
        let mut available_moves = Vec::with_capacity(40);
        self.add_forced_moves_for_all_checkers_and_kings(&mut available_moves);
        if available_moves.len() != 0 {
            return available_moves
        }
        
        for y in 0..8 {
            for x in ((1 - y % 2)..8).step_by(2) {
                self.add_normal_moves_for_checker_or_king(&mut available_moves, x, y);
            }
        }
        
        self.available_moves_exists = Some(available_moves.len() > 0);
        return available_moves
    }

    // return if it is a jump
    fn _do_move(&mut self, mv: Move) -> bool {
        let mut is_it_was_jump = false;

        self.field[mv.to.y][mv.to.x] = self.field[mv.from.y][mv.from.x];

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

        if is_it_was_jump {
            match self.state {
                State::WhiteTurn => self.black_amount -= 1,
                State::BlackTurn => self.white_amount -= 1,
                _ => (),
            }
        }

        return is_it_was_jump
    }

    pub fn do_move_without_checks(&mut self, mv: Move) {
        let is_it_was_jump = self._do_move(mv);

        let mut forced_to_jump_on_next_turn = false;
        self.prev_turn_jump = None;
        if is_it_was_jump {
            let mut jump_moves = vec![];
            self.add_jump_moves_for_checker_or_king(&mut jump_moves, mv.to.x, mv.to.y);

            if jump_moves.len() != 0 {
                self.prev_turn_jump = Some(mv.to);
                forced_to_jump_on_next_turn = true;
            }
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
    }

    pub fn do_move(&mut self, mv: Move) -> Result<(), &'static str> {
        let mut available_moves = vec![];
        self.add_forced_moves_for_all_checkers_and_kings(&mut available_moves);
        if available_moves.len() == 0 {
            self.add_normal_moves_for_checker_or_king(&mut available_moves, mv.from.x, mv.from.y);
        }

        match available_moves.contains(&mv) {
            true => {
                let is_it_was_jump = self._do_move(mv);

                let mut forced_to_jump_on_next_turn = false;
                self.prev_turn_jump = None;
                if is_it_was_jump {
                    let mut jump_moves = vec![];
                    self.add_jump_moves_for_checker_or_king(&mut jump_moves, mv.to.x, mv.to.y);

                    if jump_moves.len() != 0 {
                        self.prev_turn_jump = Some(mv.to);
                        forced_to_jump_on_next_turn = true;
                    }
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
            false => Err("move unavailable"),
        }
    }

    pub fn is_ended(&self) -> bool {
        match self.state {
            State::WhiteWin => true,
            State::BlackWin => true,
            State::Draw => true,
            _ => false,
        }
    }

    pub fn who_turn(&self) -> Side {
        match self.state {
            State::WhiteTurn => Side::White,
            State::BlackTurn => Side::Black,
            State::WhiteWin => Side::Black,
            State::BlackWin => Side::White,
            _ => Side::White,
        }
    }

    pub fn who_win(&self) -> Option<Side> {
        match self.state {
            State::WhiteWin => Some(Side::White),
            State::BlackWin => Some(Side::Black),
            _ => None,
        }
    }

    pub fn count(&self, cell_type: Cell) -> usize {
        match cell_type {
            Cell::Black => self.black_amount,
            Cell::White => self.white_amount,
            _ => {
                let mut result = 0;
                for y in 0..self.field.len() {
                    for x in 0..self.field[y].len() {
                        if self.field[y][x] == cell_type {
                            result += 1;
                        }
                    }
                }
                return result;
            },
        }
    }
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

    #[test]
    fn forced_to_jump_over_king_too() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'B', ' ', ' ', ' ', ' ', ' ', ' '],
            ['w', ' ', 'w', ' ', ' ', ' ', ' ', ' '],
        ]);
        assert!(board.do_move(Move::new(2, 7, 3, 6)).is_err());
    }

    #[test]
    fn forced_to_jump_after_enemy_double_jump() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', 'b', ' '],
            [' ', 'b', ' ', 'b', ' ', 'b', ' ', 'w'],
            ['w', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            ['w', ' ', 'w', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'w', ' ', 'w'],
            [' ', ' ', 'w', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(0, 3, 2, 1)), Ok(()));
        assert_eq!(board.do_move(Move::new(2, 1, 4, 3)), Ok(()));
        assert!(board.do_move(Move::new(3, 0, 4, 1)).is_err());
    }

    #[test]
    fn dont_delete_cell_after_jump() {
        let mut board = Board::from_arr(State::WhiteTurn, [
            [' ', ' ', ' ', 'b', ' ', 'b', ' ', ' '],
            [' ', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', ' ', ' ', 'w', ' ', 'b'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            ['b', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'w', ' ', ' ', ' ', ' ', ' ', 'w'],
            ['w', ' ', 'w', ' ', ' ', ' ', 'w', ' '],
        ]);
        assert_eq!(board.do_move(Move::new(5, 2, 7, 0)), Ok(()));
        assert_eq!(board.field[0][7], Cell::WhiteKing);
    }

    #[test]
    fn king_cant_jump_over_two_in_a_row() {
        let mut board = Board::from_arr(State::BlackTurn, [
            [' ', 'b', ' ', ' ', ' ', 'b', ' ', 'b'],
            ['b', ' ', 'b', ' ', 'b', ' ', 'b', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'b'],
            ['b', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'w', ' ', ' '],
            [' ', ' ', ' ', ' ', 'w', ' ', ' ', ' '],
            [' ', ' ', ' ', 'w', ' ', ' ', ' ', ' '],
            ['w', ' ', 'B', ' ', 'w', ' ', 'w', ' '],
        ]);
        assert!(board.do_move(Move::new(2, 7, 6, 3)).is_err());
    }
}
