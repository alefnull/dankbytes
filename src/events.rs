use core::ops::AddAssign;
use std::collections::HashMap;

use rand::Rng;
use thousands::Separable;

use crate::drugs::{Drug, get_rand_drug, get_rand_prices};
use crate::inventory::Inventory;

const EVENT_CHANCE: f32 = 0.1;
const BUSTED_DRUGS_MIN: usize = 1;
const BUSTED_DRUGS_MAX: usize = 3;
const MUGGING_DRUGS_MIN: usize = 1;
const MUGGING_DRUGS_MAX: usize = 5;

// MARK: EventType
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EventType {
  #[default]
  DrugBust,
  DrugShipment,
  Mugging,
  Count, // #types-1
}

// MARK: - Event struct
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Event {
  pub e_type: EventType,
  pub e_msg: String,
  pub e_drugs: Vec<Drug>,
}

impl Event {
  // MARK: Event::drug_bust()
  pub fn drug_bust(prices: &mut [u32; 7]) -> Self {
    let mut rng = rand::rng();
    let busted_amt = rng.random_range(BUSTED_DRUGS_MIN..=BUSTED_DRUGS_MAX);
    let mut busted_drugs = Vec::new();

    for _ in 0..busted_amt {
      let drug = get_rand_drug();
      if let Some(price) = prices.get_mut(drug as usize) {
        *price += *price / 2; // increase price by 50%
        busted_drugs.push(drug);
      }
    }

    Self {
      e_type: EventType::DrugBust,
      e_msg: "Cops made a huge bust! Prices have skyrocketed!".to_string(),
      e_drugs: busted_drugs,
    }
  }

  // MARK: Event::drug_shipment()
  pub fn drug_shipment(prices: &mut [u32; 7]) -> Self {
    let drug = get_rand_drug();
    if let Some(price) = prices.get_mut(drug as usize) {
      *price = (*price / 2).max(1); // decrease price by 50%, but ensure it's at least 1
    }

    Self {
      e_type: EventType::DrugShipment,
      e_msg: format!(
        "A huge shipment just came in! {} prices have bottomed out!",
        drug
      ),
      e_drugs: vec![drug],
    }
  }

  // MARK: Event::mugging()
  pub fn mugging(held_inv: &mut Inventory, held_cash: &mut u32) -> Self {
    let mut rng = rand::rng();
    let mugged_map = steal_drugs(held_inv, &mut rng);
    let cash_taken = steal_cash(held_cash, &mut rng);
    let e_msg = create_mugging_message(&mugged_map, *held_cash, cash_taken);

    Self {
      e_type: EventType::Mugging,
      e_msg,
      e_drugs: mugged_map.keys().cloned().collect(),
    }
  }
}

// MARK: - steal_drugs() [helper]
fn steal_drugs(held_inv: &mut Inventory, rng: &mut impl Rng) -> HashMap<Drug, u32> {
  let mut mugged_map = HashMap::new();

  if held_inv.has_items() {
    (0..rng.random_range(MUGGING_DRUGS_MIN..=MUGGING_DRUGS_MAX)).for_each(|_| {
      let drug = get_rand_drug(); // Get random drug once
      if let Some(held_amt) = held_inv.get_amount(drug).filter(|&amt| amt > 0) {
        let mugged_amt = rng.random_range(1..=held_amt);
        mugged_map.entry(drug).or_insert(0).add_assign(mugged_amt);
        held_inv.remove(drug, mugged_amt).unwrap_or_default();
      }
    });
  }

  mugged_map
}

// MARK: - steal_cash() [helper]
fn steal_cash(held_cash: &mut u32, rng: &mut impl Rng) -> u32 {
  if rng.random::<f32>() < 0.5 && *held_cash > 1 {
    let max_cash_taken = ((*held_cash - 1) / 2).max(1);
    let cash_taken = rng.random_range(1..=max_cash_taken);
    *held_cash = held_cash.saturating_sub(cash_taken);
    cash_taken
  } else {
    0
  }
}

// MARK: - create_mugging_message() [helper]
fn create_mugging_message(
  mugged_map: &HashMap<Drug, u32>,
  cash_before: u32,
  cash_taken: u32,
) -> String {
  match (mugged_map.is_empty(), cash_before, cash_taken) {
    (true, _, 0) => "You were mugged, but they found nothing to take!".to_string(),
    (true, cash_before, cash_taken) => {
      if cash_taken > 0 && cash_before - cash_taken <= 1 {
        "You were mugged, but they felt bad and left you with a dollar!".to_string()
      } else {
        format!(
          "You were mugged! They took ${}!",
          cash_taken.separate_with_commas()
        )
      }
    }
    (false, 0, 0) => {
      let list = mugged_map
        .iter()
        .map(|(drug, count)| format!("{} {}", count.separate_with_commas(), drug))
        .collect::<Vec<_>>()
        .join(", ");
      format!("You were mugged! They took {}!", list)
    }
    (false, _, cash_taken) => {
      let list = mugged_map
        .iter()
        .map(|(drug, count)| format!("{} {}", count.separate_with_commas(), drug))
        .collect::<Vec<_>>()
        .join(", ");
      format!(
        "You were mugged! They took {} and ${}!",
        list,
        cash_taken.separate_with_commas()
      )
    }
  }
}

// MARK: generate_event()
pub fn generate_event(game: &mut crate::game::Game) -> Option<Event> {
  let rand_num = rand::random::<f32>();
  if rand_num < EVENT_CHANCE {
    let event_type = rand::rng().random_range(0..EventType::Count as usize);
    let event = match event_type {
      0 => Event::drug_bust(&mut game.prices),
      1 => Event::drug_shipment(&mut game.prices),
      2 => {
        let event = Event::mugging(&mut game.inventory, &mut game.cash);
        game.prices = get_rand_prices();
        event
      }
      _ => Event::default(),
    };
    Some(event)
  } else {
    None
  }
}
