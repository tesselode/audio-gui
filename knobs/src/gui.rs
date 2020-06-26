use crate::{
	canvas::{Canvas, Color, ShapeStyle},
	geometry::rect::Rect,
};

#[derive(Debug)]
pub struct Element {
	pub rect: Rect,
	pub height: f32,
}

#[derive(Debug)]
pub struct Elements {
	elements: Vec<Element>,
}

impl Elements {
	pub fn new() -> Self {
		Self { elements: vec![] }
	}
}

#[derive(Default)]
pub struct ElementSettings {
	pub rect: Rect,
	pub height: f32,
}

pub struct Gui {
	pub elements: Elements,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			elements: Elements::new(),
		}
	}

	pub fn add(&mut self, settings: ElementSettings) -> usize {
		let id = self.elements.elements.len();
		self.elements.elements.push(Element {
			rect: settings.rect,
			height: settings.height,
		});
		id
	}

	pub fn draw(&self) -> Canvas {
		let mut canvas = Canvas::new();
		for element in &self.elements.elements {
			canvas.draw_rectangle(
				element.rect,
				ShapeStyle::Stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
			)
		}
		canvas
	}
}
