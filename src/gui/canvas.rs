use super::{point::Point, rectangle::Rectangle};

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

pub enum DrawMode {
	Fill,
	Stroke(f32),
}

pub struct Style {
	pub mode: DrawMode,
	pub color: Color,
}

pub enum DrawOperation {
	Rectangle(Rectangle, Style),
	Circle(Point, f32, Style),
	Polyline(Vec<Point>, Style),
	Polygon(Vec<Point>, Style),
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

	pub fn draw_polyline(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polyline(points, style));
	}

	pub fn draw_polygon(&mut self, points: Vec<Point>, style: Style) {
		self.operations.push(DrawOperation::Polygon(points, style));
	}
}
