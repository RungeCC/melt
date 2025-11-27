use serde::{Deserialize, Serialize};
use skrifa::FontRef;
use skrifa::raw::tables::meta::DataMapRecord;
use skrifa::raw::{Offset, TableProvider, tables};
use std::collections::BTreeSet;
use ttf_parser::opentype_layout;

use crate::melt::repr::FontRepr;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesignedLanguages(BTreeSet<String>);

impl DesignedLanguages {
  pub fn from_skrifa_ref(font: &FontRef) -> Option<Self> {
    let meta = font.meta().ok()?;
    let data = meta.offset_data();
    let tag_data = |record: &DataMapRecord| {
      let offset = record.data_offset().to_usize();
      let length = record.data_length() as usize;
      data.read_array::<u8>(offset..offset + length).ok()
    };
    meta
      .data_maps()
      .iter()
      .find(|record| record.tag == tables::meta::DLNG)
      .and_then(|record| {
        tag_data(record).and_then(|bytes| {
          std::str::from_utf8(bytes).ok().map(|str| {
            DesignedLanguages(str.split(", ").map(String::from).collect())
          })
        })
      })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct SupportedLanguages(BTreeSet<String>);

impl SupportedLanguages {
  pub fn from_skrifa_ref(font: &FontRef) -> Option<Self> {
    let meta = font.meta().ok()?;
    let data = meta.offset_data();
    let tag_data = |record: &DataMapRecord| {
      let offset = record.data_offset().to_usize();
      let length = record.data_length() as usize;
      data.read_array::<u8>(offset..offset + length).ok()
    };
    meta
      .data_maps()
      .iter()
      .find(|record| record.tag == tables::meta::SLNG)
      .and_then(|record| {
        tag_data(record).and_then(|bytes| {
          std::str::from_utf8(bytes).ok().map(|str| {
            SupportedLanguages(str.split(", ").map(String::from).collect())
          })
        })
      })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontScript {
  tag: String,
  languages: BTreeSet<String>,
  default_language: Option<String>,
}

impl FontScript {
  fn from_script_table(table: opentype_layout::Script) -> Self {
    let tag = table.tag.to_string();
    let default_language =
      table.default_language.map(|lang| lang.tag.to_string());
    let languages = table
      .languages
      .into_iter()
      .map(|lang| lang.tag.to_string())
      .collect();
    Self {
      tag,
      languages,
      default_language,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FontScripts {
  scripts: BTreeSet<String>,
  languages: BTreeSet<String>,
  designed: DesignedLanguages,
  supported: SupportedLanguages,
}

impl FontScripts {
  fn scripts_from_layout_table(
    table: opentype_layout::LayoutTable,
  ) -> Vec<FontScript> {
    table
      .scripts
      .into_iter()
      .map(FontScript::from_script_table)
      .collect()
  }

  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    let tables = repr.ttf_parser.tables();
    let gpos = tables.gpos.map_or(vec![], Self::scripts_from_layout_table);
    let gsub = tables.gsub.map_or(vec![], Self::scripts_from_layout_table);
    let scripts = gpos
      .clone()
      .into_iter()
      .map(|script| script.tag)
      .chain(gsub.clone().into_iter().map(|script| script.tag))
      .collect();
    let languages = gpos
      .into_iter()
      .flat_map(|script| script.languages)
      .chain(gsub.into_iter().flat_map(|script| script.languages))
      .collect();
    FontScripts {
      scripts,
      languages,
      designed: DesignedLanguages::from_skrifa_ref(&repr.font_ref)
        .unwrap_or_default(),
      supported: SupportedLanguages::from_skrifa_ref(&repr.font_ref)
        .unwrap_or_default(),
    }
  }
}
