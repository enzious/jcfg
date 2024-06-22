use serde::{ser, Serialize};

use crate::error::{Error, Result};

#[derive(Default)]
pub struct Serializer {
  output: String,
  depth: usize,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
  T: Serialize,
{
  let mut serializer = Serializer {
    ..Default::default()
  };
  value.serialize(&mut serializer)?;
  Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
  type Ok = ();

  type Error = Error;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> Result<()> {
    self.output += if v { "true" } else { "false" };
    Ok(())
  }

  fn serialize_i8(self, v: i8) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i16(self, v: i16) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i32(self, v: i32) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i64(self, v: i64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u16(self, v: u16) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u32(self, v: u32) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u64(self, v: u64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_f32(self, v: f32) -> Result<()> {
    self.serialize_f64(f64::from(v))
  }

  fn serialize_f64(self, v: f64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_char(self, v: char) -> Result<()> {
    self.serialize_str(&v.to_string())
  }

  fn serialize_str(self, v: &str) -> Result<()> {
    self.output += "\"";
    self.output += v;
    self.output += "\"";
    Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<()> {
    use serde::ser::SerializeSeq;
    let mut seq = self.serialize_seq(Some(v.len()))?;
    for byte in v {
      seq.serialize_element(byte)?;
    }
    seq.end()
  }

  fn serialize_none(self) -> Result<()> {
    self.serialize_unit()
  }

  fn serialize_some<T>(self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<()> {
    self.output += "null";
    Ok(())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
    self.serialize_unit()
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<()> {
    self.serialize_str(variant)
  }

  fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    self.output += "{";
    variant.serialize(&mut *self)?;
    self.output += ":";
    value.serialize(&mut *self)?;
    self.output += "}";
    Ok(())
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    self.output += "[";
    Ok(self)
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    self.output += "{";
    variant.serialize(&mut *self)?;
    self.output += ":[";
    Ok(self)
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    self.depth += 1;
    Ok(self)
  }

  fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
    self.serialize_map(Some(len))
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    self.output += "{";
    variant.serialize(&mut *self)?;
    self.output += ":{";
    Ok(self)
  }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.ends_with('[') {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    self.output += "]";
    Ok(())
  }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.ends_with('[') {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    self.output += "]";
    Ok(())
  }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.ends_with('[') {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    self.output += "]";
    Ok(())
  }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.ends_with('[') {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    self.output += "]}";
    Ok(())
  }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_key<T>(&mut self, key: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += "\n";
    }
    key.serialize(&mut **self)
  }

  fn serialize_value<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    match self.depth {
      0 => self.output += ": ",
      _ => self.output += " = ",
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    if self.depth > 0 {
      self.output += "}";
    }
    Ok(())
  }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += "\n";

      if self.depth == 1 {
        self.output += "\n";
      }
    }
    self.output += &indent::indent_all_by((self.depth - 1) * 2, key);
    match self.depth {
      1 => self.output += ": ",
      _ => self.output += " = ",
    }
    if self.depth == 1 {
      self.output += "{";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    if self.depth != 1 {
      self.output += "\n}";
    }
    self.depth -= 1;
    Ok(())
  }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.ends_with('{') {
      self.output += ",";
    }
    key.serialize(&mut **self)?;
    self.output += ":";
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    self.output += "}}";
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::error::Result;

  use super::to_string;

  pub fn debug_to_string<T>(value: &T) -> Result<String>
  where
    T: serde::ser::Serialize,
  {
    to_string(value).map(|output| {
      println!("output: {}", &output);
      output
    })
  }

  #[test]
  fn test_struct() {
    use serde_derive::Serialize;

    #[derive(Serialize)]
    struct General {
      name: &'static str,
      protected_folders: Vec<&'static str>,
    }

    #[derive(Serialize)]
    struct Test {
      general: General,
      general2: General,
    }

    let test = Test {
      general: General {
        name: "blah",
        protected_folders: vec!["config/", "docs/"],
      },
      general2: General {
        name: "blah",
        protected_folders: vec!["config/", "docs/"],
      },
    };

    let expected = r#"
general: {
  name = "blah"
  protected_folders = ["config/","docs/"]
}

general2: {
  name = "blah"
  protected_folders = ["config/","docs/"]
}"#
      .trim();
    assert_eq!(debug_to_string(&test).unwrap(), expected);
  }

  #[test]
  fn test_enum() {
    use serde_derive::Serialize;

    #[derive(Serialize)]
    enum E {
      Unit,
      Newtype(u32),
      Tuple(u32, u32),
      Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = r#""Unit""#;
    assert_eq!(debug_to_string(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = r#"{"Newtype":1}"#;
    assert_eq!(debug_to_string(&n).unwrap(), expected);

    let t = E::Tuple(1, 2);
    let expected = r#"{"Tuple":[1,2]}"#;
    assert_eq!(debug_to_string(&t).unwrap(), expected);

    let s = E::Struct { a: 1 };
    let expected = r#"{"Struct":{"a":1}}"#;
    assert_eq!(debug_to_string(&s).unwrap(), expected);
  }
}
