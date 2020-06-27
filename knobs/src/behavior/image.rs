use super::Behavior;
use crate::{canvas::Color, geometry::vector::Vector, resources::ImageId};

pub struct Image {
	image_id: ImageId,
	default_scale: Vector,
}

impl Image {
	pub fn new(id: ImageId) -> Self {
		Self {
			image_id: id,
			default_scale: Vector::new(1.0, 1.0),
		}
	}

	pub fn default_scale(self, scale: Vector) -> Self {
		Self {
			default_scale: scale,
			..self
		}
	}
}

impl Behavior for Image {
	fn layout(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
		resources: &crate::resources::Resources,
	) {
		let element = elements.get_mut(element_id);
		if element.rect.size == Vector::zero() {
			let image = resources.get_image(self.image_id);
			element.rect.size = Vector::new(
				image.width() as f32 * self.default_scale.x,
				image.height() as f32 * self.default_scale.y,
			);
		}
	}

	fn draw_below(
		&self,
		element: &crate::gui::Element,
		canvas: &mut crate::canvas::Canvas,
		resources: &crate::resources::Resources,
	) {
		let image = resources.get_image(self.image_id);
		canvas.draw_image(
			self.image_id,
			element.rect.position,
			Vector::new(
				element.rect.size.x / image.width() as f32,
				element.rect.size.y / image.height() as f32,
			),
			Color::new(1.0, 1.0, 1.0, 1.0),
		);
	}
}
