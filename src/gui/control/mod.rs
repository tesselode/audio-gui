pub mod behavior;

use super::rectangle::Rectangle;
use behavior::ControlBehavior;

pub struct ControlSettings {
	pub rectangle: Rectangle,
	pub height: i32,
	pub behaviors: Vec<Box<dyn ControlBehavior>>,
}

pub struct Control {
	pub rectangle: Rectangle,
	pub height: i32,
}

impl Control {
	pub fn new(settings: &ControlSettings) -> Self {
		Self {
			rectangle: settings.rectangle,
			height: settings.height,
		}
	}
}
