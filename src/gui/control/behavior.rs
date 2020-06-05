use super::{super::mouse_button::MouseButton, Control};

pub trait ControlBehavior {
	fn on_hover(&mut self, _control: &mut Control, _x: f32, _y: f32) {}

	fn on_unhover(&mut self, _control: &mut Control) {}

	fn on_press(&mut self, _control: &mut Control, _button: MouseButton, _x: f32, _y: f32) {}

	fn on_release(&mut self, _control: &mut Control, _button: MouseButton, _x: f32, _y: f32) {}

	fn on_drag(
		&mut self,
		_control: &mut Control,
		_button: MouseButton,
		_x: f32,
		_y: f32,
		_dx: f32,
		_dy: f32,
	) {
	}
}
