// TODO: finish this mod
#![allow(dead_code)]
/// This mod provides direct access to the font metrics
/// from opentype OS/2 table by wrapping `ttf_parser`
use crate::melt::repr::FontRepr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LineMetrics {
  position: f64,
  thickness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScriptMetrics {
  width: f64,
  height: f64,
  horizontal_offset: f64,
  vertical_offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FontMetrics {
  pub(crate) em: u16,
  pub(crate) ascender: i16,
  pub(crate) descender: i16,
  pub(crate) line_gap: i16,
  pub(crate) height: i16,
  pub(crate) italic_angle: f32,
}

impl FontMetrics {
  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    let ttf = &repr.ttf_parser;
    FontMetrics {
      em: ttf.units_per_em(),
      ascender: ttf.ascender(),
      descender: ttf.descender(),
      line_gap: ttf.line_gap(),
      height: ttf.height(),
      italic_angle: ttf.italic_angle(),
    }
  }
}
