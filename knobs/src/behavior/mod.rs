pub mod flex;
pub mod image;
pub mod rectangle;
pub mod text;

pub use self::image::Image;
pub use flex::Flex;
pub use rectangle::Rectangle;
pub use text::Text;

use crate::{
	canvas::Canvas,
	event::{Event, EventQueue},
	gui::{Element, ElementId, Elements},
	resources::Resources,
};

pub trait Behavior {
	fn on(&mut self, _event: &Event, _elements: &mut Elements, _event_queue: &mut EventQueue) {}

	fn layout(&mut self, _elements: &mut Elements, _element_id: ElementId, _resources: &Resources) {
	}

	fn draw_below(&self, _element: &Element, _canvas: &mut Canvas, _resources: &Resources) {}
	fn draw_above(&self, _element: &Element, _canvas: &mut Canvas, _resources: &Resources) {}
}
