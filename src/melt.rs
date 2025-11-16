mod repr;
mod ttf;
mod typst;

use repr::FontRepr;
use serde::{Deserialize, Serialize};
use ttf_parser;

use ttf::{FontFeatures, FontNames, FontScripts};
use typst::{FontMetrics, TypstFontInfo};

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
  typst: TypstFontInfo,
}

impl FontInfo {
  pub fn new(data: &[u8], index: u32) -> Option<Self> {
    let repr = FontRepr::new(data, index)?;
    Some(FontInfo {
      properties: FontProperties::from_repr(&repr),
      metrics: FontMetrics::from_repr(&repr),
      typst: TypstFontInfo::from_repr(&repr),
    })
  }

  pub fn from_collections(data: &[u8]) -> impl Iterator<Item = Option<Self>> {
    let counts = ttf_parser::fonts_in_collection(data).unwrap_or(1);
    (0..counts).map(move |id| FontInfo::new(data, id))
  }
}
