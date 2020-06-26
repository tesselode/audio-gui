use super::{Element, ElementId, ElementIdIter};
use std::{
	iter::Zip,
	slice::{Iter, IterMut},
};

#[derive(Debug)]
pub struct TreeNode {
	pub element_id: ElementId,
	pub children: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct Elements {
	pub(super) elements: Vec<Element>,
}

impl Elements {
	pub fn new() -> Self {
		Self { elements: vec![] }
	}

	pub(super) fn next_element_id(&self) -> ElementId {
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
