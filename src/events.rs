use crate::drugs::{Drug, get_rand_drug};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
  DrugBust,
  DrugShipment,
  Count,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
  pub e_type: EventType,
  pub e_msg: String,
  pub e_drug: Drug,
}

impl Event {
  pub fn drug_bust(prices: &mut [u32; 7]) -> Self {
    let drug = get_rand_drug();
    let entry = prices.get_mut(drug as usize).unwrap();
    *entry = (*entry + 20) as u32; // Reduce price by 50%

    Self {
      e_type: EventType::DrugBust,
      e_msg: format!("Cops made a huge bust! {} prices have skyrocketed!", drug),
      e_drug: drug,
    }
  }

  pub fn drug_shipment(prices: &mut [u32; 7]) -> Self {
    let drug = get_rand_drug();
    let entry = prices.get_mut(drug as usize).unwrap();
    *entry = (*entry - 20) as u32; // Increase price by 50%

    Self {
      e_type: EventType::DrugShipment,
      e_msg: format!(
        "A huge shipment just came in! {} prices have bottomed out!",
        drug
      ),
      e_drug: drug,
    }
  }
}
