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

pub trait Display {
	fn draw_rectangle(&mut self, rectangle: Rectangle, style: Style);
	fn draw_circle(&mut self, position: Point, radius: f32, style: Style);
	fn draw_polyline(&mut self, points: Vec<Point>, style: Style);
	fn draw_polygon(&mut self, points: Vec<Point>, style: Style);
}
