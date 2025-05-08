use crate::drugs::{Drug, get_rand_drug};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
  PoliceBust,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
  pub e_type: EventType,
  pub e_msg: String,
  pub e_drug: Drug,
}

impl Event {
  pub fn police_bust(prices: &mut [u32; 7]) -> Self {
    let drug = get_rand_drug();
    let entry = prices.get_mut(drug as usize).unwrap();
    *entry = (*entry - 20) as u32; // Reduce price by 50%

    Self {
      e_type: EventType::PoliceBust,
      e_msg: format!("Cops made a huge bust! {} prices have bottomed out!", drug),
      e_drug: drug,
    }
  }
}
