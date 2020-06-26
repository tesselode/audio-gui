use super::Behavior;
use crate::gui::{ElementId, Elements};

pub enum Axis {
	Horizontal,
	Vertical,
}

pub struct Flex {
	main_axis: Axis,
}

impl Flex {
	pub fn new(main_axis: Axis) -> Self {
		Self { main_axis }
	}
}

impl Behavior for Flex {
	fn layout(&mut self, elements: &mut Elements, element_id: ElementId) {
		match self.main_axis {
			Axis::Horizontal => {
				let mut last_x = elements.get(element_id).rect.position.x;
				for child_id in elements.children_of(element_id) {
					let mut child = elements.get_mut(child_id);
					child.rect.position.x = last_x;
					last_x = child.rect.position.x + child.rect.size.x;
				}
			}
			Axis::Vertical => {}
		}
	}
}
