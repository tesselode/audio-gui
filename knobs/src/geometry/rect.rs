use super::vector::Vector;

pub struct Rect {
	position: Vector,
	size: Vector,
}

impl Rect {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			position: Vector::new(x, y),
			size: Vector::new(width, height),
		}
	}
}
