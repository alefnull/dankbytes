use eframe::egui::ViewportBuilder;
use eframe::{Error, NativeOptions, Result, egui};

mod drugs;
mod game;
mod locations;
mod ui;

fn main() -> Result<(), Error> {
  let game = game::Game::new();

  eframe::run_native(
    "Dank Bytes",
    NativeOptions {
      viewport: ViewportBuilder::default()
        .with_inner_size(egui::vec2(800.0, 600.0))
        .with_resizable(false),
      centered: false,
      ..Default::default()
    },
    Box::new(|cc| {
      cc.egui_ctx.set_pixels_per_point(1.25);
      Ok(Box::new(game))
    }),
  )?;

  Ok(())
}
