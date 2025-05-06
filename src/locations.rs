/*
  /##                                 /##     /##
 | ##                                | ##    |__/
 | ##  /######   /#######  /######  /######   /##  /######  /#######   /#######
 | ## /##__  ## /##_____/ |____  ##|_  ##_/  | ## /##__  ##| ##__  ## /##_____/
 | ##| ##  \ ##| ##        /#######  | ##    | ##| ##  \ ##| ##  \ ##|  ######
 | ##| ##  | ##| ##       /##__  ##  | ## /##| ##| ##  | ##| ##  | ## \____  ##
 | ##|  ######/|  #######|  #######  |  ####/| ##|  ######/| ##  | ## /#######/
 |__/ \______/  \_______/ \_______/   \___/  |__/ \______/ |__/  |__/|_______/
*/
#[derive(Default, PartialEq, Eq)]
pub enum Location {
  #[default]
  Fairfield,
  Oakwood,
  Lakeview,
  Highland,
  Edgewater,
  Centerville,
}

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
