use super::Behavior;

pub enum Distribution {
	Stack(f32),
	SpaceEvenly,
	AlignToGrid,
}

pub struct Row {
	distribution: Distribution,
	cross_axis_alignment: f32,
}

impl Row {
	pub fn new(distribution: Distribution, cross_axis_alignment: f32) -> Self {
		Self {
			distribution,
			cross_axis_alignment,
		}
	}

	fn stack(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
		spacing: f32,
	) {
		let mut next_x = 0.0;
		for id in elements.children_of(element_id) {
			let mut element = elements.get_mut(id);
			element.rect.position.x = next_x;
			next_x = element.rect.position.x + element.rect.size.x + spacing;
		}
		let element = elements.get_mut(element_id);
		element.rect.size.x = next_x - spacing;
	}

	fn distribute(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
	) {
		match self.distribution {
			Distribution::Stack(spacing) => {
				self.stack(elements, element_id, spacing);
			}
			Distribution::SpaceEvenly => {
				let mut total_space = elements.get(element_id).rect.width();
				for id in elements.children_of(element_id) {
					total_space -= elements.get(id).rect.width();
				}
				let space_per_element =
					total_space / (elements.children_of(element_id).len() - 1) as f32;
				self.stack(elements, element_id, space_per_element);
			}
			Distribution::AlignToGrid => {
				let child_ids = elements.children_of(element_id);
				for i in 0..child_ids.len() {
					let origin = i as f32 / (child_ids.len() - 1) as f32;
					let child_id = child_ids.get(i).unwrap();
					let target_x = elements.get(element_id).rect.width() * origin;
					let child = elements.get_mut(*child_id);
					child.rect.set_x(target_x, origin);
				}
			}
		}
	}

	fn align(&mut self, elements: &mut crate::gui::Elements, element_id: crate::gui::ElementId) {
		if elements.get(element_id).rect.height() == 0.0 {
			if let Some(max_height) = elements
				.children_of(element_id)
				.iter()
				.map(|id| elements.get(*id).rect.height())
				.max_by(|a, b| a.partial_cmp(b).unwrap())
			{
				elements.get_mut(element_id).rect.set_height(max_height);
			}
		}
		let target_y = elements.get(element_id).rect.height() * self.cross_axis_alignment;
		for id in elements.children_of(element_id) {
			let element = elements.get_mut(id);
			element.rect.set_y(target_y, self.cross_axis_alignment);
		}
	}
}

impl Behavior for Row {
	fn layout(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
		_resources: &crate::resources::Resources,
	) {
		self.distribute(elements, element_id);
		self.align(elements, element_id);
	}
}
