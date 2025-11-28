/// This module provides Typst compatible font information.
use super::repr::FontRepr;
use serde::{Deserialize, Serialize};
use typst_library::text::{self, Coverage, FontFlags, FontVariant};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TypstFontInfo {
  family: String,
  variant: FontVariant,
  coverage: Coverage,

  is_monospace: bool,
  is_serif: bool,
  is_variable: bool,
  has_math_table: bool,
}

impl TypstFontInfo {
  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    let info = &repr.info;
    let contains_flag = |flag| info.flags.contains(flag);

    Self {
      family: info.family.clone(),
      variant: info.variant,
      coverage: info.coverage.clone(),
      is_monospace: contains_flag(FontFlags::MONOSPACE),
      is_serif: contains_flag(FontFlags::SERIF),
      is_variable: contains_flag(FontFlags::VARIABLE),
      has_math_table: contains_flag(FontFlags::MATH),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LineMetrics {
  position: f64,
  thickness: f64,
}

impl LineMetrics {
  fn from_typst_metrics(metrics: &text::LineMetrics) -> Self {
    Self {
      position: metrics.position.get(),
      thickness: metrics.thickness.get(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScriptMetrics {
  width: f64,
  height: f64,
  horizontal_offset: f64,
  vertical_offset: f64,
}

impl ScriptMetrics {
  fn from_typst_metrics(metrics: &text::ScriptMetrics) -> Self {
    Self {
      width: metrics.width.get(),
      height: metrics.height.get(),
      horizontal_offset: metrics.horizontal_offset.get(),
      vertical_offset: metrics.vertical_offset.get(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TypstFontMetrics {
  units_per_em: f64,
  // all quantities below are in em
  ascender: f64,
  descender: f64,
  cap_height: f64,
  x_height: f64,

  strikethrough: LineMetrics,
  overline: LineMetrics,
  underline: LineMetrics,
  subscript: Option<ScriptMetrics>,
  superscript: Option<ScriptMetrics>,
}

impl TypstFontMetrics {
  #[allow(dead_code)]
  pub(super) fn ascender(&self) -> f64 {
    self.units_per_em * self.ascender
  }

  #[allow(dead_code)]
  pub(super) fn descender(&self) -> f64 {
    self.units_per_em * self.descender
  }

  #[allow(dead_code)]
  pub(super) fn cap_height(&self) -> f64 {
    self.units_per_em * self.cap_height
  }

  #[allow(dead_code)]
  pub(super) fn x_height(&self) -> f64 {
    self.units_per_em * self.x_height
  }
}

impl TypstFontMetrics {
  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    let typst_metrics = &repr.metrics;
    TypstFontMetrics {
      units_per_em: typst_metrics.units_per_em,
      ascender: typst_metrics.ascender.get(),
      descender: typst_metrics.descender.get(),
      cap_height: typst_metrics.cap_height.get(),
      x_height: typst_metrics.x_height.get(),
      strikethrough: LineMetrics::from_typst_metrics(
        &typst_metrics.strikethrough,
      ),
      overline: LineMetrics::from_typst_metrics(&typst_metrics.overline),
      underline: LineMetrics::from_typst_metrics(&typst_metrics.underline),
      subscript: typst_metrics
        .subscript
        .map(|sub| ScriptMetrics::from_typst_metrics(&sub)),
      superscript: typst_metrics
        .superscript
        .map(|sup| ScriptMetrics::from_typst_metrics(&sup)),
    }
  }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TypstFontIntrospection {
  info: TypstFontInfo,
  metrics: TypstFontMetrics,
}

impl TypstFontIntrospection {
  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    TypstFontIntrospection {
      info: TypstFontInfo::from_repr(repr),
      metrics: TypstFontMetrics::from_repr(repr),
    }
  }
}
