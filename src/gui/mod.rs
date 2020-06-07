pub mod canvas;
pub mod control;
pub mod mouse_button;
pub mod point;
pub mod rectangle;

use canvas::{Canvas, Color, DrawMode, Style};
use control::{behavior::ControlBehavior, Control, ControlSettings};
use enum_map::{enum_map, EnumMap};
use mouse_button::MouseButton;
use std::collections::HashMap;

pub type ControlId = usize;

#[derive(Copy, Clone)]
pub enum Event {
	Hover(f32, f32),
	Unhover,
	Press(MouseButton, f32, f32),
	Release(MouseButton, f32, f32),
	Click(MouseButton, f32, f32),
	Drag(MouseButton, f32, f32, f32, f32),
}

pub struct Controls {
	controls: HashMap<ControlId, Control>,
	next_control_id: ControlId,
}

impl Controls {
	fn new() -> Self {
		Self {
			controls: HashMap::new(),
			next_control_id: 0,
		}
	}

	fn add(&mut self, settings: &ControlSettings) -> ControlId {
		let id = self.next_control_id;
		self.next_control_id += 1;
		self.controls.insert(id, Control::new(settings));
		id
	}

	pub fn get(&self, id: &ControlId) -> Option<&Control> {
		self.controls.get(id)
	}

	pub fn get_mut(&mut self, id: &ControlId) -> Option<&mut Control> {
		self.controls.get_mut(id)
	}
}

pub struct Gui {
	pub controls: Controls,
	behaviors: HashMap<ControlId, Vec<Box<dyn ControlBehavior>>>,
	hovered_control: Option<ControlId>,
	held_control: EnumMap<MouseButton, Option<ControlId>>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			controls: Controls::new(),
			behaviors: HashMap::new(),
			hovered_control: None,
			held_control: enum_map! {
				MouseButton::Left => None,
				MouseButton::Middle => None,
				MouseButton::Right => None,
			},
		}
	}

	pub fn add_control(&mut self, settings: ControlSettings) -> ControlId {
		let id = self.controls.add(&settings);
		self.behaviors.insert(id, settings.behaviors);
		id
	}

	fn emit(&mut self, event: Event, id: ControlId) {
		if let Some(behaviors) = self.behaviors.get_mut(&id) {
			for behavior in behaviors {
				behavior.on(event, &mut self.controls, &id);
			}
		}
	}

	/* it makes the most sense to store the control hovered/held state
	in the Gui struct, since only one control will be hovered/held at
	a time. however, it's also nice for the Behaviors to be able to access
	the hovered/held state via the Control structs, so we save that info
	there as well. but the state in the Gui struct is the "canonical" one. */
	fn update_control_state(&mut self) {
		for (id, control) in &mut self.controls.controls {
			control.is_hovered = self.hovered_control == Some(*id);
			control.is_held[MouseButton::Left] = self.held_control[MouseButton::Left] == Some(*id);
			control.is_held[MouseButton::Middle] =
				self.held_control[MouseButton::Middle] == Some(*id);
			control.is_held[MouseButton::Right] =
				self.held_control[MouseButton::Right] == Some(*id);
		}
	}

	pub fn on_mouse_move(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
		let previous_hovered_control = self.hovered_control;
		// get the first hovered control
		self.hovered_control = None;
		for (id, control) in &self.controls.controls {
			if control.rectangle.contains_point(x, y) {
				self.hovered_control = Some(*id);
				break;
			}
		}
		// save the hovered state to the controls
		self.update_control_state();
		// emit hover/unhover events
		if self.hovered_control != previous_hovered_control {
			if let Some(id) = self.hovered_control {
				let control = self.controls.get(&id).unwrap();
				let relative_x = x - control.rectangle.x;
				let relative_y = y - control.rectangle.y;
				self.emit(Event::Hover(relative_x, relative_y), id);
			}
			if let Some(id) = previous_hovered_control {
				self.emit(Event::Unhover, id);
			}
		}
		// emit drag events
		let held_control = self.held_control;
		for (mouse_button, held) in &held_control {
			if let Some(id) = held {
				let control = self.controls.get(&id).unwrap();
				let relative_x = x - control.rectangle.x;
				let relative_y = y - control.rectangle.y;
				self.emit(
					Event::Drag(mouse_button, relative_x, relative_y, dx, dy),
					*id,
				);
			}
		}
	}

	pub fn on_mouse_down(&mut self, mouse_button: MouseButton, x: f32, y: f32) {
		if let Some(id) = self.hovered_control {
			// update the held state
			self.held_control[mouse_button] = Some(id);
			self.update_control_state();
			// emit the press event
			let control = self.controls.get(&id).unwrap();
			let relative_x = x - control.rectangle.x;
			let relative_y = y - control.rectangle.y;
			self.emit(Event::Press(mouse_button, relative_x, relative_y), id);
		}
	}

	pub fn on_mouse_up(&mut self, mouse_button: MouseButton, x: f32, y: f32) {
		let previous_held_control = self.held_control;
		if let Some(id) = previous_held_control[mouse_button] {
			// update the held state
			self.held_control[mouse_button] = None;
			self.update_control_state();
			// emit release/click events
			let control = self.controls.get(&id).unwrap();
			let relative_x = x - control.rectangle.x;
			let relative_y = y - control.rectangle.y;
			self.emit(Event::Release(mouse_button, relative_x, relative_y), id);
			if self.hovered_control == Some(id) {
				self.emit(Event::Click(mouse_button, relative_x, relative_y), id);
			}
		}
	}

	pub fn draw(&self, canvas: &mut Canvas) {
		for (id, control) in &self.controls.controls {
			for behavior in &self.behaviors[id] {
				behavior.draw(control, canvas);
			}
		}
	}

	pub fn draw_debug(&self, canvas: &mut Canvas) {
		for (id, control) in &self.controls.controls {
			let color = if self.held_control[MouseButton::Left] == Some(*id) {
				Color::new(1.0, 1.0, 0.0, 1.0)
			} else if self.hovered_control == Some(*id) {
				Color::new(1.0, 0.0, 0.0, 1.0)
			} else {
				Color::new(1.0, 1.0, 1.0, 1.0)
			};
			canvas.draw_rectangle(
				control.rectangle,
				Style {
					mode: DrawMode::Stroke(2.0),
					color,
				},
			);
		}
	}
}
