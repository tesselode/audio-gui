use super::vector::Vector;

#[derive(Debug)]
pub struct Rect {
	pub position: Vector,
	pub size: Vector,
}

impl Rect {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			position: Vector::new(x, y),
			size: Vector::new(width, height),
		}
	}
}

impl Default for Rect {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0, 0.0)
	}
}
