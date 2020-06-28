pub mod element_id;
pub mod elements;

pub use element_id::{ElementId, ElementIdIter};
pub use elements::{Elements, TreeNode};

use crate::{
	canvas::Canvas,
	component::Component,
	event::{Event, EventQueue},
	geometry::{rect::Rect, vector::Vector},
	input::MouseButton,
	resources::Resources,
};
use enum_map::{enum_map, EnumMap};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Element {
	pub rect: Rect,
	pub height: f32,
	pub parent_id: Option<ElementId>,
	pub hovered: bool,
	pub held: EnumMap<MouseButton, bool>,
}

#[derive(Default)]
pub struct ElementSettings {
	pub rect: Rect,
	pub height: f32,
	pub components: Vec<Box<dyn Component>>,
	pub children: Vec<ElementSettings>,
}

pub struct Gui {
	pub resources: Resources,
	pub elements: Elements,
	pub components: HashMap<ElementId, Vec<Box<dyn Component>>>,
	event_queue: EventQueue,
	parent_stack: Vec<ElementId>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			resources: Resources::new(),
			elements: Elements::new(),
			components: HashMap::new(),
			event_queue: EventQueue::new(),
			parent_stack: vec![],
		}
	}

	pub fn add(&mut self, settings: ElementSettings) -> ElementId {
		let id = self.elements.next_element_id();
		self.elements.elements.push(Element {
			rect: settings.rect,
			height: settings.height,
			parent_id: match self.parent_stack.last() {
				Some(index) => Some(*index),
				None => None,
			},
			hovered: false,
			held: enum_map! {
				MouseButton::Left => false,
				MouseButton::Middle => false,
				MouseButton::Right => false,
			},
		});
		let mut components = vec![];
		for component in settings.components {
			components.push(component);
		}
		self.components.insert(id, components);
		self.parent_stack.push(id);
		for child_settings in settings.children {
			self.add(child_settings);
		}
		self.parent_stack.pop();
		id
	}

	fn flush_event_queue(&mut self) {
		while self.event_queue.events.len() > 0 {
			let mut events = vec![];
			for event in self.event_queue.events.drain(..) {
				events.push(event);
			}
			for (event, element_id) in &events {
				if let Some(id) = element_id {
					for component in self.components.get_mut(&id).unwrap() {
						component.on(event, &mut self.elements, &mut self.event_queue);
					}
				} else {
					for (_, components) in &mut self.components {
						for component in components {
							component.on(event, &mut self.elements, &mut self.event_queue);
						}
					}
				}
			}
		}
	}

	pub fn emit(&mut self, event: Event, element_id: Option<ElementId>) {
		self.event_queue.push(event, element_id);
		self.flush_event_queue();
	}

	pub fn drain_output_events(&mut self) -> Vec<Event> {
		let mut events = vec![];
		for event in self.event_queue.output_events.drain(..) {
			events.push(event);
		}
		events
	}

	fn update_hover_state(
		&mut self,
		nodes: &Vec<TreeNode>,
		mouse_position: Vector,
		mut blocked: bool,
	) -> bool {
		for node in nodes.iter().rev() {
			let element_position = self.elements.get(node.element_id).rect.position;
			let relative_mouse_position = mouse_position - element_position;
			/* update the child elements first. if one of them blocks the parent element,
			then we know the parent element can't be hovered. */
			if self.update_hover_state(&node.children, relative_mouse_position, blocked) {
				blocked = true;
			}
			let mut element = self.elements.get_mut(node.element_id);
			let hovered_previous = element.hovered;
			/* the parent element is hovered if the mouse is over its rect and it's not blocked
			by any other elements */
			let hovered = !blocked && element.rect.contains_point(mouse_position);
			element.hovered = hovered;
			if hovered {
				blocked = true;
			}
			// emit hover/unhover events
			if hovered && !hovered_previous {
				self.emit(
					Event::Hover(node.element_id, relative_mouse_position),
					Some(node.element_id),
				);
			}
			if hovered_previous && !hovered {
				self.emit(Event::Unhover(node.element_id), Some(node.element_id));
			}
		}
		blocked
	}

	pub fn on_move_mouse(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
		let mouse_position = Vector::new(x, y);
		let nodes = self.elements.get_tree(None);
		self.update_hover_state(&nodes, mouse_position, false);
	}

	pub fn on_press_mouse_button(&mut self, button: MouseButton) {
		let mut events = vec![];
		for (id, element) in self.elements.iter_mut() {
			if element.hovered {
				element.held[button] = true;
				events.push((Event::Press(id, button), Some(id)));
			}
		}
		for (event, id) in events {
			self.emit(event, id);
		}
	}

	pub fn on_release_mouse_button(&mut self, button: MouseButton) {
		let mut events = vec![];
		for (id, element) in self.elements.iter_mut() {
			if element.held[button] {
				element.held[button] = false;
				events.push((Event::Release(id, button), Some(id)));
				if element.hovered {
					events.push((Event::Click(id, button), Some(id)));
				}
			}
		}
		for (event, id) in events {
			self.emit(event, id);
		}
	}

	fn draw_nodes(&self, nodes: &Vec<TreeNode>, canvas: &mut Canvas) {
		for node in nodes {
			let element = self.elements.get(node.element_id);
			for component in self.components.get(&node.element_id).unwrap() {
				component.draw_below(element, canvas, &self.resources);
			}
			canvas.push_translation(element.rect.position);
			self.draw_nodes(&node.children, canvas);
			canvas.pop_translation();
			for component in self.components.get(&node.element_id).unwrap() {
				component.draw_above(element, canvas, &self.resources);
			}
		}
	}

	fn layout(&mut self, nodes: &Vec<TreeNode>) {
		for node in nodes {
			self.layout(&node.children);
			for component in self.components.get_mut(&node.element_id).unwrap() {
				component.layout(&mut self.elements, node.element_id, &self.resources);
			}
		}
	}

	pub fn draw(&mut self) -> Canvas {
		let mut canvas = Canvas::new();
		let nodes = self.elements.get_tree(None);
		self.layout(&nodes);
		self.draw_nodes(&nodes, &mut canvas);
		canvas
	}
}
