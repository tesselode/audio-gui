pub mod rectangle;

use crate::{canvas::Canvas, gui::Element};

pub trait Behavior {
	fn draw_below(&self, _element: &Element, _canvas: &mut Canvas) {}
	fn draw_above(&self, _element: &Element, _canvas: &mut Canvas) {}
}
