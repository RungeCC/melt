use crate::melt::repr::FontRepr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ttf_parser::{self, opentype_layout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FontFeatures(BTreeSet<String>);
impl FontFeatures {
  fn features_from_layout_table(
    table: &opentype_layout::LayoutTable,
  ) -> BTreeSet<String> {
    table
      .features
      .into_iter()
      .map(|feat| feat.tag.to_string())
      .collect()
  }

  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    let tables = repr.ttf_parser.tables();
    let gpos = tables.gpos.map_or(BTreeSet::new(), |layout| {
      Self::features_from_layout_table(&layout)
    });
    let gsub = tables.gsub.map_or(BTreeSet::new(), |layout| {
      Self::features_from_layout_table(&layout)
    });

    FontFeatures(gpos.into_iter().chain(gsub).collect())
  }
}
