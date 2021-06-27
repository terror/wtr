use crate::common::*;

const CLOUDS: &str = "
            .-~~~-.
    .- ~ ~-(       )_ _
   /                     ~ -.
  |                           \
   \\                         .'
     ~- . _____________ . -~
    ";

const RAIN: &str = "
      _(  )_( )_
     (_   _    _)
    / /(_) (__)
   / / / / / /
  / / / / / /
    ";

const SUN: &str = "
        ;   :   ;
     .   \\_,!,_/   ,
      `.,'     `.,'
       /         \
  ~ -- :         : -- ~
       \\         /
      ,'`._   _.'`.
     '   / `!` \\   `
        ;   :   ;
    ";

#[derive(Debug)]
pub enum Ascii {
  Clouds,
  Rain,
  Sun,
  Unknown(String),
}

impl From<String> for Ascii {
  fn from(s: String) -> Self {
    match &s[..] {
      "Clouds" => Ascii::Clouds,
      "Rain" => Ascii::Rain,
      "Sun" => Ascii::Sun,
      _ => Ascii::Unknown(s),
    }
  }
}

impl fmt::Display for Ascii {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Ascii::Clouds => write!(f, "{}", CLOUDS),
      Ascii::Rain => write!(f, "{}", RAIN),
      Ascii::Sun => write!(f, "{}", SUN),
      _ => write!(f, "Unknown."),
    }
  }
}
