use std::collections::HashMap;

use crate::drugs::*;
use crate::locations::*;
use crate::ui::*;
use eframe::{App, egui};

const INTEREST_RATE: f32 = 0.05;

/*
   /######  /######## /#######  /##   /##  /######  /########
  /##__  ##|__  ##__/| ##__  ##| ##  | ## /##__  ##|__  ##__/
 | ##  \__/   | ##   | ##  \ ##| ##  | ##| ##  \__/   | ##
 |  ######    | ##   | #######/| ##  | ##| ##         | ##
  \____  ##   | ##   | ##__  ##| ##  | ##| ##         | ##
  /##  \ ##   | ##   | ##  \ ##| ##  | ##| ##    ##   | ##
 |  ######/   | ##   | ##  | ##|  ######/|  ######/   | ##
  \______/    |__/   |__/  |__/ \______/  \______/    |__/
*/
#[derive(Default)]
pub struct Game {
  pub location: Location,
  pub inventory: HashMap<Drug, (u32, u32)>,
  pub cash: u32,
  pub debt: u32,
  pub repay_amt: u32,
  pub days: u32,
}

/*
  /###### /##      /## /#######  /##
 |_  ##_/| ###    /###| ##__  ##| ##
   | ##  | ####  /####| ##  \ ##| ##
   | ##  | ## ##/## ##| #######/| ##
   | ##  | ##  ###| ##| ##____/ | ##
   | ##  | ##\  # | ##| ##      | ##
  /######| ## \/  | ##| ##      | ########
 |______/|__/     |__/|__/      |________/
*/
impl Game {
  /*
                                        /### /###
                                       /##_/|_  ##
    /#######   /######  /##  /##  /## /##/    \  ##
   | ##__  ## /##__  ##| ## | ## | ##| ##      | ##
   | ##  \ ##| ########| ## | ## | ##| ##      | ##
   | ##  | ##| ##_____/| ## | ## | ##|  ##     /##/
   | ##  | ##|  #######|  #####/####/ \  ### /###/
   |__/  |__/ \_______/ \_____/\___/   \___/|___/
  */
  pub fn new() -> Game {
    let inv = HashMap::from([
      (Drug::Weed, (0, 0)),
      (Drug::Cocaine, (0, 0)),
      (Drug::Meth, (0, 0)),
      (Drug::Heroin, (0, 0)),
      (Drug::Ecstasy, (0, 0)),
      (Drug::Lsd, (0, 0)),
      (Drug::Shrooms, (0, 0)),
    ]);
    Game {
      location: Location::default(),
      inventory: inv,
      cash: 2000,
      debt: 2000,
      repay_amt: 0,
      days: 0,
    }
  }

  /*
      /##                                            /##   /### /###
     | ##                                           | ##  /##_/|_  ##
    /######    /######  /######  /##    /## /###### | ## /##/    \  ##
   |_  ##_/   /##__  ##|____  ##|  ##  /##//##__  ##| ##| ##      | ##
     | ##    | ##  \__/ /####### \  ##/##/| ########| ##| ##      | ##
     | ## /##| ##      /##__  ##  \  ###/ | ##_____/| ##|  ##     /##/
     |  ####/| ##     |  #######   \  #/  |  #######| ## \  ### /###/
      \___/  |__/      \_______/    \_/    \_______/|__/  \___/|___/
  */
  pub fn travel(&mut self, location: Location) {
    if self.location == location {
      return;
    }
    self.location = location;
    self.days += 1;
    self.debt += (self.debt as f32 * INTEREST_RATE) as u32;
  }

  /*
    /##                             /### /###
   | ##                            /##_/|_  ##
   | #######  /##   /## /##   /## /##/    \  ##
   | ##__  ##| ##  | ##| ##  | ##| ##      | ##
   | ##  \ ##| ##  | ##| ##  | ##| ##      | ##
   | ##  | ##| ##  | ##| ##  | ##|  ##     /##/
   | #######/|  ######/|  ####### \  ### /###/
   |_______/  \______/  \____  ##  \___/|___/
                        /##  | ##
                       |  ######/
                        \______/
  */
  pub fn buy(&mut self, drug: Drug, buy_amt: u32) {
    let price = match drug {
      Drug::Weed => 40,
      Drug::Cocaine => 50,
      Drug::Meth => 75,
      Drug::Heroin => 100,
      Drug::Ecstasy => 30,
      Drug::Lsd => 40,
      Drug::Shrooms => 40,
    };

    let entry = self.inventory.entry(drug).or_default();
    let (held_amt, _) = *entry;
    if self.cash >= price * buy_amt {
      self.cash -= price * buy_amt;
      *entry = (held_amt + buy_amt, price);
    }
  }

  /*
                        /## /##   /### /###
                       | ##| ##  /##_/|_  ##
     /#######  /###### | ##| ## /##/    \  ##
    /##_____/ /##__  ##| ##| ##| ##      | ##
   |  ###### | ########| ##| ##| ##      | ##
    \____  ##| ##_____/| ##| ##|  ##     /##/
    /#######/|  #######| ##| ## \  ### /###/
   |_______/  \_______/|__/|__/  \___/|___/
  */
  pub fn sell(&mut self, drug: Drug, sell_amt: u32) {
    let price = match drug {
      Drug::Weed => 40,
      Drug::Cocaine => 50,
      Drug::Meth => 75,
      Drug::Heroin => 100,
      Drug::Ecstasy => 30,
      Drug::Lsd => 40,
      Drug::Shrooms => 40,
    };

    let entry = self.inventory.entry(drug).or_default();
    let (held_amt, _) = *entry;
    if held_amt >= sell_amt {
      self.cash += price * sell_amt;
      if held_amt - sell_amt == 0 {
        *entry = (0, 0);
      } else {
        *entry = (held_amt - sell_amt, price);
      }
    }
  }

  /*
                                               /##           /##         /##       /### /###
                                              | ##          | ##        | ##      /##_/|_  ##
     /######   /######  /##   /##         /#######  /###### | #######  /######   /##/    \  ##
    /##__  ## |____  ##| ##  | ##        /##__  ## /##__  ##| ##__  ##|_  ##_/  | ##      | ##
   | ##  \ ##  /#######| ##  | ##       | ##  | ##| ########| ##  \ ##  | ##    | ##      | ##
   | ##  | ## /##__  ##| ##  | ##       | ##  | ##| ##_____/| ##  | ##  | ## /##|  ##     /##/
   | #######/|  #######|  #######       |  #######|  #######| #######/  |  ####/ \  ### /###/
   | ##____/  \_______/ \____  ## /######\_______/ \_______/|_______/    \___/    \___/|___/
   | ##                 /##  | ##|______/
   | ##                |  ######/
   |__/                 \______/
  */
  pub fn pay_debt(&mut self, amount: u32) {
    let amount = std::cmp::min(amount, self.debt);
    if self.cash >= amount {
      self.cash -= amount;
      self.debt -= amount;
    }
  }
}

/*
  /###### /##      /## /#######  /##              /######  /#######  /#######
 |_  ##_/| ###    /###| ##__  ##| ##             /##__  ##| ##__  ##| ##__  ##
   | ##  | ####  /####| ##  \ ##| ##            | ##  \ ##| ##  \ ##| ##  \ ##
   | ##  | ## ##/## ##| #######/| ##            | ########| #######/| #######/
   | ##  | ##  ###| ##| ##____/ | ##            | ##__  ##| ##____/ | ##____/
   | ##  | ##\  # | ##| ##      | ##            | ##  | ##| ##      | ##
  /######| ## \/  | ##| ##      | ########      | ##  | ##| ##      | ##
 |______/|__/     |__/|__/      |________/      |__/  |__/|__/      |__/
*/
impl App for Game {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
        |ui| {
          main_panel(self, ctx);
          ui.vertical(|_ui| {
            top_right_panel(self, ctx);
            bottom_right_panel(self, ctx);
          });
        },
      )
    });
  }
}
