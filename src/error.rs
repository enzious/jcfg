use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Message(String),

  Eof,
  Syntax,
  ExpectedBoolean,
  ExpectedInteger,
  ExpectedString,
  ExpectedNull,
  ExpectedArray,
  ExpectedArrayComma,
  ExpectedArrayEnd,
  ExpectedMap,
  ExpectedMapColon,
  ExpectedMapComma,
  ExpectedMapEnd,
  ExpectedEnum,
  TrailingCharacters,
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Message(msg) => formatter.write_str(msg),
      Error::Eof => formatter.write_str("unexpected end of input"),
      Error::ExpectedBoolean => formatter.write_str("Expected boolean"),
      Error::ExpectedInteger => formatter.write_str("Expected integer"),
      Error::ExpectedString => formatter.write_str("Expected string"),
      Error::ExpectedNull => formatter.write_str("Expected null"),
      Error::ExpectedArray => formatter.write_str("Expected array"),
      Error::ExpectedArrayComma => formatter.write_str("Expected array comma"),
      Error::ExpectedArrayEnd => formatter.write_str("Expected array end"),
      Error::ExpectedMap => formatter.write_str("Expected map"),
      Error::ExpectedMapColon => formatter.write_str("Expected map colon"),
      Error::ExpectedMapComma => formatter.write_str("Expected map comma"),
      Error::ExpectedMapEnd => formatter.write_str("Expected map end"),
      Error::ExpectedEnum => formatter.write_str("Expected enum"),
      Error::TrailingCharacters => formatter.write_str("Unexpected trailing characters"),
      _ => unimplemented!(),
    }
  }
}

impl std::error::Error for Error {}
