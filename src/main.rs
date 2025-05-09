#![windows_subsystem = "windows"]
use eframe::egui::{self, ViewportBuilder};
use eframe::{Error, NativeOptions, Result};

mod drugs;
mod events;
mod game;
mod locations;
mod ui;

const LEAF_PNG: &[u8] = include_bytes!("../assets/leaf.png");

fn main() -> Result<(), Error> {
  let game = game::Game::new();
  let ico = eframe::icon_data::from_png_bytes(LEAF_PNG).expect("Failed to load icon");

  eframe::run_native(
    "Dank Bytes",
    NativeOptions {
      viewport: ViewportBuilder::default()
        .with_inner_size(egui::vec2(840.0, 420.0))
        .with_icon(ico)
        .with_resizable(false),
      centered: true,
      ..Default::default()
    },
    Box::new(|cc| {
      cc.egui_ctx.set_pixels_per_point(1.4);
      Ok(Box::new(game))
    }),
  )?;

  Ok(())
}
