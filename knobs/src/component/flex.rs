use super::Component;

pub enum Axis {
	Horizontal,
	Vertical,
}

pub enum Distribution {
	Stack(f32),
	SpaceEvenly,
	AlignToGrid,
}

pub struct Flex {
	main_axis: Axis,
	distribution: Distribution,
	cross_axis_alignment: f32,
}

impl Flex {
	pub fn new(main_axis: Axis, distribution: Distribution, cross_axis_alignment: f32) -> Self {
		Self {
			main_axis,
			distribution,
			cross_axis_alignment,
		}
	}

	fn get_space_per_element(
		&self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
	) -> f32 {
		match self.main_axis {
			Axis::Horizontal => {
				let mut total_space = elements.get(element_id).rect.get_width();
				for id in elements.children_of(element_id) {
					total_space -= elements.get(id).rect.get_width();
				}
				total_space / (elements.children_of(element_id).len() - 1) as f32
			}
			Axis::Vertical => {
				let mut total_space = elements.get(element_id).rect.get_height();
				for id in elements.children_of(element_id) {
					total_space -= elements.get(id).rect.get_height();
				}
				total_space / (elements.children_of(element_id).len() - 1) as f32
			}
		}
	}

	fn stack(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
		spacing: f32,
	) {
		match self.main_axis {
			Axis::Horizontal => {
				let mut next_x = 0.0;
				for id in elements.children_of(element_id) {
					let mut element = elements.get_mut(id);
					element.rect.position.x = next_x;
					next_x = element.rect.position.x + element.rect.size.x + spacing;
				}
				let element = elements.get_mut(element_id);
				element.rect.size.x = next_x - spacing;
			}
			Axis::Vertical => {
				let mut next_y = 0.0;
				for id in elements.children_of(element_id) {
					let mut element = elements.get_mut(id);
					element.rect.position.y = next_y;
					next_y = element.rect.position.y + element.rect.size.y + spacing;
				}
				let element = elements.get_mut(element_id);
				element.rect.size.y = next_y - spacing;
			}
		}
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
				let space_per_element = self.get_space_per_element(elements, element_id);
				self.stack(elements, element_id, space_per_element);
			}
			Distribution::AlignToGrid => match self.main_axis {
				Axis::Horizontal => {
					let child_ids = elements.children_of(element_id);
					for i in 0..child_ids.len() {
						let origin = i as f32 / (child_ids.len() - 1) as f32;
						let child_id = child_ids.get(i).unwrap();
						let target_x = elements.get(element_id).rect.get_width() * origin;
						let child = elements.get_mut(*child_id);
						child.rect.set_x(target_x, origin);
					}
				}
				Axis::Vertical => {
					let child_ids = elements.children_of(element_id);
					for i in 0..child_ids.len() {
						let origin = i as f32 / (child_ids.len() - 1) as f32;
						let child_id = child_ids.get(i).unwrap();
						let target_y = elements.get(element_id).rect.get_height() * origin;
						let child = elements.get_mut(*child_id);
						child.rect.set_y(target_y, origin);
					}
				}
			},
		}
	}

	fn align(&mut self, elements: &mut crate::gui::Elements, element_id: crate::gui::ElementId) {
		match self.main_axis {
			Axis::Horizontal => {
				if elements.get(element_id).rect.get_height() == 0.0 {
					if let Some(max_height) = elements
						.children_of(element_id)
						.iter()
						.map(|id| elements.get(*id).rect.get_height())
						.max_by(|a, b| a.partial_cmp(b).unwrap())
					{
						elements.get_mut(element_id).rect.set_height(max_height);
					}
				}
				let target_y =
					elements.get(element_id).rect.get_height() * self.cross_axis_alignment;
				for id in elements.children_of(element_id) {
					let element = elements.get_mut(id);
					element.rect.set_y(target_y, self.cross_axis_alignment);
				}
			}
			Axis::Vertical => {
				if elements.get(element_id).rect.get_width() == 0.0 {
					if let Some(max_width) = elements
						.children_of(element_id)
						.iter()
						.map(|id| elements.get(*id).rect.get_width())
						.max_by(|a, b| a.partial_cmp(b).unwrap())
					{
						elements.get_mut(element_id).rect.set_width(max_width);
					}
				}
				let target_x =
					elements.get(element_id).rect.get_width() * self.cross_axis_alignment;
				for id in elements.children_of(element_id) {
					let element = elements.get_mut(id);
					element.rect.set_x(target_x, self.cross_axis_alignment);
				}
			}
		}
	}
}

impl Component for Flex {
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
