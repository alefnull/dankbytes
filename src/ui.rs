use eframe::egui::{self, Align, Layout};
use egui_extras::Column;

use crate::drugs::Drug;
use crate::game::Game;
use crate::locations::Location;

/*
  /##            /######   /##
 | ##           /##__  ## | ##
 | ##  /###### | ##  \__//######
 | ## /##__  ##| ####   |_  ##_/
 | ##| ########| ##_/     | ##
 | ##| ##_____/| ##       | ## /##
 | ##|  #######| ##       |  ####/
 |__/ \_______/|__/        \___/
*/
pub fn main_panel(game: &mut Game, ctx: &egui::Context) {
  egui::SidePanel::left("left_panel")
    .exact_width(ctx.screen_rect().width() / 2.0)
    .resizable(false)
    .show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
          ui.label(format!("Location: {}", game.location));
        });
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
          ui.label(format!("Days: {}", game.days));
        });
      });

      ui.separator();

      ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
          ui.label(format!("Cash: ${}", game.cash));
        });
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
          ui.label(format!("Debt: ${}", game.debt));
        });
      });

      ui.separator();

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
            ui.label("Buy Price");
          });
        })
        .body(|mut body| {
          for drug in [
            Drug::Weed,
            Drug::Cocaine,
            Drug::Meth,
            Drug::Heroin,
            Drug::Ecstasy,
            Drug::Lsd,
            Drug::Shrooms,
          ] {
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
    });
}

/*
    /##                                         /##           /##         /##
   | ##                                        |__/          | ##        | ##
  /######    /######   /######         /######  /##  /###### | #######  /######
 |_  ##_/   /##__  ## /##__  ##       /##__  ##| ## /##__  ##| ##__  ##|_  ##_/
   | ##    | ##  \ ##| ##  \ ##      | ##  \__/| ##| ##  \ ##| ##  \ ##  | ##
   | ## /##| ##  | ##| ##  | ##      | ##      | ##| ##  | ##| ##  | ##  | ## /##
   |  ####/|  ######/| #######/      | ##      | ##|  #######| ##  | ##  |  ####/
    \___/   \______/ | ##____/       |__/      |__/ \____  ##|__/  |__/   \___/
                     | ##                           /##  \ ##
                     | ##                          |  ######/
                     |__/                           \______/
*/
pub fn top_right_panel(game: &mut Game, ctx: &egui::Context) {
  egui::TopBottomPanel::top("top_right_panel")
    .exact_height(ctx.screen_rect().height() / 2.0)
    .show(ctx, |ui| {
      ui.with_layout(
        egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true),
        |ui| {
          for loc in [
            Location::Fairfield,
            Location::Oakwood,
            Location::Lakeview,
            Location::Highland,
            Location::Edgewater,
            Location::Centerville,
          ] {
            if ui.button(loc.to_string()).clicked() {
              game.travel(loc);
            }
          }
        },
      )
    });
}

/*
  /##                   /##                     /##           /##         /##
 | ##                  | ##                    |__/          | ##        | ##
 | #######   /######  /######          /######  /##  /###### | #######  /######
 | ##__  ## /##__  ##|_  ##_/         /##__  ##| ## /##__  ##| ##__  ##|_  ##_/
 | ##  \ ##| ##  \ ##  | ##          | ##  \__/| ##| ##  \ ##| ##  \ ##  | ##
 | ##  | ##| ##  | ##  | ## /##      | ##      | ##| ##  | ##| ##  | ##  | ## /##
 | #######/|  ######/  |  ####/      | ##      | ##|  #######| ##  | ##  |  ####/
 |_______/  \______/    \___/        |__/      |__/ \____  ##|__/  |__/   \___/
                                                    /##  \ ##
                                                   |  ######/
                                                    \______/
*/
pub fn bottom_right_panel(game: &mut Game, ctx: &egui::Context) {
  egui::CentralPanel::default().show(ctx, |ui| {
    ui.with_layout(
      egui::Layout::top_down(egui::Align::LEFT).with_main_wrap(true),
      |ui| {
        ui.horizontal(|ui| {
          ui.label("Repay Debt: ");
          ui.add(egui::Slider::new(&mut game.repay_amt, 0..=game.debt));
          if ui.button("Pay").clicked() {
            game.pay_debt(game.repay_amt);
            game.repay_amt = 0;
          }
        });
        for drug in [
          Drug::Weed,
          Drug::Cocaine,
          Drug::Meth,
          Drug::Heroin,
          Drug::Ecstasy,
          Drug::Lsd,
          Drug::Shrooms,
        ] {
          ui.horizontal(|ui| {
            ui.label(format!("{}: ", drug));
            if ui.button("Buy").clicked() {
              game.buy(drug, 1);
            }
            if ui.button("Sell").clicked() {
              game.sell(drug, 1);
            }
          });
        }
      },
    )
  });
}
