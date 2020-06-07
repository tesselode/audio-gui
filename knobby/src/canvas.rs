use super::geometry::{Point, Rectangle};

#[derive(Copy, Clone)]
pub struct Color {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

impl Color {
	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}
}

#[derive(Copy, Clone)]
pub enum DrawMode {
	Fill,
	Stroke(f32),
}

#[derive(Copy, Clone)]
pub struct Style {
	pub mode: DrawMode,
	pub color: Color,
}

pub enum ArcKind {
	Pie,
	Open,
	Closed,
}

pub enum Alignment {
	Start,
	Middle,
	End,
	Custom(f32),
}

impl Alignment {
	pub fn as_f32(&self) -> f32 {
		match self {
			Alignment::Start => 0.0,
			Alignment::Middle => 0.5,
			Alignment::End => 1.0,
			Alignment::Custom(align) => *align,
		}
	}
}

pub struct TextStyle {
	pub font_id: usize,
	pub size: f32,
	pub horizontal_alignment: Alignment,
	pub vertical_alignment: Alignment,
	pub color: Color,
}

pub enum DrawOperation {
	Rectangle(Rectangle, Style),
	Circle(Point, f32, Style),
	Arc(ArcKind, Point, f32, f32, f32, Style),
	Polyline(Vec<Point>, Style),
	Polygon(Vec<Point>, Style),
	Text(String, Point, TextStyle),
}

pub struct Canvas {
	pub operations: Vec<DrawOperation>,
}

impl Canvas {
	pub fn new() -> Self {
		Self { operations: vec![] }
	}

	pub fn draw_rectangle(&mut self, rectangle: Rectangle, style: Style) {
		self.operations
			.push(DrawOperation::Rectangle(rectangle, style));
	}

	pub fn draw_circle(&mut self, position: Point, radius: f32, style: Style) {
		self.operations
			.push(DrawOperation::Circle(position, radius, style));
	}

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

	pub fn draw_polyline(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polyline(points, style));
	}

	pub fn draw_polygon(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polygon(points, style));
	}

	pub fn draw_text(&mut self, text: String, position: Point, style: TextStyle) {
		self.operations
			.push(DrawOperation::Text(text, position, style));
	}
}
