use super::{geometry::Rectangle, input::MouseButton};
use enum_map::{enum_map, EnumMap};

pub struct ControlSettings {
	pub rectangle: Rectangle,
	pub height: i32,
}

pub struct Control {
	pub rectangle: Rectangle,
	pub height: i32,
	pub is_hovered: bool,
	pub is_held: EnumMap<MouseButton, bool>,
}

impl Control {
	pub fn new(settings: &ControlSettings) -> Self {
		Self {
			rectangle: settings.rectangle,
			height: settings.height,
			is_hovered: false,
			is_held: enum_map! {
				MouseButton::Left => false,
				MouseButton::Middle => false,
				MouseButton::Right => false,
			},
		}
	}
}
