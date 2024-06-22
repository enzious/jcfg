mod fmt;

pub struct Serializer<'d> {
    dst: &'d mut String,
    settings: crate::fmt::DocumentFormatter,
}
