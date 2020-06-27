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

	fn distribute(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
	) {
		match self.distribution {
			Distribution::Stack(spacing) => {
				let mut next_x = 0.0;
				for id in elements.children_of(element_id) {
					let mut element = elements.get_mut(id);
					element.rect.position.x = next_x;
					next_x = element.rect.position.x + element.rect.size.x + spacing;
				}
				let element = elements.get_mut(element_id);
				element.rect.size.x = next_x - spacing;
			}
			Distribution::SpaceEvenly => {}
			Distribution::AlignToGrid => {}
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
