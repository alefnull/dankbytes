/*
        /##
       | ##
   /#######  /######  /##   /##  /######   /#######
  /##__  ## /##__  ##| ##  | ## /##__  ## /##_____/
 | ##  | ##| ##  \__/| ##  | ##| ##  \ ##|  ######
 | ##  | ##| ##      | ##  | ##| ##  | ## \____  ##
 |  #######| ##      |  ######/|  ####### /#######/
  \_______/|__/       \______/  \____  ##|_______/
                                /##  \ ##
                               |  ######/
                                \______/
*/
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
