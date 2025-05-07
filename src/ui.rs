use std::collections::HashMap;

use eframe::egui::{self, Align, Button, Layout, Widget};
use egui_extras::Column;
use hello_egui::flex::{Flex, item};
// use hello_egui::material_icons::icons;

use crate::drugs::{Drug, get_drug_list, rand_prices};
use crate::game::{Game, GameLength};
use crate::locations::Location;

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
  if game.debt == 0 {
    ui.add_enabled_ui(false, |ui| {
      Flex::horizontal().show(ui, |flex| {
        flex.add(
          item(),
          egui::Slider::new(&mut game.repay_amt, 0..=100)
            .trailing_fill(true)
            .prefix("$")
            .drag_value_speed(0.3),
        );
        flex.add(item(), Button::new("Repay"));
      });
    });
  } else {
    Flex::horizontal().show(ui, |flex| {
      flex.add(
        item(),
        egui::Slider::new(&mut game.repay_amt, 0..=game.debt)
          .trailing_fill(true)
          .prefix("$")
          .drag_value_speed(0.3),
      );
      if flex.add(item(), Button::new("Repay")).clicked() {
        game.pay_debt(game.repay_amt);
      }
    });
  }
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
            .add(item().grow(1.0), Button::new(loc.to_string()))
            .clicked()
          {
            game.travel(loc);
          }
        }
      });
      ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
        |ui| {
          render_drug_trading_table(game, ui);
        },
      );
      if game.days + 1 >= game.game_length as u32 {
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
              // self.init = true;
              game.location = Location::default();
              game.inventory = HashMap::from([
                (Drug::Weed, (0, 0)),
                (Drug::Cocaine, (0, 0)),
                (Drug::Meth, (0, 0)),
                (Drug::Heroin, (0, 0)),
                (Drug::Ecstasy, (0, 0)),
                (Drug::Lsd, (0, 0)),
                (Drug::Shrooms, (0, 0)),
              ]);
              game.prices = rand_prices();
              game.trade_amts = [0; 7];
              game.cash = 2000;
              game.debt = 2000;
              game.repay_amt = 0;
              game.game_length = GameLength::Short;
              game.days = 0;
              game.game_over = false;
            }
          });
        // return;
      }
    });
}

// MARK: - bottom_right_panel()
// pub fn bottom_right_panel(game: &mut Game, ctx: &egui::Context) {
//   egui::CentralPanel::default().show(ctx, |ui| {
//     ui.with_layout(
//       egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
//       |ui| {
//         render_drug_trading_table(game, ui);
//       },
//     )
//   });
// }

// MARK: render_drug_trading_row()
fn render_drug_trading_row(game: &mut Game, drug: Drug, row: &mut egui_extras::TableRow) {
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
      // if ui.button(icons::ICON_ADD).clicked()
      if ui.button("Buy").clicked()
        && game.cash >= game.prices[drug as usize] * game.trade_amts[drug as usize]
      {
        game.buy(drug, game.trade_amts[drug as usize]);
        // game.trade_amts[drug as usize] = 0;
      }

      egui::DragValue::new(&mut game.trade_amts[drug as usize])
        .range(0..=100)
        .speed(0.1)
        .ui(ui);

      // if ui.button(icons::ICON_REMOVE).clicked() {
      if ui.button("Sell").clicked() {
        let entry = game.inventory.entry(drug).or_default();
        let (amt, _) = *entry;
        if amt >= game.trade_amts[drug as usize] {
          game.sell(drug, game.trade_amts[drug as usize]);
          // game.trade_amts[drug as usize] = 0;
        }
      }
    });
  });
}

// MARK: render_drug_trading_table()
fn render_drug_trading_table(game: &mut Game, ui: &mut egui::Ui) {
  egui_extras::TableBuilder::new(ui)
    .columns(Column::auto(), 3)
    .body(|mut body| {
      for drug in get_drug_list() {
        body.row(14.0, |mut row| {
          render_drug_trading_row(game, drug, &mut row);
        });
      }
    });
}
