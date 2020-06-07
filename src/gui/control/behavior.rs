use super::Control;
use crate::gui::{canvas::Canvas, ControlId, Controls, Event, GlobalEvent, Parameters};

pub trait ControlBehavior {
	fn on(
		&mut self,
		_event: Event,
		_controls: &mut Controls,
		_id: &ControlId,
		_parameters: &mut Parameters,
	) {
	}

	fn on_global(
		&mut self,
		_event: GlobalEvent,
		_controls: &mut Controls,
		_parameters: &mut Parameters,
	) {
	}

	fn draw(&self, _control: &Control, _parameters: &Parameters, _canvas: &mut Canvas) {}
}
