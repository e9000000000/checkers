mod board;
mod player_random;
mod player_minmax;
mod gui;


fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 320.0])
            .with_min_inner_size([300.0, 320.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/black_king.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "checkers",
        native_options,
        Box::new(|_cc| Box::new(gui::App::new())),
    )
}
