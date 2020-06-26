use crate::geometry::{rect::Rect, vector::Vector};

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
	pub translation_stack: Vec<Vector>,
}

impl Canvas {
	pub fn new() -> Self {
		Self {
			operations: vec![],
			translation_stack: vec![],
		}
	}

	fn get_current_translation(&self) -> Vector {
		*self
			.translation_stack
			.last()
			.unwrap_or(&Vector::new(0.0, 0.0))
	}

	pub fn push_translation(&mut self, translation: Vector) {
		self.translation_stack
			.push(self.get_current_translation() + translation);
	}

	pub fn pop_translation(&mut self) {
		self.translation_stack.pop();
	}

	pub fn draw_rectangle(&mut self, rect: Rect, style: ShapeStyle) {
		self.operations.push(DrawOperation::DrawRectangle(
			rect.shifted(self.get_current_translation()),
			style,
		));
	}
}
