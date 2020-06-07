#[derive(Copy, Clone)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn shifted(&self, dx: f32, dy: f32) -> Self {
		Self {
			x: self.x + dx,
			y: self.y + dy,
		}
	}
}
