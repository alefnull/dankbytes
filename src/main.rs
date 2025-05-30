#![windows_subsystem = "windows"]
use eframe::egui::{self, ViewportBuilder};
use eframe::{Error, NativeOptions, Result};
use hello_egui::material_icons;

mod drugs;
mod events;
mod game;
mod inventory;
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
        .with_resizable(false)
        .with_maximize_button(false),
      centered: true,
      ..Default::default()
    },
    Box::new(|cc| {
      material_icons::initialize(&cc.egui_ctx);
      cc.egui_ctx.set_pixels_per_point(1.4);
      Ok(Box::new(game))
    }),
  )?;

  Ok(())
}
