use super::Control;
use crate::gui::{canvas::Canvas, Controls, Event, EventQueue};

pub trait ControlBehavior<CustomEvent> {
	fn on(
		&mut self,
		_event: Event<CustomEvent>,
		_controls: &mut Controls,
		_event_queue: &mut EventQueue<CustomEvent>,
	) {
	}

	fn draw(&self, _control: &Control, _canvas: &mut Canvas) {}
}
