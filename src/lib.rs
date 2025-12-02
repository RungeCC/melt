mod melt;

mod wasm_host {
  use crate::melt;
  use serde_cbor::{from_reader, to_vec};

  pub use wasm_minimal_protocol::*;

  initiate_protocol!();

  #[wasm_func]
  pub fn fonts_collection_info(data: &[u8]) -> Vec<u8> {
    to_vec(&melt::FontInfo::from_collections(data).collect::<Vec<_>>())
      .unwrap()
  }

  #[wasm_func]
  pub fn glyphes_infos(
    data: &[u8],
    index: &[u8],
    codepoints: &[u8],
  ) -> Vec<u8> {
    if let Ok(index) = from_reader::<u32, _>(index)
      && let Ok(codepoints) =
        from_reader::<Vec<u32>, _>(codepoints).map(|codes| {
          codes
            .into_iter()
            .map(std::char::from_u32)
            .collect::<Vec<Option<char>>>()
        })
    {
      to_vec(&melt::glyphes_infos(
        data,
        index,
        codepoints.clone().into_iter(),
      ))
      .unwrap()
    } else {
      let result: Vec<Option<char>> = Vec::new();
      to_vec(&result).unwrap()
    }
  }

  #[wasm_func]
  #[allow(dead_code)]
  pub fn glyphes_shapes(
    data: &[u8],
    index: &[u8],
    styles: &[u8],
    codepoints: &[u8],
  ) -> Vec<u8> {
    if let Ok(index) = from_reader::<u32, _>(index)
      && let Ok(codepoints) =
        from_reader::<Vec<u32>, _>(codepoints).map(|codes| {
          codes
            .into_iter()
            .map(std::char::from_u32)
            .collect::<Vec<Option<char>>>()
        })
    {
      let styles = from_reader::<melt::SvgPathStyles, _>(styles)
        .unwrap_or(melt::SvgPathStyles::default());
      to_vec(&melt::glyph_shapes(
        data,
        index,
        styles,
        codepoints.clone().into_iter(),
      ))
      .unwrap()
    } else {
      let result: Vec<Option<char>> = Vec::new();
      to_vec(&result).unwrap()
    }
  }
}
