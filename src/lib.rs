mod melt;

mod wasm_host {
  use crate::melt;
  use serde_cbor::to_vec;

  pub use wasm_minimal_protocol::*;

  initiate_protocol!();

  #[wasm_func]
  pub fn fonts_collection_info(data: &[u8]) -> Vec<u8> {
    to_vec(&melt::FontInfo::from_collections(data).collect::<Vec<_>>())
      .unwrap()
  }
}
