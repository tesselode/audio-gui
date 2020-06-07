use crate::{
	canvas::Canvas,
	control::Control,
	event::Event,
	gui::{Controls, EventQueue},
};

pub trait Behavior<CustomEvent> {
	fn on(
		&mut self,
		_event: Event<CustomEvent>,
		_controls: &mut Controls,
		_event_queue: &mut EventQueue<CustomEvent>,
	) {
	}

	fn draw(&self, _control: &Control, _canvas: &mut Canvas) {}
}
