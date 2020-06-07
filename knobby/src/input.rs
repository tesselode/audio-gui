use enum_map::Enum;

/// A list of mouse buttons.
#[derive(Copy, Clone, Enum, Debug)]
pub enum MouseButton {
	/// The left mouse button.
	Left,
	/// The middle mouse button (i.e. the scroll wheel is pressed down).
	Middle,
	/// The right mouse button.
	Right,
}
