use super::{geometry::Rectangle, input::MouseButton};
use enum_map::{enum_map, EnumMap};

/// The configuration for a new control.
pub struct ControlSettings {
	/// The rectangular bounds of the control.
	pub rectangle: Rectangle,
	/// The height of the control.
	pub height: i32,
}

/// A rectangular space in the GUI that can be interacted with.
///
/// Usually this is a knob, slider, menu, etc.
///
/// A control is associated with any number of behaviors,
/// which define how the control behaves and looks. A control
/// doesn't do anything on its own.
pub struct Control {
	/// The rectangular bounds of the control.
	pub rectangle: Rectangle,
	/// The height of the control.
	pub height: i32,
	/// Whether the mouse is currently hovering over the control.
	pub is_hovered: bool,
	/// Whether the user is "holding down" the control with the given
	/// `MouseButton`.
	pub is_held: EnumMap<MouseButton, bool>,
}

impl Control {
	/// Creates a new control.
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
