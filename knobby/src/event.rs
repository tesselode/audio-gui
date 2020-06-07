use crate::{gui::ControlId, input::MouseButton};

/// Represents something that happened in the GUI or the
/// audio thread.
#[derive(Copy, Clone)]
pub enum Event<CustomEvent> {
	/// Emitted when a control is hovered.
	///
	/// Contains:
	/// - the ID of the control
	/// - the x position of the mouse relative to the left side of the control
	/// - the y position of the mouse relative to the top of the control
	Hover(ControlId, f32, f32),
	/// Emitted when a control with the given ID is no longer hovered.
	Unhover(ControlId),
	/// Emitted when a control starts being "held down".
	///
	/// Contains:
	/// - the ID of the control
	/// - the button used to hold the control
	/// - the x position of the mouse relative to the left side of the control
	/// - the y position of the mouse relative to the top of the control
	Press(ControlId, MouseButton, f32, f32),
	/// Emitted when a control stops being "held down".
	///
	/// Contains:
	/// - the ID of the control
	/// - the button previously used to hold the control
	/// - the x position of the mouse relative to the left side of the control
	/// - the y position of the mouse relative to the top of the control
	Release(ControlId, MouseButton, f32, f32),
	/// Emitted when a control is clicked.
	///
	/// More specifically, this is emitted when a control
	/// is held down with a certain mouse button, and then that
	/// mouse button is released while the mouse is still over
	/// the control.
	///
	/// This is probably the event you'll want to use for button-clicking
	/// behavior.
	///
	/// Contains:
	/// - the ID of the control
	/// - the button used to hold the control
	/// - the x position of the mouse relative to the left side of the control
	/// - the y position of the mouse relative to the top of the control
	Click(ControlId, MouseButton, f32, f32),
	/// Emitted when a control is dragged.
	///
	/// Contains:
	/// - the ID of the control
	/// - the button used to hold the control
	/// - the x position of the mouse relative to the left side of the control
	/// - the y position of the mouse relative to the top of the control
	/// - the amount the mouse moved on the x-axis
	/// - the amount the mouse moved on the y-axis
	Drag(ControlId, MouseButton, f32, f32, f32, f32),
	/// Emitted when a parameter is changed or should be changed.
	///
	/// When emitted to the GUI, this represents a parameter that
	/// *was* changed in the audio thread. When emitted to the audio
	/// thread, this represents a parameter that *should* change
	/// because of a GUI interaction.
	///
	/// Contains:
	/// - the index of the parameter
	/// - the new value of the parameter
	SetParameter(i32, f32),
	/// Emitted to tell the audio thread that the parameter with the given
	/// ID should be reset to its default value.
	ResetParameter(i32),
	/// A user-defined event.
	Custom(CustomEvent),
}
