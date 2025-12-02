mod repr;
pub mod ttf;
mod typst;

use repr::FontRepr;
use serde::{Deserialize, Serialize};

use ttf::features::FontFeatures;
use ttf::glyphs::{GlyphsInfo, GlyphsShapes};
use ttf::metrics::FontMetrics;
use ttf::names::FontNames;
use ttf::scripts::FontScripts;
use typst::TypstFontIntrospection;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontProperties {
  names: FontNames,
  scripts: FontScripts,
  features: FontFeatures,
}

impl FontProperties {
  fn from_repr(repr: &FontRepr) -> Self {
    FontProperties {
      names: FontNames::from_repr(repr),
      scripts: FontScripts::from_repr(repr),
      features: FontFeatures::from_repr(repr),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontInfo {
  properties: FontProperties,
  metrics: FontMetrics,
  typst: TypstFontIntrospection,
}

impl FontInfo {
  pub fn new(data: &[u8], index: u32) -> Option<Self> {
    let repr = FontRepr::new(data, index)?;
    Some(FontInfo {
      properties: FontProperties::from_repr(&repr),
      metrics: FontMetrics::from_repr(&repr),
      typst: TypstFontIntrospection::from_repr(&repr),
    })
  }

  pub fn from_collections(data: &[u8]) -> impl Iterator<Item = Option<Self>> {
    let counts = ttf_parser::fonts_in_collection(data).unwrap_or(1);
    (0..counts).map(move |id| FontInfo::new(data, id))
  }
}

pub fn glyphs_infos(
  data: &[u8],
  index: u32,
  codes: impl Iterator<Item = Option<char>>,
) -> Option<GlyphsInfo> {
  let repr = FontRepr::new(data, index)?;
  Some(GlyphsInfo::from_option_iter(&repr, codes))
}

pub fn glyphs_shapes(
  data: &[u8],
  index: u32,
  codes: impl Iterator<Item = Option<char>>,
) -> Option<GlyphsShapes> {
  let repr = FontRepr::new(data, index)?;
  Some(GlyphsShapes::from_option_iter(&repr, codes))
}
