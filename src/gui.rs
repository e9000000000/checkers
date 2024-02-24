use crate::board;
use crate::player_random;
use crate::player_minmax;

#[derive(PartialEq, Copy, Clone)]
enum GameMode {
    SelfPlay,
    Random,
    MinMax5,
    MinMax10,
    MinMax15,
    RandomVsRandom,
}


pub struct App {
    show_game_ended_popup: bool,
    player_side: board::Side,
    game_mode: GameMode,
    bd: board::Board,
    highlighted: Vec<board::Point>,
    selected_cell: Option<board::Point>,
}


impl App {
    pub fn new() -> Self {
        let mut bd = Self {
            show_game_ended_popup: false,
            player_side: board::Side::White,
            game_mode: GameMode::SelfPlay,
            bd: board::Board::new(),
            highlighted: vec![],
            selected_cell: None,
        };
        bd.highlight_available_checkers_to_move();
        return bd;
    }

    fn restart(&mut self) {
        self.show_game_ended_popup = false;
        self.bd = board::Board::new();
        self.selected_cell = None;
        self.enemy_try_move();
        self.highlight_available_checkers_to_move();
    }

    fn render_coordinate(&self, layout: &egui::Layout, ui: &mut egui::Ui, text: String) {
        ui.with_layout(*layout, |ui| {
            ui.label(text);
        });

    }

    fn render_cell(&mut self, ui: &mut egui::Ui, x: usize, y: usize) {
        let color: egui::Color32;
        if self.highlighted.contains(&board::Point::new(x, y)) {
            color = egui::Color32::DARK_GREEN;
        } else {
            if self.bd.is_playable_cell(x, y) {
                color = egui::Color32::BLACK;
            } else {
                color = egui::Color32::DARK_GRAY;
            }
        }

        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            ui.scope(|ui| {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = color;

                let image = match self.bd.get_cell(x, y) {
                    board::Cell::Empty => None,
                    board::Cell::Black => Some(egui::Image::from_uri("file://assets/black_checker.png")),
                    board::Cell::BlackKing => Some(egui::Image::from_uri("file://assets/black_king.png")),
                    board::Cell::White => Some(egui::Image::from_uri("file://assets/white_checker.png")),
                    board::Cell::WhiteKing => Some(egui::Image::from_uri("file://assets/white_king.png")),
                };

                if ui.add(egui::Button::opt_image_and_text(image, None)).clicked() {
                    self.on_click(x, y);
                }
            });
        });
    }

    fn highlight_available_checkers_to_move(&mut self) {
        let available_moves = self.bd.all_available_moves();
        self.highlighted = vec![];
        for i in 0..available_moves.len() {
            self.highlighted.push(available_moves[i].from)
        }
    }

    fn enemy_try_move(&mut self) {
        match self.game_mode {
            GameMode::SelfPlay => (),
            GameMode::RandomVsRandom => {
                while !self.bd.is_ended() {
                    let chouse_result = player_random::chouse_move(&mut self.bd);
                    match chouse_result {
                        Some(mv) => self.bd.do_move(mv).unwrap(),
                        None => (),
                    };
                }
            },
            gm => {
                let chouse_func = match gm {
                    GameMode::Random => player_random::chouse_move,
                    GameMode::MinMax5 => player_minmax::chouse_move5,
                    GameMode::MinMax10 => player_minmax::chouse_move10,
                    GameMode::MinMax15 => player_minmax::chouse_move15,
                    _ => unreachable!(),
                };

                while self.player_side != self.bd.who_turn() && !self.bd.is_ended() {
                    let chouse_result = chouse_func(&mut self.bd);
                    match chouse_result {
                        Some(mv) => self.bd.do_move(mv).unwrap(),
                        None => (),
                    };
                }
            },
        }
    }

    fn show_game_ended_popup_if_game_ended(&mut self) {
        if self.bd.is_ended() {
            self.show_game_ended_popup = true;
        }
    }

    /// return true if move success else - return false
    fn try_move(&mut self, x: usize, y: usize) -> bool {
        match self.selected_cell {
            None => false,
            Some(selected_cell) => {
                let destination_point = board::Point::new(x, y);
                for i in 0..self.highlighted.len() {
                    let highlighted_cell = self.highlighted[i];
                    if highlighted_cell != destination_point {
                        continue;
                    }

                    self.bd.do_move(board::Move::new(selected_cell.x, selected_cell.y, x, y)).unwrap();
                    self.enemy_try_move();
                    self.highlight_available_checkers_to_move();
                    return true;
                }
                return false;
            },
        }
    }

    fn on_click(&mut self, x: usize, y: usize) {
        match self.bd.get_cell(x, y) {
            board::Cell::Empty => {
                if !self.try_move(x, y) {
                    self.highlight_available_checkers_to_move();
                }
            },
            _ => {
                let available_moves = self.bd.available_moves_for_cell(x, y);

                match available_moves.len() {
                    0 => {
                        self.highlight_available_checkers_to_move();
                    },
                    _ => {
                        self.selected_cell = Some(board::Point::new(x, y));
                        self.highlighted = vec![];

                        for i in 0..available_moves.len() {
                            self.highlighted.push(available_moves[i].to)
                        }
                    },
                }
            }
        }
    }

    fn render_board(&mut self, ui: &mut egui::Ui) {
        let layout = egui::Layout::centered_and_justified(egui::Direction::TopDown);
        let available_size = f32::min(ui.available_width(), ui.available_height());
        egui::Grid::new("checkers_grid")
        .spacing(egui::vec2(2., 2.))
        .min_col_width(available_size / 9. -2.)
        .max_col_width(available_size / 9. -2.)
        .min_row_height(available_size / 9. -2.)
        .show(ui, |ui| {
            self.render_coordinate(&layout, ui, format!("{}", self.bd.move_amount));

            let (start, end, step) = match self.player_side {
                board::Side::White => (0, 8, 1),
                board::Side::Black => (7, -1, -1),
            };
            let mut y = start;
            let mut x = start;

            while x != end {
                self.render_coordinate(&layout, ui, format!("{}", x));
                x += step;
            }
            ui.end_row();

            while y != end {
                x = start;
                while x != end {
                    if x == start {
                        self.render_coordinate(&layout, ui, format!("{}", y));
                    }

                    self.render_cell(ui, x as usize, y as usize);
                    x += step;
                }
                ui.end_row();
                y += step;
            }
        });
    }

    fn change_game_mode(&mut self, new_mode: GameMode) {
        self.game_mode = new_mode;
        self.restart();
    }

    fn who_win(&self) -> String {
        match self.bd.who_win() {
            Some(board::Side::Black) => "Black".to_string(),
            Some(board::Side::White) => "White".to_string(),
            None => "No one".to_string(),
        }
    }
}

impl eframe::App for App {
    #[allow(unused_variables)]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {

    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Checkers", |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url(
                            "Source code.",
                            "https://github.com/e9000000000/checkers",
                            ));
                    if ui.button("Restart").clicked() {
                        self.restart();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Game mode", |ui| {
                    if ui.radio(self.game_mode == GameMode::SelfPlay, "self play").clicked() {
                        self.change_game_mode(GameMode::SelfPlay)
                    }
                    if ui.radio(self.game_mode == GameMode::Random, "random").clicked() {
                        self.change_game_mode(GameMode::Random)
                    }
                    if ui.radio(self.game_mode == GameMode::MinMax5, "min max 5").clicked() {
                        self.change_game_mode(GameMode::MinMax5)
                    }
                    if ui.radio(self.game_mode == GameMode::MinMax10, "min max 10").clicked() {
                        self.change_game_mode(GameMode::MinMax10)
                    }
                    if ui.radio(self.game_mode == GameMode::MinMax15, "min max 15").clicked() {
                        self.change_game_mode(GameMode::MinMax15)
                    }
                    if ui.radio(self.game_mode == GameMode::RandomVsRandom, "random vs random").clicked() {
                        self.change_game_mode(GameMode::RandomVsRandom)
                    }
                });
                ui.menu_button("Change side", |ui| {
                    if ui.radio(self.player_side == board::Side::White, "white").clicked() {
                        self.player_side = board::Side::White;
                        self.restart();
                    }
                    if ui.radio(self.player_side == board::Side::Black, "black").clicked() {
                        self.player_side = board::Side::Black;
                        self.restart();
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_board(ui);
            self.show_game_ended_popup_if_game_ended();
            if self.show_game_ended_popup {
                egui::Window::new("End of the game").collapsible(false).anchor(egui::Align2::CENTER_CENTER, [0., 0.]).movable(true).show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                        ui.label(format!("{} won", self.who_win()));
                        if ui.button("restart").clicked() {
                            self.restart();
                        }
                    });
                });
            }
        });
    }
}
