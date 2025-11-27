use skrifa::FontRef;
use ttf_parser::Face;
use typst_library::text;

pub(crate) struct FontRepr<'a> {
  #[allow(dead_code)]
  pub(crate) ttf_parser: Face<'a>,
  #[allow(dead_code)]
  pub(crate) font_ref: FontRef<'a>,
  pub(crate) info: text::FontInfo,
  pub(crate) metrics: text::FontMetrics,
  #[allow(dead_code)]
  data: &'a [u8],
  #[allow(dead_code)]
  index: u32,
}

impl<'a> FontRepr<'a> {
  pub(crate) fn new(data: &'a [u8], index: u32) -> Option<Self> {
    let ttf = Face::parse(data, index).ok()?;
    // The below line shall never encounter additional error if
    // ttf parsing succeeds.
    let info = text::FontInfo::new(data, index).unwrap();
    let metrics = text::FontMetrics::from_ttf(&ttf);
    let font_ref = FontRef::from_index(data, index).ok()?;
    Some(FontRepr {
      ttf_parser: ttf,
      font_ref,
      info,
      metrics,
      data,
      index,
    })
  }
}
