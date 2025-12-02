use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use ttf_parser::PlatformId;

/// OpenType platform and encoding.
///
/// From: <https://learn.microsoft.com/en-us/typography/opentype/spec/name#platform-ids>
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(super) enum PlatformEncoding {
  Windows(WindowsEncoding),
  Macintosh(MacintoshEncoding),
  Unicode(UnicodeEncoding),
  Iso(u16),
  Custom(u16),
}

#[allow(clippy::enum_glob_use)]
impl PlatformEncoding {
  pub(super) fn from_indices(
    platform_id: PlatformId,
    encoding_id: u16,
  ) -> Self {
    use PlatformId::*;
    match platform_id {
      Windows => {
        PlatformEncoding::Windows(WindowsEncoding::from_index(encoding_id))
      }
      Macintosh => PlatformEncoding::Macintosh(
        MacintoshEncoding::from_index(encoding_id),
      ),
      Unicode => {
        PlatformEncoding::Unicode(UnicodeEncoding::from_index(encoding_id))
      }
      Iso => PlatformEncoding::Iso(encoding_id),
      Custom => PlatformEncoding::Custom(encoding_id),
    }
  }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub(super) enum UnicodeEncoding {
  Unicode1_0 = 0,
  Unicode1_1 = 1,
  IsoIec10646 = 2,
  UnicodeBMP = 3,
  UnicodeFull = 4,
  Other(u16),
}

impl UnicodeEncoding {
  pub(super) fn from_index(index: u16) -> Self {
    match index {
      0 => UnicodeEncoding::Unicode1_0,
      1 => UnicodeEncoding::Unicode1_1,
      2 => UnicodeEncoding::IsoIec10646,
      3 => UnicodeEncoding::UnicodeBMP,
      4 => UnicodeEncoding::UnicodeFull,
      x => UnicodeEncoding::Other(x),
    }
  }
}

impl Serialize for UnicodeEncoding {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      UnicodeEncoding::Unicode1_0 => serializer.serialize_str("Unicode 1.0"),
      UnicodeEncoding::Unicode1_1 => serializer.serialize_str("Unicode 1.1"),
      UnicodeEncoding::IsoIec10646 => {
        serializer.serialize_str("ISO/IEC 10646")
      }
      UnicodeEncoding::UnicodeBMP => serializer.serialize_str("Unicode BMP"),
      UnicodeEncoding::UnicodeFull => {
        serializer.serialize_str("Unicode Full")
      }
      UnicodeEncoding::Other(x) => {
        serializer.serialize_str(&format!("Other({x:02x})"))
      }
    }
  }
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize)]
pub(super) enum MacintoshEncoding {
  Roman = 0,
  Japanese = 1,
  ChineseTradational = 2,
  Korean = 3,
  Arabic = 4,
  Hebrew = 5,
  Greek = 6,
  Russian = 7,
  RSymbol = 8,
  Devanagari = 9,
  Gurmukhi = 10,
  Gujarati = 11,
  Odia = 12,
  Bangla = 13,
  Tamil = 14,
  Telugu = 15,
  Kannada = 16,
  Malayalam = 17,
  Sinhalese = 18,
  Burmese = 19,
  Khmer = 20,
  Thai = 21,
  Laotian = 22,
  Georgian = 23,
  Armenian = 24,
  ChineseSimplified = 25,
  Tibetan = 26,
  Mongolian = 27,
  Geez = 28,
  Slavic = 29,
  Vietnamese = 30,
  Sindhi = 31,
  Uninterpreted = 32,
}

impl MacintoshEncoding {
  fn from_index(index: u16) -> Self {
    match index {
      // Safety: we know that the index is valid in range [0, 33)
      valid @ 0..33 => unsafe {
        std::mem::transmute::<u16, MacintoshEncoding>(valid)
      },
      _ => MacintoshEncoding::Uninterpreted,
    }
  }
}

struct MacintoshEncodingVisitor;

impl Visitor<'_> for MacintoshEncodingVisitor {
  type Value = MacintoshEncoding;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a valid Macintosh Language ID (u16)")
  }

  fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
  where
    E: Error,
  {
    Ok(MacintoshEncoding::from_index(value))
  }
}

impl<'de> Deserialize<'de> for MacintoshEncoding {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_u16(MacintoshEncodingVisitor)
  }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(super) enum WindowsEncoding {
  Symbol = 0,
  UnicodeBMP = 1,
  ShiftJIS = 2,
  Rpc = 3,
  Big5 = 4,
  Wansung = 5,
  Johab = 6,
  UnicodeFull = 10,
  Other(u16),
}

impl WindowsEncoding {
  fn from_index(index: u16) -> Self {
    match index {
      0 => WindowsEncoding::Symbol,
      1 => WindowsEncoding::UnicodeBMP,
      2 => WindowsEncoding::ShiftJIS,
      3 => WindowsEncoding::Rpc,
      4 => WindowsEncoding::Big5,
      5 => WindowsEncoding::Wansung,
      6 => WindowsEncoding::Johab,
      10 => WindowsEncoding::UnicodeFull,
      x => WindowsEncoding::Other(x),
    }
  }
}
