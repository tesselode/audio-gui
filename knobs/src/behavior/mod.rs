pub mod rectangle;

use crate::{
	canvas::Canvas,
	event::{Event, EventQueue},
	gui::{Element, ElementId, Elements},
};

pub trait Behavior {
	fn on(&mut self, _event: &Event, _elements: &mut Elements, _event_queue: &mut EventQueue) {}

	fn layout(&mut self, _elements: &mut Elements, _element_id: ElementId) {}

	fn draw_below(&self, _element: &Element, _canvas: &mut Canvas) {}
	fn draw_above(&self, _element: &Element, _canvas: &mut Canvas) {}
}
