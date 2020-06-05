use super::super::mouse_button::MouseButton;
use crate::gui::{ControlId, Controls};

pub trait ControlBehavior {
	fn on_hover(&mut self, _controls: &mut Controls, _id: &ControlId, _x: f32, _y: f32) {}

	fn on_unhover(&mut self, _controls: &mut Controls, _id: &ControlId) {}

	fn on_press(
		&mut self,
		_controls: &mut Controls,
		_id: &ControlId,
		_button: MouseButton,
		_x: f32,
		_y: f32,
	) {
	}

	fn on_release(
		&mut self,
		_controls: &mut Controls,
		_id: &ControlId,
		_button: MouseButton,
		_x: f32,
		_y: f32,
	) {
	}

	fn on_drag(
		&mut self,
		_controls: &mut Controls,
		_id: &ControlId,
		_button: MouseButton,
		_x: f32,
		_y: f32,
		_dx: f32,
		_dy: f32,
	) {
	}
}
