use crate::melt::repr::FontRepr;
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

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct SvgPathStyles {
  scaling: f32,
}

impl Default for SvgPathStyles {
  fn default() -> Self {
    Self { scaling: 1.0 }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct GlyphShapeBuilder {
  style: SvgPathStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphShape {
  // Current only SVG.
  svg: String,
}

impl GlyphShapeBuilder {
  #[allow(dead_code)]
  fn new(style: SvgPathStyles) -> Self {
    Self { style }
  }

  #[allow(dead_code)]
  fn with_style(mut self, style: SvgPathStyles) -> Self {
    self.style = style;
    self
  }
}

impl GlyphShapeBuilder {
  fn glyph_shape(
    &self,
    repr: &FontRepr,
    glyph: &GlyphInfo,
  ) -> Option<GlyphShape> {
    let mut builder = SvgPen::new();
    // px -> pt conversion, 1pt = 1.33px, then apply additional scaling
    let scale = 1. / 0.75 * self.style.scaling;
    let ttf = &repr.ttf_parser;
    let glyph_id = glyph.id();
    let bbox = ttf.outline_glyph(glyph_id, &mut builder)?;
    let width = glyph.vertical_advance.map_or(
      f32::from(bbox.width())
        + 2.0 * f32::from(glyph.horizontal_side_bearing.unwrap_or(0)),
      |e| f32::from(e) * scale,
    ) * scale;
    let height = f32::from(bbox.height()) * scale;
    let y_origin = f32::from(-bbox.y_max) * scale;
    #[rustfmt::skip]
    let svg = format!(
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 {y_origin} {width} {height}">
  <g transform="scale({scale}, -{scale})">
    <path d="{path_data}" fill="black" />
  </g>
</svg>"#,
      path_data = builder.0
    );
    Some(GlyphShape { svg })
  }
}

impl GlyphShape {
  pub(crate) fn from_character_styled(
    repr: &FontRepr,
    styles: SvgPathStyles,
    ch: char,
  ) -> Option<Self> {
    let glyph = GlyphInfo::from_character(repr, ch)?;
    GlyphShapeBuilder::new(styles).glyph_shape(repr, &glyph)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GlyphShapes(Vec<Option<GlyphShape>>);

impl GlyphShapes {
  pub(crate) fn from_option_iter_styled(
    repr: &FontRepr,
    styles: SvgPathStyles,
    codes: impl Iterator<Item = Option<char>>,
  ) -> Self {
    let glyph_shapes = codes
      .map(|code| GlyphShape::from_character_styled(repr, styles, code?));
    GlyphShapes(glyph_shapes.collect())
  }
}
