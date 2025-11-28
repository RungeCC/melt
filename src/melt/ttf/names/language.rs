use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};

/// Language id for Macintosh platform.
///
/// From: <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html>
#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacintoshLanguage {
  English = 0,
  French = 1,
  German = 2,
  Italian = 3,
  Dutch = 4,
  Swedish = 5,
  Spanish = 6,
  Danish = 7,
  Portuguese = 8,
  Norwegian = 9,
  Hebrew = 10,
  Japanese = 11,
  Arabic = 12,
  Finnish = 13,
  Greek = 14,
  Icelandic = 15,
  Maltese = 16,
  Turkish = 17,
  Croatian = 18,
  ChineseTraditional = 19,
  Urdu = 20,
  Hindi = 21,
  Thai = 22,
  Korean = 23,
  Lithuanian = 24,
  Polish = 25,
  Hungarian = 26,
  Estonian = 27,
  Latvian = 28,
  Sami = 29,
  Faroese = 30,
  FarsiPersian = 31,
  Russian = 32,
  ChineseSimplified = 33,
  Flemish = 34,
  IrishGaelic = 35,
  Albanian = 36,
  Romanian = 37,
  Czech = 38,
  Slovak = 39,
  Slovenian = 40,
  Yiddish = 41,
  Serbian = 42,
  Macedonian = 43,
  Bulgarian = 44,
  Ukrainian = 45,
  Byelorussian = 46,
  Uzbek = 47,
  Kazakh = 48,
  AzerbaijaniCyrillic = 49,
  AzerbaijaniArabic = 50,
  Armenian = 51,
  Georgian = 52,
  Moldavian = 53,
  Kirghiz = 54,
  Tajiki = 55,
  Turkmen = 56,
  MongolianMongolianScript = 57,
  MongolianCyrillic = 58,
  Pashto = 59,
  Kurdish = 60,
  Kashmiri = 61,
  Sindhi = 62,
  Tibetan = 63,
  Nepali = 64,
  Sanskrit = 65,
  Marathi = 66,
  Bengali = 67,
  Assamese = 68,
  Gujarati = 69,
  Punjabi = 70,
  Oriya = 71,
  Malayalam = 72,
  Kannada = 73,
  Tamil = 74,
  Telugu = 75,
  Sinhalese = 76,
  Burmese = 77,
  Khmer = 78,
  Lao = 79,
  Vietnamese = 80,
  Indonesian = 81,
  Tagalog = 82,
  MalayRomanScript = 83,
  MalayArabicScript = 84,
  Amharic = 85,
  Tigrinya = 86,
  Galla = 87,
  Somali = 88,
  Swahili = 89,
  KinyarwandaRuanda = 90,
  Rundi = 91,
  NyanjaChewa = 92,
  Malagasy = 93,
  Esperanto = 94,
  // Gap in IDs from 95 to 127
  Welsh = 128,
  Basque = 129,
  Catalan = 130,
  Latin = 131,
  Quechua = 132,
  Guarani = 133,
  Aymara = 134,
  Tatar = 135,
  Uighur = 136,
  Dzongkha = 137,
  JavaneseRomanScript = 138,
  SundaneseRomanScript = 139,
  Galician = 140,
  Afrikaans = 141,
  Breton = 142,
  Inuktitut = 143,
  ScottishGaelic = 144,
  ManxGaelic = 145,
  IrishGaelicDotAbove = 146,
  Tongan = 147,
  GreekPolytonic = 148,
  Greenlandic = 149,
  AzerbaijaniRoman = 150,
}

impl MacintoshLanguage {
  #[allow(clippy::too_many_lines)]
  pub fn name(self) -> &'static str {
    match self {
      Self::English => "English",
      Self::French => "French",
      Self::German => "German",
      Self::Italian => "Italian",
      Self::Dutch => "Dutch",
      Self::Swedish => "Swedish",
      Self::Spanish => "Spanish",
      Self::Danish => "Danish",
      Self::Portuguese => "Portuguese",
      Self::Norwegian => "Norwegian",
      Self::Hebrew => "Hebrew",
      Self::Japanese => "Japanese",
      Self::Arabic => "Arabic",
      Self::Finnish => "Finnish",
      Self::Greek => "Greek",
      Self::Icelandic => "Icelandic",
      Self::Maltese => "Maltese",
      Self::Turkish => "Turkish",
      Self::Croatian => "Croatian",
      Self::ChineseTraditional => "Chinese (traditional)",
      Self::Urdu => "Urdu",
      Self::Hindi => "Hindi",
      Self::Thai => "Thai",
      Self::Korean => "Korean",
      Self::Lithuanian => "Lithuanian",
      Self::Polish => "Polish",
      Self::Hungarian => "Hungarian",
      Self::Estonian => "Estonian",
      Self::Latvian => "Latvian",
      Self::Sami => "Sami",
      Self::Faroese => "Faroese",
      Self::FarsiPersian => "Farsi/Persian",
      Self::Russian => "Russian",
      Self::ChineseSimplified => "Chinese (simplified)",
      Self::Flemish => "Flemish",
      Self::IrishGaelic => "Irish Gaelic",
      Self::Albanian => "Albanian",
      Self::Romanian => "Romanian",
      Self::Czech => "Czech",
      Self::Slovak => "Slovak",
      Self::Slovenian => "Slovenian",
      Self::Yiddish => "Yiddish",
      Self::Serbian => "Serbian",
      Self::Macedonian => "Macedonian",
      Self::Bulgarian => "Bulgarian",
      Self::Ukrainian => "Ukrainian",
      Self::Byelorussian => "Byelorussian",
      Self::Uzbek => "Uzbek",
      Self::Kazakh => "Kazakh",
      Self::AzerbaijaniCyrillic => "Azerbaijani (Cyrillic script)",
      Self::AzerbaijaniArabic => "Azerbaijani (Arabic script)",
      Self::Armenian => "Armenian",
      Self::Georgian => "Georgian",
      Self::Moldavian => "Moldavian",
      Self::Kirghiz => "Kirghiz",
      Self::Tajiki => "Tajiki",
      Self::Turkmen => "Turkmen",
      Self::MongolianMongolianScript => "Mongolian (Mongolian script)",
      Self::MongolianCyrillic => "Mongolian (Cyrillic script)",
      Self::Pashto => "Pashto",
      Self::Kurdish => "Kurdish",
      Self::Kashmiri => "Kashmiri",
      Self::Sindhi => "Sindhi",
      Self::Tibetan => "Tibetan",
      Self::Nepali => "Nepali",
      Self::Sanskrit => "Sanskrit",
      Self::Marathi => "Marathi",
      Self::Bengali => "Bengali",
      Self::Assamese => "Assamese",
      Self::Gujarati => "Gujarati",
      Self::Punjabi => "Punjabi",
      Self::Oriya => "Oriya",
      Self::Malayalam => "Malayalam",
      Self::Kannada => "Kannada",
      Self::Tamil => "Tamil",
      Self::Telugu => "Telugu",
      Self::Sinhalese => "Sinhalese",
      Self::Burmese => "Burmese",
      Self::Khmer => "Khmer",
      Self::Lao => "Lao",
      Self::Vietnamese => "Vietnamese",
      Self::Indonesian => "Indonesian",
      Self::Tagalog => "Tagalog",
      Self::MalayRomanScript => "Malay (Roman script)",
      Self::MalayArabicScript => "Malay (Arabic script)",
      Self::Amharic => "Amharic",
      Self::Tigrinya => "Tigrinya",
      Self::Galla => "Galla",
      Self::Somali => "Somali",
      Self::Swahili => "Swahili",
      Self::KinyarwandaRuanda => "Kinyarwanda/Ruanda",
      Self::Rundi => "Rundi",
      Self::NyanjaChewa => "Nyanja/Chewa",
      Self::Malagasy => "Malagasy",
      Self::Esperanto => "Esperanto",
      Self::Welsh => "Welsh",
      Self::Basque => "Basque",
      Self::Catalan => "Catalan",
      Self::Latin => "Latin",
      Self::Quechua => "Quechua",
      Self::Guarani => "Guarani",
      Self::Aymara => "Aymara",
      Self::Tatar => "Tatar",
      Self::Uighur => "Uighur",
      Self::Dzongkha => "Dzongkha",
      Self::JavaneseRomanScript => "Javanese (Roman script)",
      Self::SundaneseRomanScript => "Sundanese (Roman script)",
      Self::Galician => "Galician",
      Self::Afrikaans => "Afrikaans",
      Self::Breton => "Breton",
      Self::Inuktitut => "Inuktitut",
      Self::ScottishGaelic => "Scottish Gaelic",
      Self::ManxGaelic => "Manx Gaelic",
      Self::IrishGaelicDotAbove => "Irish Gaelic (with dot above)",
      Self::Tongan => "Tongan",
      Self::GreekPolytonic => "Greek (polytonic)",
      Self::Greenlandic => "Greenlandic",
      Self::AzerbaijaniRoman => "Azerbaijani (Roman script)",
    }
  }
}

impl Display for MacintoshLanguage {
  /// Returns the original string representation from the specification.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl MacintoshLanguage {
  /// Safety: we know that the index is valid in range
  /// [0, 95) + [128, 151)
  pub fn from_index(index: u16) -> Option<Self> {
    match index {
      0..95 | 128..151 => {
        Some(unsafe { std::mem::transmute::<u16, MacintoshLanguage>(index) })
      }
      _ => None,
    }
  }
}

impl Serialize for MacintoshLanguage {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.name())
  }
}

struct MacintoshLanguageVisitor;

impl Visitor<'_> for MacintoshLanguageVisitor {
  type Value = MacintoshLanguage;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a valid Macintosh Language ID (u16)")
  }

  fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
  where
    E: Error,
  {
    MacintoshLanguage::from_index(value).ok_or_else(|| {
      E::custom(format!("Unknown MacintoshLanguage ID: {value}"))
    })
  }
}

impl<'de> Deserialize<'de> for MacintoshLanguage {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_u16(MacintoshLanguageVisitor)
  }
}
