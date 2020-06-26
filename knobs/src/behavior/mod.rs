pub mod flex;
pub mod rectangle;

use crate::{
	canvas::Canvas,
	gui::{Element, ElementId, Elements},
};

pub trait Behavior {
	fn layout(&mut self, _elements: &mut Elements, _element_id: ElementId) {}

	fn draw_below(&self, _element: &Element, _canvas: &mut Canvas) {}
	fn draw_above(&self, _element: &Element, _canvas: &mut Canvas) {}
}
