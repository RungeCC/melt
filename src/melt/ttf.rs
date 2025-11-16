use super::repr::FontRepr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ttf_parser::{self, Face, PlatformId, opentype_layout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct FontFeatures(BTreeSet<String>);
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
  pub(super) fn from_repr(repr: &FontRepr) -> Self {
    let tables = repr.ttf.tables();
    let gpos = tables.gpos.map_or(BTreeSet::new(), |layout| {
      Self::features_from_layout_table(&layout)
    });
    let gsub = tables.gsub.map_or(BTreeSet::new(), |layout| {
      Self::features_from_layout_table(&layout)
    });

    FontFeatures(gpos.into_iter().chain(gsub.into_iter()).collect())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontScript {
  tag: String,
  languages: BTreeSet<String>,
  default_language: Option<String>,
}
impl FontScript {
  fn from_script_table(table: &opentype_layout::Script) -> Self {
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
pub(super) struct FontScripts {
  scripts: BTreeSet<String>,
  languages: BTreeSet<String>,
}

impl FontScripts {
  fn scripts_from_layout_table(
    table: &opentype_layout::LayoutTable,
  ) -> Vec<FontScript> {
    table
      .scripts
      .into_iter()
      .map(|script| FontScript::from_script_table(&script))
      .collect()
  }
  pub(super) fn from_repr(repr: &FontRepr) -> Self {
    let tables = repr.ttf.tables();
    let gpos = tables
      .gpos
      .map_or(vec![], |layout| Self::scripts_from_layout_table(&layout));
    let gsub = tables
      .gsub
      .map_or(vec![], |layout| Self::scripts_from_layout_table(&layout));
    let all_scripts = gpos
      .clone()
      .into_iter()
      .map(|script| script.tag)
      .chain(gsub.clone().into_iter().map(|script| script.tag))
      .collect();
    let all_languages = gpos
      .into_iter()
      .map(|script| script.languages)
      .flatten()
      .chain(gsub.into_iter().map(|script| script.languages).flatten())
      .collect();
    FontScripts {
      scripts: all_scripts,
      languages: all_languages,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontName {
  name: String,
  language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct FontNames {
  ps_name: Option<FontName>,
  family: Option<FontName>,
  subfamily: Option<FontName>,
  version: Option<FontName>,
  full_name: Option<FontName>,
  unique_id: Option<FontName>,
  typographic_family: Option<FontName>,
  typographic_subfamily: Option<FontName>,
  designer: Option<FontName>,
  copyright: Option<FontName>,
}
macro_rules! setters {
  ($($id:ident => $key:expr), * $(,)?) => {
    $(fn $id(self: &mut Self, value: Option<FontName>) -> &mut Self {
      self.$id = value;
      self
    })*
  };
}
impl FontNames {
  fn decode_macos_roman(coded: &[u8]) -> String {
    /// - from: https://en.wikipedia.org/wiki/Mac_OS_Roman
    /// - see also: typst source code
    /// - note: \u{a0} = nbsp
    /// - note: \u{f8ff} = solid apple logo
    const TABLE: [char; 128] = [
      'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å', 'ç',
      'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ',
      'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶', 'ß', '®', '©',
      '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥', '¥', 'µ', '∂', '∑',
      '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿', '¡', '¬', '√', 'ƒ', '≈',
      '∆', '«', '»', '…', '\u{a0}', 'À', 'Ã', 'Õ', 'Œ', 'œ', '–', '—', '“',
      '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄', '€', '‹', '›', 'ﬁ', 'ﬂ', '‡',
      '·', '‚', '„', '‰', 'Â', 'Ê', 'Á', 'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó',
      'Ô', '\u{f8ff}', 'Ò', 'Ú', 'Û', 'Ù', 'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚',
      '¸', '˝', '˛', 'ˇ',
    ];

    coded
      .iter()
      .copied()
      .map(|code| match code {
        0..128 => code as char,
        _ => TABLE[(code - 128) as usize],
      })
      .collect()
  }

  /// Currently, could only work with unicode name
  fn query_ttf_name(ttf: &Face, name_id: u16) -> Option<FontName> {
    ttf.names().into_iter().find_map(|entry| {
      if entry.name_id == name_id {
        let language = entry.language().to_string();
        entry
          .to_string()
          .or(
            if entry.platform_id == PlatformId::Macintosh
              && entry.encoding_id == 0
            {
              Some(Self::decode_macos_roman(entry.name))
            } else {
              None
            },
          )
          .map(|name| FontName { name, language })
      } else {
        None
      }
    })
  }

  setters![
    ps_name => POST_SCRIPT_NAME,
    family => FAMILY,
    subfamily => SUBFAMILY,
    version => VERSION,
    full_name => FULL_NAME,
    unique_id => UNIQUE_ID,
    typographic_family => TYPOGRAPHIC_FAMILY,
    typographic_subfamily => TYPOGRAPHIC_SUBFAMILY,
    designer => DESIGNER,
    copyright => COPYRIGHT_NOTICE,
  ];

  pub(super) fn from_repr(repr: &FontRepr) -> Self {
    use ttf_parser::name_id::*;
    let ttf = &repr.ttf;

    macro_rules! make_font_names {
      ($fonts:ident, $($id:ident => $key:expr), * $(,)?) => {
        $fonts $(.$id(Self::query_ttf_name(ttf, $key)))*;
      };
    }
    let mut fonts = FontNames::default();
    make_font_names![
      fonts,
      ps_name => POST_SCRIPT_NAME,
      family => FAMILY,
      subfamily => SUBFAMILY,
      version => VERSION,
      full_name => FULL_NAME,
      unique_id => UNIQUE_ID,
      typographic_family => TYPOGRAPHIC_FAMILY,
      typographic_subfamily => TYPOGRAPHIC_SUBFAMILY,
      designer => DESIGNER,
      copyright => COPYRIGHT_NOTICE,
    ];
    fonts
  }
}
