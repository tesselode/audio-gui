use crate::{behavior::Behavior, canvas::Canvas, geometry::rect::Rect};

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
	pub behaviors: Vec<Box<dyn Behavior>>,
}

impl Gui {
	pub fn new() -> Self {
		Self {
			elements: Elements::new(),
			behaviors: vec![],
		}
	}

	pub fn add(&mut self, settings: ElementSettings, behavior: Box<dyn Behavior>) -> usize {
		let id = self.elements.elements.len();
		self.elements.elements.push(Element {
			rect: settings.rect,
			height: settings.height,
		});
		self.behaviors.push(behavior);
		id
	}

	pub fn draw(&self) -> Canvas {
		let mut canvas = Canvas::new();
		for i in 0..self.elements.elements.len() {
			let element = &self.elements.elements.get(i).unwrap();
			let behavior = &self.behaviors.get(i).unwrap();
			behavior.draw_below(element, &mut canvas);
			behavior.draw_above(element, &mut canvas);
		}
		canvas
	}
}
