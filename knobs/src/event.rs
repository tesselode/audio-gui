use crate::{geometry::vector::Vector, gui::ElementId};

pub enum Event {
	Hover(ElementId, Vector),
	Unhover(ElementId),
}
