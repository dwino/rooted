pub use crate::prelude::*;
// GENERAL
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
#[derive(Clone, PartialEq)]
pub struct Name(pub String);
