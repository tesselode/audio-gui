use super::Behavior;
use crate::{canvas::Color, geometry::vector::Vector, resources::FontId, util};

pub struct Text {
	font_id: FontId,
	text: String,
	scale: Vector,
	color: Color,
}

impl Text {
	pub fn new(font_id: FontId, text: &str, scale: Vector) -> Self {
		Self {
			font_id,
			text: text.into(),
			scale,
			color: Color::new(1.0, 1.0, 1.0, 1.0),
		}
	}

	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}
}

impl Behavior for Text {
	fn layout(
		&mut self,
		elements: &mut crate::gui::Elements,
		element_id: crate::gui::ElementId,
		resources: &crate::resources::Resources,
	) {
		let element = elements.get_mut(element_id);
		if element.rect.size == Vector::zero() {
			let font = resources.get_font(self.font_id);
			element.rect.size = util::size_of_text(font, &self.text, self.scale);
		}
	}

	fn draw_below(
		&self,
		element: &crate::gui::Element,
		canvas: &mut crate::canvas::Canvas,
		resources: &crate::resources::Resources,
	) {
		let font = resources.get_font(self.font_id);
		let text_size = util::size_of_text(font, &self.text, self.scale);
		canvas.draw_text(
			self.font_id,
			&self.text,
			element.rect.position,
			Vector::new(
				self.scale.x * element.rect.width() / text_size.x,
				self.scale.y * element.rect.height() / text_size.y,
			),
			self.color,
		);
	}
}
