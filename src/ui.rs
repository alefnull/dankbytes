use eframe::egui::text::LayoutJob;
use eframe::egui::{self, Align, Button, Color32, FontId, Layout, TextFormat, Widget};
use egui_extras::Column;
use hello_egui::material_icons::icons;
use thousands::Separable;

use crate::game::GameLength;
use crate::{drugs::get_drug_list, events, game::Game, locations::Location};

// MARK: - render_window()
pub fn render_window(game: &mut Game, ctx: &egui::Context) {
  // Add dev mode toggle hotkey
  if ctx.input(|i| i.key_pressed(egui::Key::F12)) {
    game.toggle_dev_mode();
  }

  if game.dev_mode {
    render_dev_window(game, ctx);
  }

  let mut init = game.init;
  egui::CentralPanel::default().show(ctx, |ui| {
    if init {
      // MARK: game init window
      egui::Window::new("Game Init")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .max_width(200.0)
        .title_bar(false)
        .resizable(false)
        .open(&mut init)
        .show(ctx, |ui| {
          ui.vertical_centered(|ui| {
            ui.label("Game Length:");
            ui.add_space(5.0);
            ui.vertical_centered_justified(|ui| {
              ui.radio_value(&mut game.game_length, GameLength::Short, "One Month");
              ui.radio_value(&mut game.game_length, GameLength::Medium, "Six Months");
              ui.radio_value(&mut game.game_length, GameLength::Long, "One Year");
            });
            ui.add_space(5.0);
            if ui.button("Start").clicked() {
              game.days_left = match game.game_length {
                GameLength::Short => 30,
                GameLength::Medium => 180,
                GameLength::Long => 360,
              };
              game.init = false;
            }
          });
        });
      return;
    }
    // MARK: main game window
    ui.add_enabled_ui(!game.game_over, |ui| {
      ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
        |ui| {
          // MARK: bottom bar
          egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .exact_height(40.0)
            .show(ctx, |ui| {
              if game.event.is_some() {
                ui.horizontal(|ui| {
                  ui.label(game.event.as_ref().unwrap().e_msg.clone());
                });
              }
            });
          // MARK: main section
          ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
            |_| {
              main_panel(game, ctx);
              right_panel(game, ctx);
            },
          );
        },
      );
    });
    // MARK: game over section
    if game.days_left == 0 || (game.cash == 0 && !game.inventory.has_items()) {
      game.game_over = true;
      let mut game_over = game.game_over;
      let game_over_message = if game.days_left == 0 {
        "Game Over! You have run out of time."
      } else if game.cash == 0 && !game.inventory.has_items() {
        "Game Over! You are out of cash and have no items to sell."
      } else {
        "Game Over!"
      };

      egui::Window::new("Game Over")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .title_bar(false)
        .open(&mut game_over)
        .show(ctx, |ui| {
          ui.vertical_centered(|ui| {
            ui.label(game_over_message);
            ui.add_space(5.0);
            ui.label(format!("Final Cash: ${}", game.cash.separate_with_commas()));
            ui.label(format!("Final Debt: ${}", game.debt.separate_with_commas()));

            let (score, color) = if game.cash < game.debt {
              (
                format!(
                  "-{}",
                  (game.debt.saturating_sub(game.cash)).separate_with_commas()
                ),
                Color32::LIGHT_RED,
              )
            } else {
              (
                game.cash.saturating_sub(game.debt).separate_with_commas(),
                Color32::LIGHT_GREEN,
              )
            };

            let mut job = LayoutJob::default();
            job.append(
              "Final Score: ",
              0.0,
              TextFormat::simple(FontId::default(), Color32::GRAY),
            );
            job.append(
              score.as_str(),
              0.0,
              TextFormat::simple(FontId::default(), color),
            );
            ui.label(job);

            ui.add_space(5.0);

            if ui.button("OK").clicked() {
              game.reset();
            }
          });
        });
    }
  });
}

// MARK: - DEV render_dev_window()
fn render_dev_window(game: &mut Game, ctx: &egui::Context) {
  use crate::drugs::get_rand_prices;
  use eframe::egui::{ViewportBuilder, ViewportId};

  ctx.show_viewport_immediate(
    ViewportId::from_hash_of("dev_window"),
    ViewportBuilder::default()
      .with_inner_size(egui::vec2(300.0, 100.0))
      .with_always_on_top()
      // .with_close_button(false)
      .with_maximize_button(false)
      .with_minimize_button(false)
      .with_resizable(false)
      .with_taskbar(false)
      .with_visible(game.dev_mode)
      .with_title("Dev Window"),
    |ctx, _| {
      egui::CentralPanel::default().show(ctx, |ui| {
        if ctx.input(|i| i.key_pressed(egui::Key::F12)) {
          game.toggle_dev_mode();
        }
        // MARK: DEV money controls
        ui.horizontal(|ui| {
          if ui
            .add(Button::new("💵 +1k"))
            .on_hover_text("Add $1000 to cash")
            .clicked()
          {
            game.cash += 1000;
          }
          if ui
            .add(Button::new("💵 -1k"))
            .on_hover_text("Remove $1000 from cash")
            .clicked()
          {
            game.cash = game.cash.saturating_sub(1000);
          }
          ui.separator();
          // MARK: DEV drug manipulation
          let drugs = get_drug_list();
          egui::ComboBox::from_label("")
            .width(75.0)
            .selected_text(format!("📦 {}", drugs[game.selected_drug_idx]))
            .show_ui(ui, |ui| {
              for (idx, drug) in drugs.iter().enumerate() {
                ui.selectable_value(&mut game.selected_drug_idx, idx, drug.to_string());
              }
            });
          if ui
            .add(Button::new("+10"))
            .on_hover_text(format!(
              "Add 10 {} to inventory",
              drugs[game.selected_drug_idx]
            ))
            .clicked()
          {
            game.inventory.add(drugs[game.selected_drug_idx], 10, 0);
          }
          if ui
            .add(Button::new("-10"))
            .on_hover_text(format!(
              "Remove 10 {} from inventory",
              drugs[game.selected_drug_idx]
            ))
            .clicked()
          {
            game
              .inventory
              .remove(drugs[game.selected_drug_idx], 10)
              .ok();
          }
        });

        ui.separator();

        // MARK: DEV event triggers
        ui.horizontal(|ui| {
          if ui
            .add(Button::new("🚨Bust"))
            .on_hover_text("Trigger Drug Bust event")
            .clicked()
          {
            game.last_prices = game.prices;
            game.prices = get_rand_prices();
            game.event = Some(events::Event::drug_bust(&mut game.prices));
          }
          if ui
            .add(Button::new("🚢Shipment"))
            .on_hover_text("Trigger Drug Shipment event")
            .clicked()
          {
            game.last_prices = game.prices;
            game.prices = get_rand_prices();
            game.event = Some(events::Event::drug_shipment(&mut game.prices));
          }
          if ui
            .add(Button::new("🔪Mugging"))
            .on_hover_text("Trigger Mugging event")
            .clicked()
          {
            game.event = Some(events::Event::mugging(&mut game.inventory, &mut game.cash));
          }
        });
      });
      if ctx.input(|i| i.viewport().close_requested()) {
        game.dev_mode = false;
      }
    },
  );
}

// MARK: - main_panel()
pub fn main_panel(game: &mut Game, ctx: &egui::Context) {
  egui::SidePanel::left("left_panel")
    .exact_width(ctx.screen_rect().width() / 2.0)
    .resizable(false)
    .show(ctx, |ui| {
      render_stats_header(game, ui);
      ui.separator();
      ui.add_space(5.0);
      render_inventory_table(game, ui);
    });
}

// MARK: render_debt_repayment()
fn render_debt_repayment(game: &mut Game, ui: &mut egui::Ui) {
  ui.add_enabled_ui(game.debt != 0, |ui| {
    ui.horizontal(|ui| {
      let max_repay = if game.debt > 0 {
        game.debt.min(game.cash)
      } else {
        1
      };
      ui.add(
        egui::Slider::new(&mut game.repay_amt, 0..=max_repay)
          .trailing_fill(true)
          .prefix("$")
          .drag_value_speed(0.3),
      );
      if ui.button("Repay").clicked() {
        game.repay_debt(game.repay_amt);
      }
    });
  });
}

// MARK: render_stats_header()
fn render_stats_header(game: &mut Game, ui: &mut egui::Ui) {
  ui.horizontal(|ui| {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
      ui.label(format!("Location: {}", game.location));
    });
    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
      ui.label(format!("Days Left: {}", game.days_left));
    });
  });

  ui.horizontal(|ui| {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
      ui.label(format!("Cash: ${}", game.cash.separate_with_commas()));
    });
    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
      ui.label(format!("Debt: ${}", game.debt.separate_with_commas()));
    });
  });

  render_debt_repayment(game, ui);
}

// MARK: render_inventory_table()
fn render_inventory_table(game: &mut Game, ui: &mut egui::Ui) {
  egui_extras::TableBuilder::new(ui)
    .columns(Column::remainder(), 3)
    .striped(true)
    .header(12.0, |mut header| {
      header.col(|ui| {
        ui.label("Drug");
      });
      header.col(|ui| {
        ui.label("Amount");
      });
      header.col(|ui| {
        ui.label("Cost");
      });
    })
    .body(|mut body| {
      for drug in get_drug_list() {
        let amt = game.inventory.get_amount(drug).unwrap_or(0);
        let cost = game.inventory.get_cost(drug).unwrap_or(0);
        body.row(18.0, |mut row| {
          row.col(|ui| {
            ui.label(drug.to_string());
          });
          row.col(|ui| {
            ui.label(amt.to_string());
          });
          row.col(|ui| {
            ui.label(cost.to_string());
          });
        });
      }
    });
}

// MARK: - top_right_panel()
pub fn right_panel(game: &mut Game, ctx: &egui::Context) {
  egui::CentralPanel::default().show(ctx, |ui| {
    // MARK: travel section
    egui_extras::TableBuilder::new(ui)
      .columns(Column::remainder(), 3)
      .body(|mut body| {
        for t_row in 0..2 {
          body.row(16.0, |mut row| {
            for t_cell in 0..3 {
              let loc = match t_row * 3 + t_cell {
                1 => Location::Oakwood,
                2 => Location::Lakeview,
                3 => Location::Highland,
                4 => Location::Edgewater,
                5 => Location::Centerville,
                _ => Location::Fairfield, // 0 or > 5
              };
              row.col(|ui| {
                if ui
                  .add_enabled(
                    game.location != loc,
                    Button::new(loc.to_string()).min_size(egui::vec2(90.0, 22.0)),
                  )
                  .on_disabled_hover_text("You are already here.")
                  .clicked()
                {
                  game.travel(loc);
                }
              });
            }
          });
        }
      });
    ui.separator();
    // MARK: trading section
    ui.with_layout(
      egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
      |ui| {
        ui.add_space(5.0);
        render_drug_trading_table(game, ui);
      },
    );
  });
}

// MARK: render_drug_trading_table()
fn render_drug_trading_table(game: &mut Game, ui: &mut egui::Ui) {
  egui_extras::TableBuilder::new(ui)
    .striped(true)
    .columns(Column::auto(), 4)
    .header(12.0, |mut header| {
      header.col(|ui| {
        ui.label("Drug");
      });
      header.col(|ui| {
        ui.label("Price");
      });
      header.col(|ui| {
        ui.label("Buy");
      });
      header.col(|ui| {
        ui.label("Sell");
      });
    })
    .body(|mut body| {
      for drug in get_drug_list() {
        body.row(18.0, |mut row| {
          let col = if game.event.is_some() {
            match game.event.as_ref().unwrap().e_type {
              events::EventType::DrugBust => Color32::LIGHT_RED,
              events::EventType::DrugShipment => Color32::LIGHT_GREEN,
              _ => Color32::GRAY,
            }
          } else {
            Color32::GRAY
          };
          let ico = match game.prices[drug as usize] {
            price if price > game.last_prices[drug as usize] => icons::ICON_TRENDING_UP,
            price if price < game.last_prices[drug as usize] => icons::ICON_TRENDING_DOWN,
            _ => icons::ICON_TRENDING_FLAT,
          };
          // MARK: drug name
          row.col(|ui| {
            ui.label(drug.to_string());
          });
          // MARK: drug price
          row.col(|ui| {
            ui.horizontal(|ui| {
              if game.event.is_some() && game.event.as_ref().unwrap().e_drugs.contains(&drug) {
                ui.visuals_mut().override_text_color = Some(col);
              }
              ui.label(ico);
              ui.label(format!(" ${}", game.prices[drug as usize]));
              ui.reset_style();
            });
          });
          // MARK: buy section
          row.col(|ui| {
            ui.horizontal(|ui| {
              let max_buy = game.cash / game.prices[drug as usize];
              egui::DragValue::new(&mut game.buy_amts[drug as usize])
                .range(0..=max_buy)
                // .speed(0.1)
                .ui(ui);
              if ui.button("Buy").clicked()
                && game.cash >= game.prices[drug as usize] * game.buy_amts[drug as usize]
              {
                game.buy(drug, game.buy_amts[drug as usize]);
                game.buy_amts[drug as usize] = 0;
              }
            });
          });
          // MARK: sell section
          row.col(|ui| {
            ui.horizontal(|ui| {
              let total_inv_amt = game.inventory.get_amount(drug).unwrap_or(0);
              egui::DragValue::new(&mut game.sell_amts[drug as usize])
                .range(0..=total_inv_amt)
                // .speed(0.1)
                .ui(ui);
              if ui.button("Sell").clicked() {
                let amt = game.inventory.get_amount(drug).unwrap_or(0);
                if amt >= game.sell_amts[drug as usize] {
                  game.sell(drug, game.sell_amts[drug as usize]);
                  game.sell_amts[drug as usize] = 0;
                }
              }
            });
          });
        });
      }
    });
}
