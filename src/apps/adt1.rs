// 1. Use trait, struct and enum to implement OOP coding style.
// 2. Struct is a conduct type in Algebraic Data Type(ADT), represent a variable has m_1 * m_2 * ... difference.
// 3. Enum is a sum type in Algebraic Data Type(ADT), represent a variable has m_1 + m_2 + ... difference.
use std::convert::From;
use std::fmt;
use std::str::FromStr;
use std::string::String;

// define enum for colors
enum Color {
  Red,
  Yellow,
  Blue,
}

// implement enum methods
impl Color {
  fn to_fg_str(&self) -> &str {
    match *self {
      Color::Red => "31",
      Color::Yellow => "33",
      Color::Blue => "34",
    }
  }
  fn to_bg_str(&self) -> &str {
    match *self {
      Color::Red => "41",
      Color::Yellow => "43",
      Color::Blue => "44",
    }
  }
}

// implement trait:FromStr
impl FromStr for Color {
  type Err = ();

  fn from_str(src: &str) -> Result<Self, Self::Err> {
    let src = src.to_lowercase();

    match src.as_ref() {
      "red" => Ok(Color::Red),
      "yellow" => Ok(Color::Yellow),
      "blue" => Ok(Color::Blue),
      _ => Err(()),
    }
  }
}

// implement converter of str and String by trait:FromStr
impl<'a> From<&'a str> for Color {
  fn from(src: &str) -> Self {
    src.parse().unwrap_or(Color::Red)
  }
}
impl From<String> for Color {
  fn from(src: String) -> Self {
    src.parse().unwrap_or(Color::Red)
  }
}

// #[derive(Clone,Debug,PartialEq,Eq)]
struct ColoredString {
  input: String,
  fgcolor: Option<Color>,
  bgcolor: Option<Color>,
}

// implement default constructor
impl Default for ColoredString {
  fn default() -> Self {
    Self {
      input: String::default(),
      fgcolor: None,
      bgcolor: None,
    }
  }
}

// define methods by trait
trait Colorize {
  // methods return self for chain-calling.
  fn red(self) -> ColoredString;
  fn yellow(self) -> ColoredString;
  fn blue(self) -> ColoredString;
  fn color<S: Into<Color>>(self, color: S) -> ColoredString;
  fn on_red(self) -> ColoredString;
  fn on_yellow(self) -> ColoredString;
  fn on_blue(self) -> ColoredString;
  fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
}

// implement trait:Colorize for chain-calling
impl<'a> Colorize for ColoredString {
  fn color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString {
      fgcolor: Some(color.into()),
      ..self
    }
  }
  fn red(self) -> ColoredString {
    self.color(Color::Red)
  }
  fn yellow(self) -> ColoredString {
    self.color(Color::Yellow)
  }
  fn blue(self) -> ColoredString {
    self.color(Color::Blue)
  }
  fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString {
      bgcolor: Some(color.into()),
      ..self
    }
  }
  fn on_red(self) -> ColoredString {
    self.on_color(Color::Red)
  }
  fn on_yellow(self) -> ColoredString {
    self.on_color(Color::Yellow)
  }
  fn on_blue(self) -> ColoredString {
    self.on_color(Color::Blue)
  }
}

// implement trait:Colorize for str
impl<'a> Colorize for &'a str {
  fn color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString {
      fgcolor: Some(color.into()),
      input:String::from(self),
      ..ColoredString::default()
    }
  }
  fn red(self) -> ColoredString {
    self.color(Color::Red)
  }
  fn yellow(self) -> ColoredString {
    self.color(Color::Yellow)
  }
  fn blue(self) -> ColoredString {
    self.color(Color::Blue)
  }
  fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString {
      bgcolor: Some(color.into()),
      input:String::from(self),
      ..ColoredString::default()
    }
  }
  fn on_red(self) -> ColoredString {
    self.on_color(Color::Red)
  }
  fn on_yellow(self) -> ColoredString {
    self.on_color(Color::Yellow)
  }
  fn on_blue(self) -> ColoredString {
    self.on_color(Color::Blue)
  }
}

// implement additional methods
impl ColoredString {
  fn compute_style(&self) -> String {
    let mut res = String::from("\x1B[");
    let mut has_wrote = false;

    if let Some(ref bgcolor) = self.bgcolor {
      if has_wrote {
        res.push(';');
      }
      res.push_str(bgcolor.to_bg_str());
      has_wrote = true;
    }

    if let Some(ref fgcolor) = self.fgcolor {
      if has_wrote {
        res.push(';');
      }
      res.push_str(fgcolor.to_fg_str());
    }

    res.push('m');

    res
  }
}

// implement trait:display
impl fmt::Display for ColoredString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}{}{}",
      &self.compute_style(),
      &self.input.clone(),
      "\x1B[0m"
    )
  }
}

pub fn run() {
  println!("{} {}", "Hello".red().on_yellow(), "World".yellow().on_blue());
}
