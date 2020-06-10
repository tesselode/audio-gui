use crate::{
	behavior::Behavior,
	canvas::{Canvas, Color, DrawMode, Style},
	control::{Control, ControlSettings},
	error::InvalidFontError,
	event::Event,
	input::MouseButton,
};
use enum_map::{enum_map, EnumMap};
use rusttype::Font;
use std::collections::HashMap;

/// A unqiue identifier for a control.
pub type ControlId = usize;

/// A list of controls.
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

	/// Returns a reference to the control with the given ID.
	pub fn get(&self, id: &ControlId) -> Option<&Control> {
		self.controls.get(id)
	}

	/// Returns a mutable reference to the control with the given ID.
	pub fn get_mut(&mut self, id: &ControlId) -> Option<&mut Control> {
		self.controls.get_mut(id)
	}
}

/// A list of events to send to the audio thread.
pub struct EventQueue<CustomEvent> {
	events: Vec<Event<CustomEvent>>,
}

impl<CustomEvent> EventQueue<CustomEvent> {
	fn new() -> Self {
		Self { events: vec![] }
	}

	/// Pushes an event to the queue.
	pub fn push(&mut self, event: Event<CustomEvent>) {
		self.events.push(event);
	}
}

/// A collection of controls and associated behaviors.
///
/// A `Gui` holds controls and behaviors, takes mouse input,
/// determines when controls are interacted with, and emits
/// events from and to the audio thread.
pub struct Gui<CustomEvent> {
	/// The list of controls contained in the GUI.
	pub controls: Controls,
	behaviors: HashMap<ControlId, Vec<Box<dyn Behavior<CustomEvent>>>>,
	hovered_control: Option<ControlId>,
	held_control: EnumMap<MouseButton, Option<ControlId>>,
	event_queue: EventQueue<CustomEvent>,
	fonts: Vec<Font<'static>>,
}

impl<CustomEvent> Gui<CustomEvent>
where
	CustomEvent: Copy + Clone,
{
	/// Creates a new GUI.
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
			event_queue: EventQueue::new(),
			fonts: vec![],
		}
	}

	/// Adds a control to the GUI and attaches the given
	/// behaviors to it.
	pub fn add_control(
		&mut self,
		settings: ControlSettings,
		behaviors: Vec<Box<dyn Behavior<CustomEvent>>>,
	) -> ControlId {
		let id = self.controls.add(&settings);
		self.behaviors.insert(id, behaviors);
		id
	}

	/// Loads a font for use in the GUI.
	pub fn load_font(&mut self, font_data: &'static [u8]) -> Result<(), InvalidFontError> {
		match Font::try_from_bytes(font_data) {
			Some(font) => {
				self.fonts.push(font);
				Ok(())
			}
			None => Err(InvalidFontError {}),
		}
	}

	/// Gets a reference to a previously loaded font.
	pub fn get_font(&self, index: usize) -> Option<&Font> {
		self.fonts.get(index)
	}

	/// Emits an event to the behaviors in the GUI.
	///
	/// If a control ID is specified, the event will only be emitted to
	/// behaviors attached to the control with that ID. Otherwise, all
	/// behaviors will receive the event.
	pub fn emit(&mut self, event: Event<CustomEvent>, control_id: Option<ControlId>) {
		if let Some(id) = control_id {
			if let Some(behaviors) = self.behaviors.get_mut(&id) {
				for behavior in behaviors {
					behavior.on(event, &mut self.controls, &mut self.event_queue);
				}
			}
		} else {
			for (_, behaviors) in &mut self.behaviors {
				for behavior in behaviors {
					behavior.on(event, &mut self.controls, &mut self.event_queue);
				}
			}
		}
	}

	/// Flushes the event queue and returns a list of all of the event
	/// that the audio thread should process.
	pub fn drain_events(&mut self) -> Vec<Event<CustomEvent>> {
		self.event_queue.events.drain(..).collect()
	}

	/* it makes the most sense to store the control hovered/held state
	in the Gui struct, since only one control will be hovered/held at
	a time. however, it's also nice for the Behaviors to be able to access
	the hovered/held state via the Control structs, so we save that info
	there as well. but the state in the Gui struct is the "canonical" one. */
	fn update_control_state(&mut self) {
		for (id, control) in &mut self.controls.controls {
			control.is_hovered = self.hovered_control == Some(*id);
			for (mouse_button, held) in &mut control.is_held {
				*held = self.held_control[mouse_button] == Some(*id);
			}
		}
	}

	/// Tells the GUI about a mouse movement.
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
				self.emit(Event::Hover(id, relative_x, relative_y), Some(id));
			}
			if let Some(id) = previous_hovered_control {
				self.emit(Event::Unhover(id), Some(id));
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
					Event::Drag(*id, mouse_button, relative_x, relative_y, dx, dy),
					Some(*id),
				);
			}
		}
	}

	/// Tells the GUI about a mouse button press.
	pub fn on_mouse_down(&mut self, mouse_button: MouseButton, x: f32, y: f32) {
		if let Some(id) = self.hovered_control {
			// update the held state
			self.held_control[mouse_button] = Some(id);
			self.update_control_state();
			// emit the press event
			let control = self.controls.get(&id).unwrap();
			let relative_x = x - control.rectangle.x;
			let relative_y = y - control.rectangle.y;
			self.emit(
				Event::Press(id, mouse_button, relative_x, relative_y),
				Some(id),
			);
		}
	}

	/// Tells the GUI about a mouse button release.
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
			self.emit(
				Event::Release(id, mouse_button, relative_x, relative_y),
				Some(id),
			);
			if self.hovered_control == Some(id) {
				self.emit(
					Event::Click(id, mouse_button, relative_x, relative_y),
					Some(id),
				);
			}
		}
	}

	/// Draws the GUI to a canvas.
	pub fn draw(&self, canvas: &mut Canvas) {
		for (id, control) in &self.controls.controls {
			for behavior in &self.behaviors[id] {
				behavior.draw(control, canvas);
			}
		}
	}
}
