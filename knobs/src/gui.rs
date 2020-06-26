use crate::geometry::rect::Rect;

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
}
