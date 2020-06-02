pub mod control;
pub mod mouse_button;
pub mod rectangle;

use control::{Control, ControlSettings};
use ggez::graphics;

pub struct Gui {
	controls: Vec<Control>,
	next_control_id: usize,
	hovered_control: Option<usize>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			controls: vec![],
			next_control_id: 0,
			hovered_control: None,
		}
	}

	pub fn add_control(&mut self, settings: ControlSettings) {
		let id = self.next_control_id;
		self.next_control_id += 1;
		self.controls.push(Control::new(id, settings));
		self.controls
			.sort_by(|a, b| a.height.cmp(&b.height).reverse());
	}

	fn get_control_by_id(&self, id: usize) -> Option<&Control> {
		for control in &self.controls {
			if control.id == id {
				return Some(control);
			}
		}
		None
	}

	fn get_control_by_id_mut(&mut self, id: usize) -> Option<&mut Control> {
		for control in &mut self.controls {
			if control.id == id {
				return Some(control);
			}
		}
		None
	}

	pub fn on_mouse_move(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
		let previous_hovered_control = self.hovered_control;
		self.hovered_control = None;
		for control in &self.controls {
			if control.rectangle.contains_point(x, y) {
				self.hovered_control = Some(control.id);
				break;
			}
		}
		if self.hovered_control != previous_hovered_control {
			if let Some(id) = self.hovered_control {
				if let Some(control) = self.get_control_by_id_mut(id) {
					control.on_hover(x - control.rectangle.x, y - control.rectangle.y);
				}
			}
			if let Some(id) = previous_hovered_control {
				if let Some(control) = self.get_control_by_id_mut(id) {
					control.on_unhover();
				}
			}
		}
	}

	pub fn draw_debug(&self, mesh_builder: &mut graphics::MeshBuilder) {
		for control in &self.controls {
			let color = if self.hovered_control == Some(control.id) {
				graphics::Color::new(1.0, 0.0, 0.0, 1.0)
			} else {
				graphics::WHITE
			};
			mesh_builder.rectangle(
				graphics::DrawMode::stroke(2.0),
				graphics::Rect::new(
					control.rectangle.x,
					control.rectangle.y,
					control.rectangle.width,
					control.rectangle.height,
				),
				color,
			);
		}
	}
}
