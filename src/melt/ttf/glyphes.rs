use crate::melt::repr::FontRepr;
use crate::melt::ttf::metrics::FontMetrics;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::string::ToString;
use ttf_parser::{GlyphId, OutlineBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T> {
  x: T,
  y: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhantomPoints<T> {
  left: Point<T>,
  right: Point<T>,
  top: Point<T>,
  bottom: Point<T>,
}

impl From<ttf_parser::PhantomPoints> for PhantomPoints<f32> {
  fn from(points: ttf_parser::PhantomPoints) -> Self {
    Self {
      left: Point {
        x: points.left.x,
        y: points.left.y,
      },
      right: Point {
        x: points.right.x,
        y: points.right.y,
      },
      top: Point {
        x: points.top.x,
        y: points.top.y,
      },
      bottom: Point {
        x: points.bottom.x,
        y: points.bottom.y,
      },
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BBox<T> {
  pub x_min: T,
  pub y_min: T,
  pub x_max: T,
  pub y_max: T,
}

impl From<ttf_parser::Rect> for BBox<i16> {
  fn from(rect: ttf_parser::Rect) -> Self {
    Self {
      x_min: rect.x_min,
      y_min: rect.y_min,
      x_max: rect.x_max,
      y_max: rect.y_max,
    }
  }
}

impl From<ttf_parser::RectF> for BBox<f32> {
  fn from(rect: ttf_parser::RectF) -> Self {
    Self {
      x_min: rect.x_min,
      y_min: rect.y_min,
      x_max: rect.x_max,
      y_max: rect.y_max,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphInfo {
  id: u16,
  pub(crate) name: Option<String>,
  bbox: Option<BBox<i16>>,
  phantom_points: Option<PhantomPoints<f32>>,
  pub(crate) y_origin: Option<i16>,
  pub(crate) vertical_advance: Option<u16>,
  pub(crate) horizontal_advance: Option<u16>,
  pub(crate) vertical_side_bearing: Option<i16>,
  pub(crate) horizontal_side_bearing: Option<i16>,

  is_color: bool,
}

impl GlyphInfo {
  pub(crate) fn id(&self) -> GlyphId {
    GlyphId(self.id)
  }

  fn from_glyph_id(repr: &FontRepr, glyph_id: ttf_parser::GlyphId) -> Self {
    let ttf = &repr.ttf_parser;
    let name = ttf.glyph_name(glyph_id).map(ToString::to_string);
    let bbox = ttf.glyph_bounding_box(glyph_id).map(BBox::from);
    let y_origin = ttf.glyph_y_origin(glyph_id);
    let vertical_advance = ttf.glyph_ver_advance(glyph_id);
    let horizontal_advance = ttf.glyph_ver_advance(glyph_id);
    let vertical_side_bearing = ttf.glyph_ver_side_bearing(glyph_id);
    let horizontal_side_bearing = ttf.glyph_hor_side_bearing(glyph_id);
    let phantom_points =
      ttf.glyph_phantom_points(glyph_id).map(PhantomPoints::from);

    let is_color = ttf.is_color_glyph(glyph_id);
    Self {
      id: glyph_id.0,
      name,
      bbox,
      y_origin,
      vertical_advance,
      horizontal_advance,
      vertical_side_bearing,
      horizontal_side_bearing,
      phantom_points,

      is_color,
    }
  }

  pub(crate) fn from_character(repr: &FontRepr, ch: char) -> Option<Self> {
    let ttf = &repr.ttf_parser;
    let glyph_id = ttf.glyph_index(ch)?;
    Some(Self::from_glyph_id(repr, glyph_id))
  }

  #[allow(dead_code)]
  pub(crate) fn from_variantion(
    repr: &FontRepr,
    base_ch: char,
    variation_selector: char,
  ) -> Option<Self> {
    let ttf = &repr.ttf_parser;
    let glyph_id = ttf.glyph_variation_index(base_ch, variation_selector)?;
    Some(Self::from_glyph_id(repr, glyph_id))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphInfos(Vec<Option<GlyphInfo>>);

impl GlyphInfos {
  pub(crate) fn from_option_iter(
    repr: &FontRepr,
    codes: impl Iterator<Item = Option<char>>,
  ) -> Self {
    let glyphes = codes.map(|code| GlyphInfo::from_character(repr, code?));
    GlyphInfos(glyphes.collect())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphShape {
  svg: String,
}

struct SvgPen(String);

impl SvgPen {
  fn new() -> Self {
    Self(String::new())
  }
}

// Implement the trait to translate Font commands -> SVG commands
impl OutlineBuilder for SvgPen {
  fn move_to(&mut self, x: f32, y: f32) {
    write!(&mut self.0, "M {x} {y} ").unwrap();
  }

  fn line_to(&mut self, x: f32, y: f32) {
    write!(&mut self.0, "L {x} {y} ").unwrap();
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    // Quadratic Bezier
    write!(&mut self.0, "Q {x1} {y1} {x} {y} ").unwrap();
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    // Cubic Bezier
    write!(&mut self.0, "C {x1} {y1} {x2} {y2} {x} {y} ").unwrap();
  }

  fn close(&mut self) {
    write!(&mut self.0, "Z ").unwrap();
  }
}

impl GlyphShape {
  fn from_glyph(repr: &FontRepr, glyph: &GlyphInfo) -> Option<Self> {
    let mut builder = SvgPen::new();
    let metrics = FontMetrics::from_repr(repr);
    let ttf = &repr.ttf_parser;
    let glyph_id = glyph.id();
    let ascender = metrics.ascender;
    let descender = metrics.descender;
    let height = ascender - descender;
    #[allow(unused_variables)]
    let width = glyph.horizontal_advance.unwrap_or(1000);
    #[allow(unused_variables)]
    let side_bearing = glyph.horizontal_side_bearing.unwrap_or(0);
    #[allow(unused_variables)]
    let bbox = ttf.outline_glyph(glyph_id, &mut builder)?;
    #[rustfmt::skip]
    let svg = format!(
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <g transform="matrix(1 0 0 -1 0 {ascender})">
    <path d="{path_data}" fill="black" />
  </g>
</svg>"#,
      width = bbox.width() + 2 * side_bearing,
      height = height,
      ascender = ascender,
      path_data = builder.0
    );
    Some(GlyphShape { svg })
  }

  pub(crate) fn from_character(repr: &FontRepr, ch: char) -> Option<Self> {
    let glyph = GlyphInfo::from_character(repr, ch)?;
    GlyphShape::from_glyph(repr, &glyph)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphShapes(Vec<Option<GlyphShape>>);

impl GlyphShapes {
  pub(crate) fn from_option_iter(
    repr: &FontRepr,
    codes: impl Iterator<Item = Option<char>>,
  ) -> Self {
    let glyph_shapes =
      codes.map(|code| GlyphShape::from_character(repr, code?));
    GlyphShapes(glyph_shapes.collect())
  }
}
