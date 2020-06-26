use crate::{geometry::vector::Vector, gui::ElementId, input::MouseButton};

pub enum Event {
	Hover(ElementId, Vector),
	Unhover(ElementId),
	Press(ElementId, MouseButton),
	Release(ElementId, MouseButton),
	Click(ElementId, MouseButton),
}
