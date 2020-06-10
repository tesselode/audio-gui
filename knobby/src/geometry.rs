use rusttype::{Font, Scale};

/// Represents a point in 2D space.
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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

	pub fn around_text(font: &Font, text: &str, size: f32, position: Point) -> Self {
		let mut min_x: f32 = position.x;
		let mut min_y: f32 = position.y;
		let mut max_x: f32 = position.x;
		let mut max_y: f32 = position.y;
		for glyph in font.layout(
			text,
			Scale { x: size, y: size },
			rusttype::Point {
				x: position.x,
				y: position.y,
			},
		) {
			let v_metrics = font.v_metrics(glyph.scale());
			if let Some(bounds) = glyph.pixel_bounding_box() {
				min_x = min_x.min(bounds.min.x as f32);
				min_y = min_y.min(bounds.min.y as f32 + v_metrics.ascent);
				max_x = max_x.max(bounds.max.x as f32);
				max_y = max_y.max(bounds.max.y as f32 + v_metrics.ascent);
			}
		}
		Self {
			x: min_x,
			y: min_y,
			width: max_x - min_x,
			height: max_y - min_y,
		}
	}

	/// Returns `true` if the rectangle contains the given point.
	pub fn contains_point(&self, point: Point) -> bool {
		point.x >= self.x
			&& point.x <= self.x + self.width
			&& point.y >= self.y
			&& point.y <= self.y + self.height
	}

	/// Gets the center of the rectangle.
	pub fn get_center(&self) -> Point {
		Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
	}

	pub fn pad_left(&self, amount: f32) -> Self {
		Self {
			x: self.x - amount,
			y: self.y,
			width: self.width + amount,
			height: self.height,
		}
	}

	pub fn pad_top(&self, amount: f32) -> Self {
		Self {
			x: self.x,
			y: self.y - amount,
			width: self.width,
			height: self.height + amount,
		}
	}

	pub fn pad_right(&self, amount: f32) -> Self {
		Self {
			x: self.x,
			y: self.y,
			width: self.width + amount,
			height: self.height,
		}
	}

	pub fn pad_bottom(&self, amount: f32) -> Self {
		Self {
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height + amount,
		}
	}

	pub fn pad_horizontal(&self, amount: f32) -> Self {
		Self {
			x: self.x - amount,
			y: self.y,
			width: self.width + amount * 2.0,
			height: self.height,
		}
	}

	pub fn pad_vertical(&self, amount: f32) -> Self {
		Self {
			x: self.x,
			y: self.y - amount,
			width: self.width,
			height: self.height + amount * 2.0,
		}
	}

	pub fn pad(&self, amount: f32) -> Self {
		Self {
			x: self.x - amount,
			y: self.y - amount,
			width: self.width + amount * 2.0,
			height: self.height + amount * 2.0,
		}
	}

	pub fn scale(&self, x_amount: f32, y_amount: f32, origin: Point) -> Self {
		let width_increment = self.width * x_amount - self.width;
		let height_increment = self.height * y_amount - self.height;
		Self {
			x: self.x - width_increment * origin.x,
			y: self.y - height_increment * origin.y,
			width: self.width + width_increment,
			height: self.height + height_increment,
		}
	}
}
