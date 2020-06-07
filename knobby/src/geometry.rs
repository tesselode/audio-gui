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

#[derive(Copy, Clone)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rectangle {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}

	pub fn contains_point(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
	}

	pub fn get_center(&self) -> Point {
		Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
	}
}
