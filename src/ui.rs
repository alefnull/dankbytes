use eframe::egui::{self, Align, Button, Layout, Widget};
use egui_extras::Column;
use hello_egui::flex::{Flex, item};
// use hello_egui::material_icons::icons;

use crate::drugs::get_drug_list;
use crate::game::{Game, GameLength};
use crate::locations::Location;

// MARK: - render_window()
pub fn render_window(game: &mut Game, ctx: &egui::Context) {
  let mut init = game.init;
  egui::CentralPanel::default().show(ctx, |ui| {
    if init {
      // MARK: game init window
      egui::Window::new("Game Init")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .title_bar(false)
        .open(&mut init)
        .show(ctx, |ui| {
          ui.horizontal(|ui| {
            ui.label("Game Length:");
            ui.radio_value(&mut game.game_length, GameLength::Short, "Short");
            ui.radio_value(&mut game.game_length, GameLength::Medium, "Medium");
            ui.radio_value(&mut game.game_length, GameLength::Long, "Long");
          });
          if ui.button("Start").clicked() {
            game.init = false;
          }
        });
      return;
    }
    // MARK: main game window
    ui.with_layout(
      egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
      |ui| {
        egui::TopBottomPanel::bottom("bottom_panel")
          .resizable(false)
          .exact_height(80.0)
          .show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("BOTTOM_BAR"));
          });
        ui.with_layout(
          egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
          |_| {
            main_panel(game, ctx);
            // ui.vertical(|_ui| {
            right_panel(game, ctx);
            // bottom_right_panel(self, ctx);
            // });
          },
        );
      },
    );
  });
}

// MARK: - main_panel()
pub fn main_panel(game: &mut Game, ctx: &egui::Context) {
  egui::SidePanel::left("left_panel")
    .exact_width(ctx.screen_rect().width() / 2.0)
    .resizable(false)
    .show(ctx, |ui| {
      render_stats_header(game, ui);
      ui.separator();
      render_inventory_table(game, ui);
    });
}

// MARK: render_debt_repayment()
fn render_debt_repayment(game: &mut Game, ui: &mut egui::Ui) {
  ui.add_enabled_ui(game.debt != 0, |ui| {
    Flex::horizontal().show(ui, |flex| {
      let max_repay = if game.debt > 0 { game.debt } else { 1 };
      flex.add(
        item(),
        egui::Slider::new(&mut game.repay_amt, 0..=max_repay)
          .trailing_fill(true)
          .prefix("$")
          .drag_value_speed(0.3),
      );
      // flex.add(item(), Button::new("Repay"));
      if flex.add(item(), Button::new("Repay")).clicked() {
        game.pay_debt(game.repay_amt);
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
      ui.label(format!("Days: {}", game.days));
    });
  });

  ui.horizontal(|ui| {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
      ui.label(format!("Cash: ${}", game.cash));
    });
    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
      ui.label(format!("Debt: ${}", game.debt));
    });
  });

  render_debt_repayment(game, ui);
}

// MARK: render_inventory_table()
fn render_inventory_table(game: &mut Game, ui: &mut egui::Ui) {
  egui_extras::TableBuilder::new(ui)
    .columns(Column::remainder(), 3)
    .striped(true)
    .header(14.0, |mut header| {
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
        let entry = game.inventory.entry(drug).or_default();
        let (amt, cost) = *entry;
        body.row(14.0, |mut row| {
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
  egui::CentralPanel::default()
    // .exact_height(ctx.screen_rect().height() / 2.0)
    .show(ctx, |ui| {
      // MARK: travel section
      Flex::horizontal().wrap(true).show(ui, |flex| {
        for loc in [
          Location::Fairfield,
          Location::Oakwood,
          Location::Lakeview,
          Location::Highland,
          Location::Edgewater,
          Location::Centerville,
        ] {
          if flex
            .add(
              item().grow(0.5),
              Button::new(loc.to_string()).min_size(egui::vec2(80.0, 22.0)),
            )
            .clicked()
          {
            game.travel(loc);
          }
        }
      });
      ui.separator();
      // MARK: trading section
      ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
        |ui| {
          render_drug_trading_table(game, ui);
        },
      );
      // MARK: game over section
      if game.days >= game.game_length as u32 {
        game.game_over = true;
        let mut game_over = game.game_over;
        egui::Window::new("Game Over")
          .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
          .resizable(false)
          .title_bar(false)
          .open(&mut game_over)
          .show(ctx, |ui| {
            ui.label("Game Over! You have run out of time.");
            if ui.button("OK").clicked() {
              *game = Game::new();
            }
          });
      }
    });
}

// MARK: render_drug_trading_table()
fn render_drug_trading_table(game: &mut Game, ui: &mut egui::Ui) {
  egui_extras::TableBuilder::new(ui)
    .columns(Column::auto(), 3)
    .body(|mut body| {
      for drug in get_drug_list() {
        body.row(14.0, |mut row| {
          row.col(|ui| {
            ui.label(drug.to_string());
          });
          row.col(|ui| {
            ui.horizontal(|ui| {
              ui.label(format!("${}", game.prices[drug as usize]));
            });
          });
          row.col(|ui| {
            ui.horizontal(|ui| {
              ui.separator();
              // MARK: buy section
              let max_buy = game.cash / game.prices[drug as usize];
              egui::DragValue::new(&mut game.buy_amts[drug as usize])
                .range(0..=max_buy)
                .speed(0.1)
                .ui(ui);
              if ui.button("Buy").clicked()
                && game.cash >= game.prices[drug as usize] * game.buy_amts[drug as usize]
              {
                game.buy(drug, game.buy_amts[drug as usize]);
              }
              ui.separator();
              // MARK: sell section
              let total_inv_amt = game.inventory.entry(drug).or_default().0;
              egui::DragValue::new(&mut game.sell_amts[drug as usize])
                .range(0..=total_inv_amt)
                .speed(0.1)
                .ui(ui);

              if ui.button("Sell").clicked() {
                let entry = game.inventory.entry(drug).or_default();
                let (amt, _) = *entry;
                if amt >= game.sell_amts[drug as usize] {
                  game.sell(drug, game.sell_amts[drug as usize]);
                }
              }
            });
          });
        });
      }
    });
}
