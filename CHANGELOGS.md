# CHANGELOGS

## 0.2.0

### Lib Side Changes

- New `wasm_func`s `glyphes_infos` to retrieve glyph information for a list of Unicode codepoints.

#### Internal `FontInfo` Changes

- v0.1.0 `metrics` becomes `typst.metrics`, v0.1.0 `typst` becomes `typst.info`
- new entry `metrics` of `info`
- `typst.metrics` no more contains `italic_angle` field.
- `properties.names` contains all possible `name_id`s, and return all possible entries with corresponding `name_id`, each entry has a `language` and `platform_encoding` field.
- `properties.scripts` now contains `designed` and `supported` fields, as arrays of strings, from font's `meta` table.

### Package Side Changes

- New function `glyphes-info(data, index, codepoints)` to retrieve glyph information for a list of Unicode codepoints.
- Now `contains` function will check if the codepoint is a valid Unicode codepoint.

### Misc

- Publish script is renamed to `main.nu`
- Subcommand `build` is renamed to `release`, default release path is `release/`
- New subcommand `clean` for cleaning release artifacts
- Flag `--build (-b)` of subcommand `publish` is renamed to `--release (-r)`.
