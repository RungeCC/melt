#let melt = plugin("./melt.wasm")

// NOTE: for accurately checking if the codepoint is an _assigned_ Unicode codepoint
// we need to query the Unicode database, which is not yet implemented
#let _is-valid-unicode(codepoint) = {
  (
    type(codepoint) == int
      and 0 <= codepoint
      and codepoint <= 0x10FFFF
      and not (0xD800 <= codepoint and codepoint <= 0xDFFF) // not a surrogate
  )
}

#let fonts-collection-info(data) = cbor(melt.fonts_collection_info(data))
#let font-info(data, index: 0) = fonts-collection-info(data).at(index)

#let contains(parsed-data, codepoint) = {
  assert(
    _is-valid-unicode(codepoint),
    message: "codepoint must be a valid Unicode codepoint.",
  )
  let inside = false
  let cursor = 0
  let coverage = parsed-data.typst.coverage

  for run in coverage {
    if cursor <= codepoint and codepoint < cursor + run {
      return inside
    }
    cursor += run
    inside = not inside
  }
  false
}

#let glyphes-infos(data, index, codepoints) = {
  assert(
    type(index) == int and 0 <= index and index < 0xFFFFFFFF,
    message: "index must be an integer between 0 and 2^32 - 1",
  )
  assert(
    type(codepoints) == array and codepoints.all(_is-valid-unicode),
    message: "codepoints must be an array of valid Unicode codepoints.",
  )
  cbor(melt.glyphes_infos(data, cbor.encode(index), cbor.encode(codepoints)))
}

// additionally, we require that scaling to be representable in f32
#let svg-path-styles(scaling: 1.0) = {
  assert(
    (type(scaling) == int or type(scaling) == float),
    message: "scaling must be numeric.",
  )
  (scaling: scaling * 1.0) // into float
}

#let glyphes-shapes(data, index, codepoints, styles: none) = {
  assert(
    type(index) == int and 0 <= index and index < 0xFFFFFFFF,
    message: "index must be an integer between 0 and 2^32 - 1",
  )
  assert(
    type(codepoints) == array and codepoints.all(_is-valid-unicode),
    message: "codepoints must be an array of valid Unicode codepoints.",
  )
  cbor(melt.glyphes_shapes(
    data,
    cbor.encode(index),
    cbor.encode(styles),
    cbor.encode(codepoints),
  ))
}
