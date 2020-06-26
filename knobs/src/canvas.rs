use crate::geometry::rect::Rect;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum ShapeStyle {
	Fill(Color),
	Stroke(f32, Color),
}

#[derive(Debug, Copy, Clone)]
pub enum DrawOperation {
	DrawRectangle(Rect, ShapeStyle),
}

pub struct Canvas {
	pub operations: Vec<DrawOperation>,
}

impl Canvas {
	pub fn new() -> Self {
		Self { operations: vec![] }
	}

	pub fn draw_rectangle(&mut self, rect: Rect, style: ShapeStyle) {
		self.operations
			.push(DrawOperation::DrawRectangle(rect, style));
	}
}
