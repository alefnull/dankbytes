use std::collections::HashMap;

use rand::Rng;

use crate::drugs::{self, Drug, get_rand_drug};

const EVENT_CHANCE: f32 = 0.1;
const BUSTED_DRUGS_MIN: usize = 1;
const BUSTED_DRUGS_MAX: usize = 3;
const MUGGING_DRUGS_MIN: usize = 1;
const MUGGING_DRUGS_MAX: usize = 5;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EventType {
  #[default]
  DrugBust,
  DrugShipment,
  Mugging,
  Count, // #types-1
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Event {
  pub e_type: EventType,
  pub e_msg: String,
  pub e_drugs: Vec<Drug>,
}

impl Event {
  pub fn drug_bust(prices: &mut [u32; 7]) -> Self {
    let mut rng = rand::rng();
    let busted_amt = rng.random_range(BUSTED_DRUGS_MIN..=BUSTED_DRUGS_MAX);
    let mut busted_drugs = Vec::new();
    for _ in 0..busted_amt {
      let drug = get_rand_drug();
      let entry = prices.get_mut(drug as usize).unwrap();
      *entry = (*entry + (*entry / 2)) as u32; // Increase price by 50%
      busted_drugs.push(drug);
    }

    Self {
      e_type: EventType::DrugBust,
      e_msg: "Cops made a huge bust! Prices have skyrocketed!".to_string(),
      e_drugs: busted_drugs,
    }
  }

  pub fn drug_shipment(prices: &mut [u32; 7]) -> Self {
    let drug = get_rand_drug();
    let entry = prices.get_mut(drug as usize).unwrap();
    *entry = (*entry - (*entry / 2)) as u32; // Decrease price by 50%

    Self {
      e_type: EventType::DrugShipment,
      e_msg: format!(
        "A huge shipment just came in! {} prices have bottomed out!",
        drug
      ),
      e_drugs: vec![drug],
    }
  }

  pub fn mugging(inv: &mut HashMap<drugs::Drug, (u32, u32)>) -> Self {
    let mut rng = rand::rng();
    let num_drugs = rng.random_range(MUGGING_DRUGS_MIN..=MUGGING_DRUGS_MAX);
    let mut mugged_drugs = Vec::new();
    let mut mugged_map = HashMap::new();
    for _ in 0..num_drugs {
      let drug = get_rand_drug();
      let entry = inv.get(&drug);
      if let Some((amt, _)) = entry {
        if *amt > 0 {
          let mugged_amt = rng.random_range(1..=*amt);
          mugged_map.insert(drug, mugged_amt);
          let new_amt = amt - mugged_amt;
          if new_amt == 0 {
            inv.remove(&drug);
          } else {
            inv.insert(drug, (new_amt, entry.unwrap().1));
          }
        }
      }
    }
    if mugged_map.is_empty() {
      return Self::default();
    }
    let mut list = String::new();
    for (drug, count) in mugged_map.iter() {
      list.push_str(&format!("{} {},", count, drug));
      mugged_drugs.push(*drug);
    }
    list.pop(); // remove last comma
    Self {
      e_type: EventType::Mugging,
      e_msg: format!("You were mugged! They took {}!", list),
      e_drugs: mugged_drugs,
    }
  }
}

pub fn generate_event(game: &mut crate::game::Game) -> Option<Event> {
  let rand_num = rand::random::<f32>();
  if rand_num < EVENT_CHANCE {
    let event_type = rand::rng().random_range(0..EventType::Count as usize);
    let event = match event_type {
      0 => Event::drug_bust(&mut game.prices),
      1 => Event::drug_shipment(&mut game.prices),
      2 => Event::mugging(&mut game.inventory),
      _ => Event::default(),
    };
    Some(event)
  } else {
    None
  }
}
