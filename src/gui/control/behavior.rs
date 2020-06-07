use super::Control;
use crate::gui::{canvas::Canvas, Controls, Event, Parameters};

pub trait ControlBehavior {
	fn on(&mut self, _event: Event, _controls: &mut Controls, _parameters: &mut Parameters) {}

	fn draw(&self, _control: &Control, _parameters: &Parameters, _canvas: &mut Canvas) {}
}
