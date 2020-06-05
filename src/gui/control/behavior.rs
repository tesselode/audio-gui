use crate::gui::{ControlId, Controls, Event};

pub trait ControlBehavior {
	fn on(&mut self, _event: Event, _controls: &mut Controls, _id: &ControlId);
}
