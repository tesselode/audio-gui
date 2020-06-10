use crate::{
	canvas::Canvas,
	control::Control,
	event::Event,
	gui::{Controls, EventQueue, Resources},
};

/// Defines a set of behaviors and a visual representation for
/// a control.
///
/// A behavior can:
/// - Run code in response to events
/// - Send events to the outer audio code
/// - Modify other controls
/// - Draw something to the screen
///
/// A control can have multiple behaviors attached to it.
///
/// A common use for a behavior is to define a GUI control
/// that changes a parameter, like a knob or slider.
pub trait Behavior<CustomEvent> {
	/// Called when an event is emitted inside the GUI.
	fn on(
		&mut self,
		_event: Event<CustomEvent>,
		_controls: &mut Controls,
		_resources: &Resources,
		_event_queue: &mut EventQueue<CustomEvent>,
	) {
	}

	/// Defines how a control should be drawn.
	fn draw(&self, _control: &Control, _resources: &Resources, _canvas: &mut Canvas) {}
}
