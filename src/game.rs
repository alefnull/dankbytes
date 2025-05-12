use eframe::{App, egui};

use crate::drugs::*;
use crate::events::{Event, generate_event};
use crate::inventory::Inventory;
use crate::locations::*;
use crate::ui::*;

const INTEREST_RATE: f32 = 0.09;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum GameLength {
  #[default]
  Short = 30,
  Medium = 180,
  Long = 360,
}

// MARK: - Game struct
#[derive(Default, Clone)]
pub struct Game {
  pub init: bool,
  pub game_over: bool,
  pub game_length: GameLength,
  pub days_left: u32,
  pub location: Location,
  pub inventory: Inventory,
  pub prices: [u32; 7],
  pub last_prices: [u32; 7],
  pub buy_amts: [u32; 7],
  pub sell_amts: [u32; 7],
  pub cash: u32,
  pub debt: u32,
  pub repay_amt: u32,
  pub event: Option<Event>,
  #[cfg(debug_assertions)]
  pub dev_mode: bool,
  #[cfg(debug_assertions)]
  pub selected_drug_idx: usize,
}

// MARK: App trait impl
impl App for Game {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    render_window(self, ctx);
  }
}

// MARK: - Game impl
impl Game {
  // MARK: Game::new()
  pub fn new() -> Game {
    let rand_prices = get_rand_prices();
    Game {
      init: true,
      game_over: false,
      location: Location::default(),
      inventory: Inventory::default(),
      prices: rand_prices,
      last_prices: rand_prices,
      buy_amts: [0; 7],
      sell_amts: [0; 7],
      cash: 2000,
      debt: 2000,
      repay_amt: 0,
      game_length: GameLength::Short,
      days_left: GameLength::Short as u32,
      event: None,
      #[cfg(debug_assertions)]
      dev_mode: false,
      #[cfg(debug_assertions)]
      selected_drug_idx: 0,
    }
  }

  #[cfg(debug_assertions)]
  pub fn toggle_dev_mode(&mut self) {
    self.dev_mode = !self.dev_mode;
  }

  // MARK: Game::travel()
  pub fn travel(&mut self, location: Location) {
    if self.location == location {
      return;
    }
    self.days_left = self.days_left.saturating_sub(1);
    self.location = location;
    self.last_prices = self.prices;
    self.prices = get_rand_prices();
    self.debt += (self.debt as f32 * INTEREST_RATE) as u32;

    if let Some(event) = generate_event(self) {
      self.event = Some(event);
    } else {
      self.event = None;
    }
  }

  // MARK: Game::buy()
  pub fn buy(&mut self, drug: Drug, buy_amt: u32) {
    let price = get_drug_price(drug, &self.prices);
    if buy_amt > 0 && self.cash >= price * buy_amt {
      self.cash -= price * buy_amt;
      self.inventory.add(drug, buy_amt, price);
    }
  }

  // MARK: Game::sell()
  pub fn sell(&mut self, drug: Drug, sell_amt: u32) {
    let price = get_drug_price(drug, &self.prices);
    if sell_amt > 0 && self.inventory.get_amount(drug).unwrap_or(0) >= sell_amt {
      self.cash += price * sell_amt;
      self.inventory.remove(drug, sell_amt).unwrap();
      if self.inventory.get_amount(drug).unwrap_or(0) == 0 {
        self.inventory.reset_cost(drug);
      }
    }
  }

  // MARK: Game::repay_debt()
  pub fn repay_debt(&mut self, amount: u32) {
    let amount = std::cmp::min(amount, self.debt);
    if self.cash >= amount {
      self.cash -= amount;
      self.debt -= amount;
      self.repay_amt = 0;
    }
  }

  // MARK: Game::reset()
  pub fn reset(&mut self) {
    *self = Game::new();
  }
}
