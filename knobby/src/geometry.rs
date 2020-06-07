/// Represents a point in 2D space.
#[derive(Copy, Clone)]
pub struct Point {
	/// The x position of the point.
	pub x: f32,
	/// The y position of the point.
	pub y: f32,
}

impl Point {
	/// Creates a new point.
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	/// Creates a new point that's offset from an existing point.
	pub fn shifted(&self, dx: f32, dy: f32) -> Self {
		Self {
			x: self.x + dx,
			y: self.y + dy,
		}
	}
}

/// Represents a rectangle.
#[derive(Copy, Clone)]
pub struct Rectangle {
	/// The x position of the top-left corner of the rectangle.
	pub x: f32,
	/// The y position of the top-left corner of the rectangle.
	pub y: f32,
	/// The width of the rectangle.
	pub width: f32,
	/// The height of the rectangle.
	pub height: f32,
}

impl Rectangle {
	/// Creates a new rectangle.
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}

	/// Returns `true` if the rectangle contains the given point.
	pub fn contains_point(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
	}

	/// Gets the center of the rectangle.
	pub fn get_center(&self) -> Point {
		Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
	}
}
