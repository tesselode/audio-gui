pub mod control;
pub mod display;
pub mod mouse_button;
pub mod point;
pub mod rectangle;

use control::{behavior::ControlBehavior, Control, ControlSettings};
use display::{Color, Display, DrawMode, Style};
use std::collections::HashMap;

pub type ControlId = usize;

pub struct Gui {
	controls: HashMap<ControlId, Control>,
	behaviors: HashMap<ControlId, Vec<Box<dyn ControlBehavior>>>,
	next_control_id: ControlId,
	hovered_control: Option<ControlId>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			controls: HashMap::new(),
			behaviors: HashMap::new(),
			next_control_id: 0,
			hovered_control: None,
		}
	}

	pub fn add_control(&mut self, settings: ControlSettings) -> ControlId {
		let id = self.next_control_id;
		self.next_control_id += 1;
		self.controls.insert(id, Control::new(settings));
		self.behaviors.insert(id, vec![]);
		id
	}

	pub fn attach_behavior(&mut self, id: ControlId, behavior: Box<dyn ControlBehavior>) {
		let behaviors = self
			.behaviors
			.get_mut(&id)
			.expect(&format!("No control with ID {}", id));
		behaviors.push(behavior);
	}

	fn get_control_by_id(&self, id: usize) -> Option<&Control> {
		self.controls.get(&id)
	}

	fn get_control_by_id_mut(&mut self, id: usize) -> Option<&mut Control> {
		self.controls.get_mut(&id)
	}

	fn on_hover_control(&mut self, id: ControlId, x: f32, y: f32) {
		if let Some(control) = self.controls.get_mut(&id) {
			if let Some(behaviors) = self.behaviors.get_mut(&id) {
				for behavior in behaviors {
					behavior.on_hover(control, x, y);
				}
			}
		}
	}

	fn on_unhover_control(&mut self, id: ControlId) {
		if let Some(control) = self.controls.get_mut(&id) {
			if let Some(behaviors) = self.behaviors.get_mut(&id) {
				for behavior in behaviors {
					behavior.on_unhover(control);
				}
			}
		}
	}

	pub fn on_mouse_move(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
		let previous_hovered_control = self.hovered_control;
		self.hovered_control = None;
		for (id, control) in &self.controls {
			if control.rectangle.contains_point(x, y) {
				self.hovered_control = Some(*id);
				break;
			}
		}
		if self.hovered_control != previous_hovered_control {
			if let Some(id) = self.hovered_control {
				if let Some(control) = self.controls.get(&id) {
					let relative_x = x - control.rectangle.x;
					let relative_y = y - control.rectangle.y;
					self.on_hover_control(id, relative_x, relative_y);
				}
			}
			if let Some(id) = previous_hovered_control {
				self.on_unhover_control(id);
			}
		}
	}

	pub fn draw_debug(&self, display: &mut impl Display) {
		for (id, control) in &self.controls {
			let color = if self.hovered_control == Some(*id) {
				Color::new(1.0, 0.0, 0.0, 1.0)
			} else {
				Color::new(1.0, 1.0, 1.0, 1.0)
			};
			display.draw_rectangle(
				control.rectangle,
				Style {
					mode: DrawMode::Stroke(2.0),
					color,
				},
			);
		}
	}
}
