use super::Control;
use crate::gui::{canvas::Canvas, Controls, Event, EventQueue};

pub trait ControlBehavior {
	fn on(&mut self, _event: Event, _controls: &mut Controls, _event_queue: &mut EventQueue) {}

	fn draw(&self, _control: &Control, _canvas: &mut Canvas) {}
}
