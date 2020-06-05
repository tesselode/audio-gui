pub mod behavior;

use super::rectangle::Rectangle;

pub struct ControlSettings {
	pub rectangle: Rectangle,
	pub height: i32,
}

pub struct Control {
	pub rectangle: Rectangle,
	pub height: i32,
}

impl Control {
	pub fn new(settings: ControlSettings) -> Self {
		Self {
			rectangle: settings.rectangle,
			height: settings.height,
		}
	}
}
