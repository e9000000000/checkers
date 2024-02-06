mod board;
mod player_random;
mod player_minmax;


struct App {

}


impl App {
    fn new() -> Self {
        return App {}
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("ахуеть ано работает");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            ui.add(egui::Hyperlink::from_label_and_url(
                "Source code.",
                "https://github.com/e9000000000/checkers",
            ));

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                if ui.button("huy").clicked() {
                    println!("huy pressed");
                }
                if ui.button("huy2").clicked() {
                    println!("huy2 pressed");
                }
            });

            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                egui::Grid::new("checkers_grid").min_row_height(50.).min_col_width(50.).show(ui, |ui| {
                    ui.label("xy");
                    for x in 0..8 {
                        ui.label(format!("{}", x));
                    }
                    ui.end_row();

                    for y in 0..8 {
                        for x in 0..8 {
                            if x == 0 {
                                ui.label(format!("{}", y));
                            }

                            // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                if ui.button(format!("{},{}", x, y)).clicked() {
                                    println!("hufewwefifei");
                                }
                            // });
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/chimp.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
