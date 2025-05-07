use std::collections::HashMap;

use crate::drugs::*;
use crate::locations::*;
use crate::ui::*;
use eframe::{App, egui};

const INTEREST_RATE: f32 = 0.01;

// MARK: - Game struct
#[derive(Default)]
pub struct Game {
  pub location: Location,
  pub inventory: HashMap<Drug, (u32, u32)>,
  pub prices: [u32; 7],
  pub trade_amts: [u32; 7],
  pub cash: u32,
  pub debt: u32,
  pub repay_amt: u32,
  pub days: u32,
}

// MARK: App trait impl
impl App for Game {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
        |_| {
          main_panel(self, ctx);
          // ui.vertical(|_ui| {
          right_panel(self, ctx);
          // bottom_right_panel(self, ctx);
          // });
        },
      )
    });
  }
}

// MARK: - Game impl
impl Game {
  // MARK: Game::new()
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
      prices: rand_prices(),
      trade_amts: [0; 7],
      cash: 2000,
      debt: 2000,
      repay_amt: 0,
      days: 0,
    }
  }

  // MARK: Game::travel()
  pub fn travel(&mut self, location: Location) {
    if self.location == location {
      return;
    }
    self.location = location;
    self.prices = rand_prices();
    self.days += 1;
    self.debt += (self.debt as f32 * INTEREST_RATE) as u32;
  }

  // MARK: Game::buy()
  pub fn buy(&mut self, drug: Drug, buy_amt: u32) {
    let price = match drug {
      Drug::Weed => self.prices[Drug::Weed as usize],
      Drug::Cocaine => self.prices[Drug::Cocaine as usize],
      Drug::Meth => self.prices[Drug::Meth as usize],
      Drug::Heroin => self.prices[Drug::Heroin as usize],
      Drug::Ecstasy => self.prices[Drug::Ecstasy as usize],
      Drug::Lsd => self.prices[Drug::Lsd as usize],
      Drug::Shrooms => self.prices[Drug::Shrooms as usize],
    };
    let entry = self.inventory.entry(drug).or_default();
    let (held_amt, _) = *entry;
    if buy_amt > 0 && self.cash >= price * buy_amt {
      self.cash -= price * buy_amt;
      *entry = (held_amt + buy_amt, price);
    }
  }

  // MARK: Game::sell()
  pub fn sell(&mut self, drug: Drug, sell_amt: u32) {
    let price = match drug {
      Drug::Weed => self.prices[Drug::Weed as usize],
      Drug::Cocaine => self.prices[Drug::Cocaine as usize],
      Drug::Meth => self.prices[Drug::Meth as usize],
      Drug::Heroin => self.prices[Drug::Heroin as usize],
      Drug::Ecstasy => self.prices[Drug::Ecstasy as usize],
      Drug::Lsd => self.prices[Drug::Lsd as usize],
      Drug::Shrooms => self.prices[Drug::Shrooms as usize],
    };
    let entry = self.inventory.entry(drug).or_default();
    let (held_amt, buy_price) = *entry;
    if sell_amt > 0 && held_amt >= sell_amt {
      self.cash += price * sell_amt;
      if held_amt - sell_amt == 0 {
        *entry = (0, 0);
      } else {
        *entry = (held_amt - sell_amt, buy_price);
      }
    }
  }

  // MARK: Game::repay_debt()
  pub fn pay_debt(&mut self, amount: u32) {
    let amount = std::cmp::min(amount, self.debt);
    if self.cash >= amount {
      self.cash -= amount;
      self.debt -= amount;
      self.repay_amt = 0;
    }
  }
}
