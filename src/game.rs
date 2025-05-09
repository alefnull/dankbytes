use std::collections::HashMap;

use crate::drugs::*;
use crate::events::{Event, generate_event};
use crate::locations::*;
use crate::ui::*;
use eframe::{App, egui};

const INTEREST_RATE: f32 = 0.015;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
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
  pub inventory: HashMap<Drug, (u32, u32)>,
  pub prices: [u32; 7],
  pub buy_amts: [u32; 7],
  pub sell_amts: [u32; 7],
  pub cash: u32,
  pub debt: u32,
  pub repay_amt: u32,
  pub event: Option<Event>,
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
      init: true,
      game_over: false,
      location: Location::default(),
      inventory: inv,
      prices: get_rand_prices(),
      buy_amts: [0; 7],
      sell_amts: [0; 7],
      cash: 2000,
      debt: 2000,
      repay_amt: 0,
      game_length: GameLength::Short,
      days_left: GameLength::Short as u32,
      event: None,
    }
  }

  // MARK: Game::travel()
  pub fn travel(&mut self, location: Location) {
    if self.location == location {
      return;
    }
    self.days_left = self.days_left.saturating_sub(1);
    self.location = location;
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
    let entry = self.inventory.entry(drug).or_default();
    let (held_amt, _) = *entry;
    if buy_amt > 0 && self.cash >= price * buy_amt {
      self.cash -= price * buy_amt;
      *entry = (held_amt + buy_amt, price);
    }
  }

  // MARK: Game::sell()
  pub fn sell(&mut self, drug: Drug, sell_amt: u32) {
    let price = get_drug_price(drug, &self.prices);
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
  pub fn repay_debt(&mut self, amount: u32) {
    let amount = std::cmp::min(amount, self.debt);
    if self.cash >= amount {
      self.cash -= amount;
      self.debt -= amount;
      self.repay_amt = 0;
    }
  }
}
