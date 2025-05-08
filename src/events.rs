use rand::Rng;

use crate::drugs::{Drug, get_rand_drug};

const EVENT_CHANCE: f32 = 0.1;
const BUSTED_DRUGS_MIN: usize = 1;
const BUSTED_DRUGS_MAX: usize = 3;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EventType {
  #[default]
  DrugBust,
  DrugShipment,
  Count,
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
}

pub fn generate_event(prices: &mut [u32; 7]) -> Option<Event> {
  let rand_num = rand::random::<f32>();
  if rand_num < EVENT_CHANCE {
    let event_type = rand::rng().random_range(0..EventType::Count as usize);
    let event = match event_type {
      0 => Event::drug_bust(prices),
      1 => Event::drug_shipment(prices),
      _ => Event::default(),
    };
    Some(event)
  } else {
    None
  }
}
