mod encoding;
mod language;

use crate::melt::repr::FontRepr;
use encoding::{MacintoshEncoding, PlatformEncoding};
use language::MacintoshLanguage;
use serde::{Deserialize, Serialize};
use ttf_parser::{self, Face};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontName {
  name: Option<String>,
  language: Option<String>,
  platform_encoding: PlatformEncoding,
}

impl FontName {
  fn decode_macos_roman(coded: &[u8]) -> String {
    /// - from: <https://en.wikipedia.org/wiki/Mac_OS_Roman>
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

  fn from_ttf_name(name: ttf_parser::name::Name) -> Self {
    let platform_encoding =
      PlatformEncoding::from_indices(name.platform_id, name.encoding_id);
    let language = match platform_encoding {
      PlatformEncoding::Windows(_) => Some(name.language().to_string()),
      PlatformEncoding::Macintosh(_) => {
        MacintoshLanguage::from_index(name.language_id)
          .map(|lang| lang.to_string())
      }
      _ => None,
    };
    let name = name.to_string().or(match platform_encoding {
      PlatformEncoding::Macintosh(MacintoshEncoding::Roman) => {
        Some(Self::decode_macos_roman(name.name))
      }
      _ => None,
    });
    FontName {
      name,
      language,
      platform_encoding,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FontNameVec(Vec<FontName>);

impl FontNameVec {
  fn from_ttf(ttf: &Face, name_id: u16) -> Self {
    let name_vec: Vec<FontName> = ttf
      .names()
      .into_iter()
      .filter(|entry| entry.name_id == name_id)
      .map(FontName::from_ttf_name)
      .collect();
    FontNameVec(name_vec)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct FontNames {
  copyright_notice: FontNameVec,
  family: FontNameVec,
  subfamily: FontNameVec,
  unique_id: FontNameVec,
  full_name: FontNameVec,
  version: FontNameVec,
  post_script_name: FontNameVec,
  trademark: FontNameVec,
  manufacturer: FontNameVec,
  designer: FontNameVec,
  description: FontNameVec,
  vendor_url: FontNameVec,
  designer_url: FontNameVec,
  license: FontNameVec,
  license_url: FontNameVec,
  typographic_family: FontNameVec,
  typographic_subfamily: FontNameVec,
  compatible_full: FontNameVec,
  sample_text: FontNameVec,
  post_script_cid: FontNameVec,
  wws_family: FontNameVec,
  wws_subfamily: FontNameVec,
  light_background_palette: FontNameVec,
  dark_background_palette: FontNameVec,
  variations_post_script_name_prefix: FontNameVec,
}
macro_rules! setters {
  ($($id:ident => $key:expr), * $(,)?) => {
    $(fn $id(
        self: &mut Self,
        value: $crate::melt::ttf::names::FontNameVec
      ) -> &mut Self {
        self.$id = value;
        self
    })*
  };
}
impl FontNames {
  setters![
    copyright_notice => COPYRIGHT_NOTICE,
    family => FAMILY,
    subfamily => SUBFAMILY,
    unique_id => UNIQUE_ID,
    full_name => FULL_NAME,
    version => VERSION,
    post_script_name => POST_SCRIPT_NAME,
    trademark => TRADEMARK,
    manufacturer => MANUFACTURER,
    designer => DESIGNER,
    description => DESCRIPTION,
    vendor_url => VENDOR_URL,
    designer_url => DESIGNER_URL,
    license => LICENSE,
    license_url => LICENSE_URL,
    typographic_family => TYPOGRAPHIC_FAMILY,
    typographic_subfamily => TYPOGRAPHIC_SUBFAMILY,
    compatible_full => COMPATIBLE_FULL,
    sample_text => SAMPLE_TEXT,
    post_script_cid => POST_SCRIPT_CID,
    wws_family => WWS_FAMILY,
    wws_subfamily => WWS_SUBFAMILY,
    light_background_palette => LIGHT_BACKGROUND_PALETTE,
    dark_background_palette => DARK_BACKGROUND_PALETTE,
    variations_post_script_name_prefix => VARIATIONS_POST_SCRIPT_NAME_PREFIX,
  ];

  #[allow(clippy::wildcard_imports)]
  pub(crate) fn from_repr(repr: &FontRepr) -> Self {
    use ttf_parser::name_id::*;
    let ttf = &repr.ttf_parser;

    macro_rules! make_font_names {
      ($fonts:ident, $($id:ident => $key:expr), * $(,)?) => {
        $fonts $(.$id(
          $crate::melt::ttf::names::FontNameVec::from_ttf(ttf, $key)
        ))*;
      };
    }
    let mut fonts = FontNames::default();
    make_font_names![
      fonts,
      copyright_notice => COPYRIGHT_NOTICE,
      family => FAMILY,
      subfamily => SUBFAMILY,
      unique_id => UNIQUE_ID,
      full_name => FULL_NAME,
      version => VERSION,
      post_script_name => POST_SCRIPT_NAME,
      trademark => TRADEMARK,
      manufacturer => MANUFACTURER,
      designer => DESIGNER,
      description => DESCRIPTION,
      vendor_url => VENDOR_URL,
      designer_url => DESIGNER_URL,
      license => LICENSE,
      license_url => LICENSE_URL,
      typographic_family => TYPOGRAPHIC_FAMILY,
      typographic_subfamily => TYPOGRAPHIC_SUBFAMILY,
      compatible_full => COMPATIBLE_FULL,
      sample_text => SAMPLE_TEXT,
      post_script_cid => POST_SCRIPT_CID,
      wws_family => WWS_FAMILY,
      wws_subfamily => WWS_SUBFAMILY,
      light_background_palette => LIGHT_BACKGROUND_PALETTE,
      dark_background_palette => DARK_BACKGROUND_PALETTE,
      variations_post_script_name_prefix => VARIATIONS_POST_SCRIPT_NAME_PREFIX,
    ];
    fonts
  }
}
