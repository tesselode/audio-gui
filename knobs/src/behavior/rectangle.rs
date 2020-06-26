use super::Behavior;
use crate::{
	canvas::{Canvas, Color, ShapeStyle},
	gui::Element,
};

pub struct Rectangle {
	fill: Option<Color>,
	stroke: Option<(f32, Color)>,
}

impl Rectangle {
	pub fn new() -> Self {
		Self {
			fill: None,
			stroke: None,
		}
	}

	pub fn fill(self, color: Color) -> Self {
		Self {
			fill: Some(color),
			..self
		}
	}

	pub fn stroke(self, width: f32, color: Color) -> Self {
		Self {
			stroke: Some((width, color)),
			..self
		}
	}
}

impl Behavior for Rectangle {
	fn draw_below(&self, element: &Element, canvas: &mut Canvas) {
		if let Some(color) = self.fill {
			canvas.draw_rectangle(element.rect, ShapeStyle::Fill(color));
		}
	}

	fn draw_above(&self, element: &Element, canvas: &mut Canvas) {
		if let Some((width, color)) = self.stroke {
			if element.hover_position.is_some() {
				canvas.draw_rectangle(element.rect, ShapeStyle::Stroke(width * 2.0, color));
			} else {
				canvas.draw_rectangle(element.rect, ShapeStyle::Stroke(width, color));
			}
		}
	}
}
