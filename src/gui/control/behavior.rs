use super::Control;
use crate::gui::{canvas::Canvas, ControlId, Controls, Event};

pub trait ControlBehavior {
	fn on(&mut self, _event: Event, _controls: &mut Controls, _id: &ControlId) {}
	fn draw(&self, _control: &Control, _canvas: &mut Canvas) {}
}
