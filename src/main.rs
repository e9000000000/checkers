mod board;
mod player_random;
mod player_minmax;


struct App {
    bd: board::Board,
    highlighted: Vec<board::Point>,
    selected_cell: Option<board::Point>,
}


impl App {
    fn new() -> Self {
        return Self {
            bd: board::Board::new(),
            highlighted: vec![],
            selected_cell: None,
        }
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
                if ui.button(format!("{}", self.bd.get_cell(x, y))).clicked() {
                    self.on_click(x, y);
                }
            });
        });
    }

    fn on_click(&mut self, x: usize, y: usize) {
        match self.bd.get_cell(x, y) {
            board::Cell::Empty => {
                for i in 0..self.highlighted.len() {
                    if self.highlighted[i] == board::Point::new(x, y) {
                        match self.selected_cell {
                            Some(cell) => self.bd.do_move(board::Move::new(cell.x, cell.y, x, y)).unwrap(),
                            None => (),
                        }
                        return
                    }
                }
            },
            _ => {
                self.selected_cell = Some(board::Point::new(x, y));
                let available_moves = self.bd.available_moves_for_cell(x, y);
                self.highlighted = vec![];
                for i in 0..available_moves.len() {
                    self.highlighted.push(available_moves[i].to)
                }
            }
        }
    }

    fn render_board(&mut self, ui: &mut egui::Ui) {
        let layout = egui::Layout::centered_and_justified(egui::Direction::TopDown);
        egui::Grid::new("checkers_grid")
        .spacing(egui::vec2(2., 2.))
        .min_col_width(ui.available_width() / 9. -2.)
        .max_col_width(ui.available_width() / 9. -2.)
        .min_row_height(ui.available_height() / 9. -2.)
        .show(ui, |ui| {
            self.render_coordinate(&layout, ui, "xy".to_string());
            for x in 0..8 {
                self.render_coordinate(&layout, ui, format!("{}", x));
            }
            ui.end_row();

            for y in 0..8 {
                for x in 0..8 {
                    if x == 0 {
                        self.render_coordinate(&layout, ui, format!("{}", y));
                    }

                    self.render_cell(ui, x, y);
                }
                ui.end_row();
            }
        });
    }
}

impl eframe::App for App {
    #[allow(unused_variables)]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {

    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Checkers", |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url(
                            "Source code.",
                            "https://github.com/e9000000000/checkers",
                            ));
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_board(ui);
        });
    }
}



fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 320.0])
            .with_min_inner_size([320.0, 320.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/chimp.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "checkers",
        native_options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
