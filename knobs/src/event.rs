use crate::{geometry::vector::Vector, gui::ElementId, input::MouseButton};

pub enum Event {
	Hover(ElementId, Vector),
	Unhover(ElementId),
	Press(ElementId, MouseButton),
	Release(ElementId, MouseButton),
	Click(ElementId, MouseButton),
}

pub struct EventQueue {
	pub(crate) events: Vec<(Event, Option<ElementId>)>,
	pub(crate) output_events: Vec<Event>,
}

impl EventQueue {
	pub fn new() -> Self {
		Self {
			events: vec![],
			output_events: vec![],
		}
	}

	pub fn push(&mut self, event: Event, element_id: Option<ElementId>) {
		self.events.push((event, element_id));
	}

	pub fn output(&mut self, event: Event) {
		self.output_events.push(event);
	}
}
