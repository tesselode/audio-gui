use crate::{behavior::Behavior, canvas::Canvas, geometry::rect::Rect};
use std::{iter::Zip, ops::Range, slice::Iter};

#[derive(Debug)]
pub struct Element {
	pub rect: Rect,
	pub height: f32,
	pub parent_index: Option<usize>,
}

#[derive(Debug)]
pub struct TreeNode {
	element_index: usize,
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

	pub fn iter(&self) -> Zip<Range<usize>, Iter<Element>> {
		(0..self.elements.len()).zip(self.elements.iter())
	}

	pub fn get_tree(&self, parent_id: Option<usize>) -> Vec<TreeNode> {
		let mut nodes = vec![];
		for (element_index, element) in self.iter() {
			if element.parent_index == parent_id {
				nodes.push(TreeNode {
					element_index,
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
	pub behaviors: Vec<Option<Box<dyn Behavior>>>,
	parent_stack: Vec<usize>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			elements: Elements::new(),
			behaviors: vec![],
			parent_stack: vec![],
		}
	}

	pub fn add(&mut self, settings: ElementSettings) -> usize {
		let id = self.elements.elements.len();
		self.elements.elements.push(Element {
			rect: settings.rect,
			height: settings.height,
			parent_index: match self.parent_stack.last() {
				Some(index) => Some(*index),
				None => None,
			},
		});
		self.behaviors.push(settings.behavior);
		self.parent_stack.push(id);
		for child_settings in settings.children {
			self.add(child_settings);
		}
		self.parent_stack.pop();
		id
	}

	fn draw_nodes(&self, nodes: &Vec<TreeNode>, canvas: &mut Canvas) {
		for node in nodes {
			let element = &self.elements.elements.get(node.element_index).unwrap();
			let behavior = &self.behaviors.get(node.element_index).unwrap();
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

	pub fn draw(&self) -> Canvas {
		let mut canvas = Canvas::new();
		self.draw_nodes(&self.elements.get_tree(None), &mut canvas);
		canvas
	}
}
