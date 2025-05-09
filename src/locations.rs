// MARK: Location enum
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Location {
  #[default]
  Fairfield,
  Oakwood,
  Lakeview,
  Highland,
  Edgewater,
  Centerville,
}

// MARK: Location display
impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Location::Fairfield => write!(f, "Fairfield"),
      Location::Oakwood => write!(f, "Oakwood"),
      Location::Lakeview => write!(f, "Lakeview"),
      Location::Highland => write!(f, "Highland"),
      Location::Edgewater => write!(f, "Edgewater"),
      Location::Centerville => write!(f, "Centerville"),
    }
  }
}
