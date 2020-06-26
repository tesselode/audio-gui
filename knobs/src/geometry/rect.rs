use super::vector::Vector;

#[derive(Debug, Copy, Clone)]
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

	pub fn shifted(self, translation: Vector) -> Self {
		Self {
			position: self.position + translation,
			..self
		}
	}

	pub fn contains_point(&self, point: Vector) -> bool {
		point.x >= self.position.x
			&& point.x <= self.position.x + self.size.x
			&& point.y >= self.position.y
			&& point.y <= self.position.y + self.size.y
	}
}

impl Default for Rect {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0, 0.0)
	}
}
