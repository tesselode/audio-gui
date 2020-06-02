pub mod behavior;

use super::mouse_button::MouseButton;
use super::rectangle::Rectangle;
use behavior::ControlBehavior;

pub struct ControlSettings {
	pub rectangle: Rectangle,
	pub height: i32,
	pub behaviors: Vec<Box<dyn ControlBehavior>>,
}

pub struct Control {
	pub id: usize,
	pub rectangle: Rectangle,
	pub height: i32,
	pub behaviors: Vec<Box<dyn ControlBehavior>>,
}

impl Control {
	pub fn new(id: usize, settings: ControlSettings) -> Self {
		Self {
			id,
			rectangle: settings.rectangle,
			height: settings.height,
			behaviors: settings.behaviors,
		}
	}

	pub fn on_hover(&mut self, x: f32, y: f32) {
		for behavior in &mut self.behaviors {
			behavior.on_hover(x, y);
		}
	}

	pub fn on_unhover(&mut self) {
		for behavior in &mut self.behaviors {
			behavior.on_unhover();
		}
	}

	pub fn on_press(&mut self, button: MouseButton, x: f32, y: f32) {
		for behavior in &mut self.behaviors {
			behavior.on_press(button, x, y);
		}
	}

	pub fn on_release(&mut self, button: MouseButton, x: f32, y: f32) {
		for behavior in &mut self.behaviors {
			behavior.on_release(button, x, y);
		}
	}

	pub fn on_drag(&mut self, button: MouseButton, x: f32, y: f32, dx: f32, dy: f32) {
		for behavior in &mut self.behaviors {
			behavior.on_drag(button, x, y, dx, dy);
		}
	}
}
