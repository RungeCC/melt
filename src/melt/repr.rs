use ttf_parser::Face;
use typst_library::text;

pub(crate) struct FontRepr<'a> {
  pub(crate) ttf: Face<'a>,
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
    let info = text::FontInfo::new(data, index)?;
    let metrics = text::FontMetrics::from_ttf(&ttf);
    Some(FontRepr {
      ttf,
      info,
      metrics,
      data,
      index,
    })
  }
}
