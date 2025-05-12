use crate::drugs::Drug;
use std::collections::HashMap;
use std::fmt;

// MARK: Inventory Errors
#[derive(Debug, PartialEq)]
pub enum InventoryError {
  InsufficientAmount,
  DrugNotFound,
}

// MARK: - Inventory struct
#[derive(Debug, Clone)]
pub struct Inventory {
  items: HashMap<Drug, (Amount, Cost)>,
}

// MARK: default() impl
impl Default for Inventory {
  fn default() -> Self {
    Self::new()
  }
}

impl Inventory {
  /// MARK: new()
  pub fn new() -> Self {
    Self {
      items: HashMap::new(),
    }
  }

  // MARK: add()
  pub fn add(&mut self, drug: Drug, amount: u32, cost: u32) {
    self
      .items
      .entry(drug)
      .and_modify(|(a, c)| {
        a.add(amount);
        c.0 = cost;
      })
      .or_insert((Amount::new(amount), Cost::new(cost)));
  }

  // MARK: remove()
  pub fn remove(&mut self, drug: Drug, amount: u32) -> Result<(), InventoryError> {
    match self.items.get_mut(&drug) {
      Some((amt, _)) if amt.get() >= amount => {
        amt.sub(amount);
        Ok(())
      }
      Some(_) => Err(InventoryError::InsufficientAmount),
      None => Err(InventoryError::DrugNotFound),
    }
  }

  // MARK: get_amount()
  pub fn get_amount(&self, drug: Drug) -> Option<u32> {
    self.items.get(&drug).map(|(amt, _)| amt.get())
  }

  // MARK: get_cost()
  pub fn get_cost(&self, drug: Drug) -> Option<u32> {
    self.items.get(&drug).map(|(_, cost)| cost.get())
  }

  // MARK: reset_cost()
  pub fn reset_cost(&mut self, drug: Drug) {
    if let Some((_, cost)) = self.items.get_mut(&drug) {
      cost.0 = 0;
    }
  }

  pub fn has_items(&self) -> bool {
    self.items.values().any(|(amount, _)| amount.get() > 0)
  }
}

// MARK: - Amount type wrapper
#[derive(Debug, Clone, Copy)]
pub struct Amount(u32);

impl Amount {
  pub fn new(value: u32) -> Self {
    Self(value)
  }

  pub fn add(&mut self, value: u32) {
    self.0 += value;
  }

  pub fn sub(&mut self, value: u32) -> bool {
    if self.0 >= value {
      self.0 -= value;
      true
    } else {
      false
    }
  }

  pub fn get(&self) -> u32 {
    self.0
  }
}

impl fmt::Display for Amount {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

// MARK: - Cost type wrapper
#[derive(Debug, Clone, Copy)]
pub struct Cost(u32);

impl Cost {
  pub fn new(value: u32) -> Self {
    Self(value)
  }

  pub fn get(&self) -> u32 {
    self.0
  }
}

impl fmt::Display for Cost {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "${}", self.0)
  }
}
