use rand::Rng;

// MARK: price range constants
pub const WEED_RANGE: (u32, u32) = (40, 80);
pub const COCAINE_RANGE: (u32, u32) = (50, 90);
pub const METH_RANGE: (u32, u32) = (80, 120);
pub const HEROIN_RANGE: (u32, u32) = (100, 140);
pub const ECSTASY_RANGE: (u32, u32) = (30, 70);
pub const LSD_RANGE: (u32, u32) = (40, 80);
pub const SHROOMS_RANGE: (u32, u32) = (40, 80);

// MARK: drug enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Drug {
  Weed,
  Cocaine,
  Meth,
  Heroin,
  Ecstasy,
  Lsd,
  Shrooms,
}

// MARK: drug display
impl std::fmt::Display for Drug {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Drug::Weed => write!(f, "Weed"),
      Drug::Cocaine => write!(f, "Cocaine"),
      Drug::Meth => write!(f, "Meth"),
      Drug::Heroin => write!(f, "Heroin"),
      Drug::Ecstasy => write!(f, "Ecstasy"),
      Drug::Lsd => write!(f, "LSD"),
      Drug::Shrooms => write!(f, "Shrooms"),
    }
  }
}

// MARK: get_drug_list()
pub fn get_drug_list() -> [Drug; 7] {
  [
    Drug::Weed,
    Drug::Cocaine,
    Drug::Meth,
    Drug::Heroin,
    Drug::Ecstasy,
    Drug::Lsd,
    Drug::Shrooms,
  ]
}

pub fn get_drug_price(drug: Drug, prices: &[u32; 7]) -> u32 {
  match drug {
    Drug::Weed => prices[Drug::Weed as usize],
    Drug::Cocaine => prices[Drug::Cocaine as usize],
    Drug::Meth => prices[Drug::Meth as usize],
    Drug::Heroin => prices[Drug::Heroin as usize],
    Drug::Ecstasy => prices[Drug::Ecstasy as usize],
    Drug::Lsd => prices[Drug::Lsd as usize],
    Drug::Shrooms => prices[Drug::Shrooms as usize],
  }
}

// MARK: get_rand_drug()
pub fn get_rand_drug() -> Drug {
  let mut rng = rand::rng();
  let drugs = get_drug_list();
  let rand_index = rng.random_range(0..drugs.len());
  drugs[rand_index]
}

// MARK: get_rand_prices()
pub fn get_rand_prices() -> [u32; 7] {
  let mut rng = rand::rng();
  let mut prices = [0; 7];

  for drug in get_drug_list() {
    let price = match drug {
      Drug::Weed => rng.random_range(WEED_RANGE.0..=WEED_RANGE.1),
      Drug::Cocaine => rng.random_range(COCAINE_RANGE.0..=COCAINE_RANGE.1),
      Drug::Meth => rng.random_range(METH_RANGE.0..=METH_RANGE.1),
      Drug::Heroin => rng.random_range(HEROIN_RANGE.0..=HEROIN_RANGE.1),
      Drug::Ecstasy => rng.random_range(ECSTASY_RANGE.0..=ECSTASY_RANGE.1),
      Drug::Lsd => rng.random_range(LSD_RANGE.0..=LSD_RANGE.1),
      Drug::Shrooms => rng.random_range(SHROOMS_RANGE.0..=SHROOMS_RANGE.1),
    };
    prices[drug as usize] = price;
  }

  prices
}
