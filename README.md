# melt

**WARNING: This package is still under development. APIs are unstable and may change without notice.**

A [Typst](https://typst.app/) package for introspecting fonts.

`melt` provides low-level access to font information by combining Typst's internal font data with the `ttf-parser` library through a WebAssembly plugin. It allows you to extract detailed information about fonts directly within your Typst documents.

The name "melt" is a nod to traditional metal typesetting, where font characters (types) were cast from molten metal.

## Usage

First, add the package to your project:

```typst
#import "@preview/melt:0.2.0": *
```

Then, you can use the provided functions to inspect your font files. For example, to get information about a font:

```typst
// Read the font file as raw bytes.
#let font-data = read("your-font.ttf", encoding: none)

// Get information about the first font in the file.
#let info = font-info(font-data)

// Now you can access the font's properties.
#info.properties.names.full-name
#info.metrics.x-height

// Check if the font contains a specific character.
#contains(info, "x".to-unicode())
```

## Example: Fake Bold

Once you have parsed the fonts used in your document, you can perform more advanced operations. This example demonstrates how to create a "fake bold" effect that adjusts its spacing based on whether the font is monospace.

```typ
#import "@preview/melt:0.2.0": *

// Assume the following fonts exist in an 'assets' folder.
#let monaco-parsed = font-info(read("assets/Monaco.ttf", encoding: none))
#let sourcehansans-parsed = fonts-collection-info(read("assets/SourceHanSans.ttc", encoding: none))

// Create a dictionary to easily look up parsed font info by family name.
#let parsed-fonts = (
  (monaco-parsed, ..sourcehansans-parsed)
    .map(
      it => (lower(it.typst.info.family), it),
    )
    .to-dict()
)

// A function to create a "fake bold" effect by adding a stroke.
// It intelligently adjusts tracking for non-monospace fonts.
// Original idea: https://github.com/typst/typst/issues/2157#issuecomment-1635393083
#let fakebold(txt, stroke: 1) = {
  context {
    let font-info = parsed-fonts.at(text.font, default: none)
    let is-mono = if font-info != none { font-info.typst.info.is_monospace } else { false }

    text(
      tracking: if is-mono { 0em } else { stroke * 0.001em },
      stroke: (stroke * 0.001em) + text.fill,
      txt,
    )
  }
}

#set text(font: "Source Han Sans", lang: "cn")

#fakebold("Hello, World!", stroke: 20) \
#fakebold("Hello, World!", stroke: 50)

#set text(font: "Monaco")

#fakebold("Hello, World!", stroke: 20)\
#fakebold("Hello, World!", stroke: 50)
```

## API Reference

### `fonts-collection-info`

Parses a font file (or a font collection) and returns an array of dictionaries, with each dictionary containing information about a single font.

- `data`: `bytes` — The raw data of the font file.
- **Returns**: `array` of font information dictionaries. See `font-info` for the structure of each dictionary.
- Its signature could be explained as follows:

```rust
fn fonts_collection_info(data: &[u8]) -> Vec<Option<FontInfo>>
```

### `font-info`

A convenience function to get information about a single font. It is especially useful for font files that contain only one font.

- `data`: `bytes` — The raw data of the font file.
- `index`: `int` (optional, default: `0`) — The index of the font to inspect in a font collection.
- **Returns**: `dictionary` containing the font information with the following keys:
    - `properties`: A dictionary with the font's names, scripts, and features.
        - `names`: Contains various name strings from the font's `name` table (e.g., `family`, `full-name`, `postscript-name`). _Note: These may differ from what Typst uses. See `typst.info.family` for the name recognized by Typst._ All possible entries can be found [here](https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
          - Each `name` item is an array of every possible entries matching `name_id`, each entry has a `name`, a `language` and `platform_encoding` field.
        - `scripts`: A list of supported script and language tags from the font's `GSUB` and `GPOS` tables.
          _Note: This might not be the list of the font's intended scripts and languages._
          - It also contains `supported` and `designed` fields, from font's `meta` table, may reflect the font's intended scripts and languages, see [here](https://learn.microsoft.com/en-us/typography/opentype/spec/meta#data-maps) for details.
        - `features`: A list of supported OpenType feature tags.
    - `metrics`: A dictionary with various font metrics.
        - `italic_angle` is in degrees, all rest metrics are in font units.
    - `typst`: A dictionary containing font information and font metrics as seen by Typst's engine. 
        - `info`: This mirrors the Typst's internal `FontInfo` structure, with flags converted to booleans for convenience.
            - `coverage`: Typst's internal representation of Unicode coverage. Use this with the `contains` function to check for character support.
        - `metrics`: A dictionary containing font metrics as seen by Typst's engine. _Note: that the math metrics has not been included yet._ 

Its signature could be explained as follows:
```rust
fn font_info(
  data: &[u8],
  index: u32,
) -> Option<FontInfo>

struct FontInfo {
  properties: FontProperties,
  metrics: FontMetrics,
  typst: TypstFontIntrospection,
}

struct FontProperties {
  names: FontNames,
  scripts: FontScripts,
  features: Set<String>,
}

struct FontNames {
  copyright_notice: Vec<FontName>,
  family: Vec<FontName>,
  ... // for details, see opentype reference of `name_id`s.
}

struct FontName {
  name: Option<String>,
  language: Option<String>,
  platform_encoding: PlatformEncoding,
}

struct Scripts {
  scripts: Set<String>,
  languages: Set<String>,
  designed: Set<String>,
  supported: Set<String>,
}

struct FontMetrics {
  em: u16,
  ascender: i16,
  descender: i16,
  line_gap: i16,
  height: i16,
  italic_angle: f32,
}

struct TypstFontIntrospection {
  info: TypstFontInfo,
  metrics: TypstFontMetrics
}

struct TypstFontInfo {
  family: String,
  variant: FontVariant,
  coverage: Coverage,
  is_monospace: bool,
  is_serif: bool,
  is_variable: bool,
  has_math_table: bool,
}

struct TypstFontMetrics {
  units_per_em: int,
  ascender: f64,
  descender: f64,
  x_height: f64,
  cap_height: f64,

  strikethrough: LineMetrics,
  overline: LineMetrics,
  underline: LineMetrics,
  subscript: Option<ScriptMetrics>,
  superscript: Option<ScriptMetrics>,
}
```

### `contains`

Checks if a given codepoint is present in the font's coverage data.

- `parsed-data`: `dictionary` — The font information dictionary from `font-info` or `fonts-collection-info`.
- `codepoint`: `int` — The Unicode codepoint to check.
- **Returns**: `bool` indicating whether the codepoint is covered by the font.

Its signature could be explained as follows:
```rust
fn contains(
  data: &[u8],
  index: u32,
  codepoint: char,
) -> bool
```

### `glyphes-infos`

`glyphes_info` provides detailed glyph information including (all `metrics` are in font `unit`, relation between `em` and `unit` is through `typst.metrics.units_per_em`)
  - Glyph ID
  - name
  - `metric`: Bounding box
  - `metric`: Horizontal and vertical advances
  - `metric`: Side bearings
  - `metric`: Phantom points
  - Color glyph detection


Its signature could be explained as follows:

```rust
fn glyphes_infos(
  data: &[u8],
  index: u32,
) -> Vec<Option<GlyphInfo>>

struct GlyphInfo {
  id: u16,
  name: Option<String>,
  bbox: Option<BBox<i16>>,
  phantom_points: Option<PhantomPoints<f32>>,
  y_origin: Option<i16>,
  vertical_advance: Option<u16>,
  horizontal_advance: Option<u16>,
  vertical_side_bearing: Option<i16>,
  horizontal_side_bearing: Option<i16>,
  is_color: bool,
}
```

## Known Limitations

- Due to Typst's security model, this package cannot access system-installed fonts. You must provide the font file directly by reading it from a local path.

## Build From Source

Require that rust toolchain and `nushell` are installed.

Clone this project:

```sh
git clone https://github.com/RungeCC/melt.git
```

cd into it, then run:

```nu
nu scripts/main.nu release --yes
```

It will release the Typst package `melt` into `release/` directory.

For more details, see `scripts/main.nu`.

## TODOs

- [ ] Completely move from `ttf-parser` to `skrifa`.
- [ ] Support `glyphes-shapes` function.
- [ ] doc.rs
- [ ] Typst documentation.
- [ ] Complete mod `metrics`, support more entries
- [ ] Bundle some fonts, especially fonts that are also bundled by Typst compiler.
- [ ] Use ICU.

## License

This project is licensed under the MIT License.
