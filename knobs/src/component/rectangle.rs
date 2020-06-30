use super::Component;
use crate::{
	canvas::{Canvas, Color, ShapeStyle},
	event::{Event, EventQueue},
	gui::{Element, Elements},
	resources::Resources,
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

impl Component for Rectangle {
	fn on(&mut self, event: &Event, _elements: &mut Elements, _event_queue: &mut EventQueue) {
		match event {
			Event::Hover(id, position) => println!("Event::Hover({:?}, {:?})", id, position),
			Event::Unhover(id) => println!("Event::Unhover({:?})", id),
			Event::Press(id, button) => println!("Event::Press({:?}, {:?})", id, button),
			Event::Release(id, button) => println!("Event::Release({:?}, {:?})", id, button),
			Event::Click(id, button) => println!("Event::Click({:?}, {:?})", id, button),
			Event::Drag(id, button, position, delta) => println!(
				"Event::Drag({:?}, {:?}, {}, {})",
				id, button, position, delta
			),
		}
	}

	fn draw_below(&self, element: &Element, canvas: &mut Canvas, _resources: &Resources) {
		if let Some(color) = self.fill {
			canvas.draw_rectangle(element.rect, ShapeStyle::Fill(color));
		}
	}

	fn draw_above(&self, element: &Element, canvas: &mut Canvas, _resources: &Resources) {
		if let Some((width, color)) = self.stroke {
			if element.hovered {
				canvas.draw_rectangle(element.rect, ShapeStyle::Stroke(width * 2.0, color));
			} else {
				canvas.draw_rectangle(element.rect, ShapeStyle::Stroke(width, color));
			}
		}
	}
}