use super::geometry::{Point, Rectangle};

/// An RGBA color.
#[derive(Copy, Clone)]
pub struct Color {
	/// The red component of the color.
	pub red: f32,
	/// The green component of the color.
	pub green: f32,
	/// The blue component of the color.
	pub blue: f32,
	/// The alpha (transparency) component of the color.
	pub alpha: f32,
}

impl Color {
	/// Creates a new color.
	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}
}

/// How to draw a shape.
#[derive(Copy, Clone)]
pub enum DrawMode {
	/// Draws a shape with a solid color.
	Fill,
	/// Draws a shape as an outline with the given width.
	Stroke(f32),
}

/// The visual properties of a shape.
#[derive(Copy, Clone)]
pub struct Style {
	/// How the shape is drawn.
	pub mode: DrawMode,
	/// The color of the shape.
	pub color: Color,
}

/// The kinds of arcs that can be drawn.
///
/// The arc kinds correspond to the [ArcTypes in LÖVE](https://love2d.org/wiki/ArcType).
pub enum ArcKind {
	Pie,
	Open,
	Closed,
}

/// How an object is aligned on an axis.
pub enum Alignment {
	/// The start of the object is placed at the given position.
	Start,
	/// The center of the object is placed at the given position.
	Middle,
	/// The end of the object is placed at the given position.
	End,
	/// An arbitrary point on the object (from 0-1) is placed at the given position.
	Custom(f32),
}

impl Alignment {
	/// Gets the anchor point (from 0-1) of the alignment.
	pub fn as_f32(&self) -> f32 {
		match self {
			Alignment::Start => 0.0,
			Alignment::Middle => 0.5,
			Alignment::End => 1.0,
			Alignment::Custom(align) => *align,
		}
	}
}

/// The visual properties of a piece of text.
pub struct TextStyle {
	/// The index of the font that should be used.
	pub font_id: usize,
	/// The size of the font.
	pub size: f32,
	/// The alignment of the text on the x-axis.
	pub horizontal_alignment: Alignment,
	/// The alignment of the text on the y-axis.
	pub vertical_alignment: Alignment,
	/// The color of the text.
	pub color: Color,
}

/// Represents a drawing task for the backend to complete.
pub enum DrawOperation {
	/// A task to draw a rectangle with the given bounds and style.
	Rectangle(Rectangle, Style),
	/// A task to draw a circle with the given center, radius, and style.
	Circle(Point, f32, Style),
	/// A task to draw an arc with the given `ArcKind`, center, radius,
	/// start angle, end angle, and style.
	Arc(ArcKind, Point, f32, f32, f32, Style),
	/// A task to draw a series of lines connecting the given points
	/// with the given style.
	Polyline(Vec<Point>, Style),
	/// A task to draw a polygon made of the given points
	/// with the given style.
	Polygon(Vec<Point>, Style),
	/// A task to draw text with the given string, position, and style.
	Text(String, Point, TextStyle),
}

/// A surface that a `ControlBehavior` can draw to.
///
/// Note that, when a drawing function on a `Canvas` is called,
/// nothing is actually rendered to the screen. Rather, a `DrawingOperation`
/// is stored. The backend should read the list of `DrawingOperation`s and
/// do the actual rendering.
pub struct Canvas {
	/// The list of drawing tasks that the backend should complete.
	pub operations: Vec<DrawOperation>,
}

impl Canvas {
	/// Creates a new `Canvas`.
	pub fn new() -> Self {
		Self { operations: vec![] }
	}

	/// Draws a rectangle.
	pub fn draw_rectangle(&mut self, rectangle: Rectangle, style: Style) {
		self.operations
			.push(DrawOperation::Rectangle(rectangle, style));
	}

	/// Draws a circle.
	pub fn draw_circle(&mut self, position: Point, radius: f32, style: Style) {
		self.operations
			.push(DrawOperation::Circle(position, radius, style));
	}

	/// Draws an arc.
	pub fn draw_arc(
		&mut self,
		kind: ArcKind,
		position: Point,
		radius: f32,
		angle1: f32,
		angle2: f32,
		style: Style,
	) {
		self.operations.push(DrawOperation::Arc(
			kind, position, radius, angle1, angle2, style,
		));
	}

	/// Draws a series of lines connecting the given points.
	pub fn draw_polyline(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polyline(points, style));
	}

	/// Draws a polygon.
	pub fn draw_polygon(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polygon(points, style));
	}

	/// Draws text.
	pub fn draw_text(&mut self, text: String, position: Point, style: TextStyle) {
		self.operations
			.push(DrawOperation::Text(text, position, style));
	}
}
