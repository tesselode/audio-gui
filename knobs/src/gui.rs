use crate::{
	behavior::Behavior,
	canvas::Canvas,
	event::Event,
	geometry::{rect::Rect, vector::Vector},
	input::MouseButton,
};
use enum_map::{enum_map, EnumMap};
use std::{
	collections::HashMap,
	iter::Zip,
	slice::{Iter, IterMut},
};

#[derive(Debug, Eq, Hash, Copy, Clone)]
pub struct ElementId {
	index: usize,
}

impl PartialEq for ElementId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

pub struct ElementIdIter {
	index: usize,
	len: usize,
}

impl ElementIdIter {
	fn new(len: usize) -> Self {
		Self { index: 0, len }
	}
}

impl Iterator for ElementIdIter {
	type Item = ElementId;

	fn next(&mut self) -> Option<Self::Item> {
		let item = if self.index < self.len {
			Some(ElementId { index: self.index })
		} else {
			None
		};
		self.index += 1;
		item
	}
}

#[derive(Debug)]
pub struct Element {
	pub rect: Rect,
	pub height: f32,
	pub parent_id: Option<ElementId>,
	pub hover_position: Option<Vector>,
	pub held: EnumMap<MouseButton, bool>,
}

impl Element {
	pub fn is_hovered(&self) -> bool {
		self.hover_position.is_some()
	}
}

#[derive(Debug)]
pub struct TreeNode {
	element_id: ElementId,
	children: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct Elements {
	elements: Vec<Element>,
}

impl Elements {
	pub fn new() -> Self {
		Self { elements: vec![] }
	}

	fn next_element_id(&self) -> ElementId {
		ElementId {
			index: self.elements.len(),
		}
	}

	pub fn iter(&self) -> Zip<ElementIdIter, Iter<Element>> {
		ElementIdIter::new(self.elements.len()).zip(self.elements.iter())
	}

	pub fn iter_mut(&mut self) -> Zip<ElementIdIter, IterMut<Element>> {
		ElementIdIter::new(self.elements.len()).zip(self.elements.iter_mut())
	}

	pub fn get(&self, element_id: ElementId) -> &Element {
		self.elements.get(element_id.index).unwrap()
	}

	pub fn get_mut(&mut self, element_id: ElementId) -> &mut Element {
		self.elements.get_mut(element_id.index).unwrap()
	}

	pub fn children_of(&self, parent_id: ElementId) -> Vec<ElementId> {
		let mut children = vec![];
		for (id, element) in self.iter() {
			if element.parent_id == Some(parent_id) {
				children.push(id);
			}
		}
		children
	}

	pub fn get_tree(&self, parent_id: Option<ElementId>) -> Vec<TreeNode> {
		let mut nodes = vec![];
		for (element_index, element) in self.iter() {
			if element.parent_id == parent_id {
				nodes.push(TreeNode {
					element_id: element_index,
					children: self.get_tree(Some(element_index)),
				});
			}
		}
		nodes
	}
}

#[derive(Default)]
pub struct ElementSettings {
	pub rect: Rect,
	pub height: f32,
	pub behavior: Option<Box<dyn Behavior>>,
	pub children: Vec<ElementSettings>,
}

pub struct Gui {
	pub elements: Elements,
	pub behaviors: HashMap<ElementId, Box<dyn Behavior>>,
	parent_stack: Vec<ElementId>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			elements: Elements::new(),
			behaviors: HashMap::new(),
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
			hover_position: None,
			held: enum_map! {
				MouseButton::Left => false,
				MouseButton::Middle => false,
				MouseButton::Right => false,
			},
		});
		if let Some(behavior) = settings.behavior {
			self.behaviors.insert(id, behavior);
		}
		self.parent_stack.push(id);
		for child_settings in settings.children {
			self.add(child_settings);
		}
		self.parent_stack.pop();
		id
	}

	pub fn emit(&mut self, event: &Event, element_id: Option<ElementId>) {
		if let Some(id) = element_id {
			let behavior = self.behaviors.get_mut(&id).unwrap();
			behavior.on(event, &mut self.elements);
		} else {
			for (_, behavior) in &mut self.behaviors {
				behavior.on(event, &mut self.elements);
			}
		}
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
			if self.update_hover_state(&node.children, relative_mouse_position, blocked) {
				blocked = true;
			}
			let mut element = self.elements.get_mut(node.element_id);
			let hovered_previous = element.hover_position.is_some();
			let hovered = !blocked && element.rect.contains_point(mouse_position);
			if hovered {
				element.hover_position = Some(relative_mouse_position);
				blocked = true;
			} else {
				element.hover_position = None;
			}
			if hovered && !hovered_previous {
				self.emit(
					&Event::Hover(node.element_id, relative_mouse_position),
					Some(node.element_id),
				);
			}
			if hovered_previous && !hovered {
				self.emit(&Event::Unhover(node.element_id), Some(node.element_id));
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
			if element.is_hovered() {
				element.held[button] = true;
				events.push((Event::Press(id, button), Some(id)));
			}
		}
		for (event, id) in &events {
			self.emit(event, *id);
		}
	}

	pub fn on_release_mouse_button(&mut self, button: MouseButton) {
		let mut events = vec![];
		for (id, element) in self.elements.iter_mut() {
			if element.held[button] {
				element.held[button] = false;
				events.push((Event::Release(id, button), Some(id)));
				if element.is_hovered() {
					events.push((Event::Click(id, button), Some(id)));
				}
			}
		}
		for (event, id) in &events {
			self.emit(event, *id);
		}
	}

	fn draw_nodes(&self, nodes: &Vec<TreeNode>, canvas: &mut Canvas) {
		for node in nodes {
			let element = self.elements.get(node.element_id);
			let behavior = self.behaviors.get(&node.element_id);
			if let Some(behavior) = behavior {
				behavior.draw_below(element, canvas);
			}
			canvas.push_translation(element.rect.position);
			self.draw_nodes(&node.children, canvas);
			canvas.pop_translation();
			if let Some(behavior) = behavior {
				behavior.draw_above(element, canvas);
			}
		}
	}

	fn layout(&mut self, nodes: &Vec<TreeNode>) {
		for node in nodes {
			self.layout(&node.children);
			if let Some(behavior) = self.behaviors.get_mut(&node.element_id) {
				behavior.layout(&mut self.elements, node.element_id);
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
