use rand::Rng;

// MARK: price range constants
pub const PRICE_RANGES: [(u32, u32); 7] = [
  (40, 80),   // Weed
  (50, 90),   // Cocaine
  (80, 120),  // Meth
  (100, 140), // Heroin
  (30, 70),   // Ecstasy
  (40, 80),   // LSD
  (40, 80),   // Shrooms
];

// MARK: drug enum
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Drug {
  #[default]
  Weed,
  Cocaine,
  Meth,
  Heroin,
  Ecstasy,
  Lsd,
  Shrooms,
}

impl Drug {
  pub fn as_index(&self) -> usize {
    *self as usize
  }
}

// MARK: drug display
impl std::fmt::Display for Drug {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let names = [
      "Weed", "Cocaine", "Meth", "Heroin", "Ecstasy", "LSD", "Shrooms",
    ];
    write!(f, "{}", names[self.as_index()])
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
  prices[drug.as_index()]
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

  for (i, &(min, max)) in PRICE_RANGES.iter().enumerate() {
    prices[i] = rng.random_range(min..=max);
  }

  prices
}
