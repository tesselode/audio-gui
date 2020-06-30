use super::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
	pub position: Vector,
	pub size: Vector,
}

impl Rect {
	pub fn new() -> Self {
		Self {
			position: Vector::zero(),
			size: Vector::zero(),
		}
	}

	pub fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			position: Vector::new(x, y),
			size: Vector::new(width, height),
		}
	}

	pub fn get_x(&self, origin: f32) -> f32 {
		self.position.x + self.size.x * origin
	}

	pub fn get_y(&self, origin: f32) -> f32 {
		self.position.y + self.size.y * origin
	}

	pub fn get_position(&self, origin: Vector) -> Vector {
		Vector::new(self.get_x(origin.x), self.get_y(origin.y))
	}

	pub fn get_width(&self) -> f32 {
		self.size.x
	}

	pub fn get_height(&self) -> f32 {
		self.size.y
	}

	pub fn contains_point(&self, point: Vector) -> bool {
		point.x >= self.position.x
			&& point.x <= self.position.x + self.size.x
			&& point.y >= self.position.y
			&& point.y <= self.position.y + self.size.y
	}

	pub fn set_width(&mut self, width: f32) {
		self.size.x = width;
	}

	pub fn with_width(mut self, width: f32) -> Self {
		self.set_width(width);
		self
	}

	pub fn set_height(&mut self, height: f32) {
		self.size.y = height;
	}

	pub fn with_height(mut self, height: f32) -> Self {
		self.set_height(height);
		self
	}

	pub fn set_size(&mut self, size: Vector) {
		self.size = size;
	}

	pub fn with_size(mut self, size: Vector) -> Self {
		self.set_size(size);
		self
	}

	pub fn set_x(&mut self, x: f32, origin: f32) {
		self.position.x = x - self.size.x * origin;
	}

	pub fn with_x(mut self, x: f32, origin: f32) -> Self {
		self.set_x(x, origin);
		self
	}

	pub fn set_y(&mut self, y: f32, origin: f32) {
		self.position.y = y - self.size.y * origin;
	}

	pub fn with_y(mut self, y: f32, origin: f32) -> Self {
		self.set_y(y, origin);
		self
	}

	pub fn set_position(&mut self, position: Vector, origin: Vector) {
		self.position.x = position.x - self.size.x * origin.x;
		self.position.y = position.y - self.size.y * origin.y;
	}

	pub fn with_position(mut self, position: Vector, origin: Vector) -> Self {
		self.set_position(position, origin);
		self
	}

	pub fn shift(&mut self, translation: Vector) {
		self.position += translation;
	}

	pub fn shifted(mut self, translation: Vector) -> Self {
		self.shift(translation);
		self
	}
}

impl Default for Rect {
	fn default() -> Self {
		Self::from_xywh(0.0, 0.0, 0.0, 0.0)
	}
}
