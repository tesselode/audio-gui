use crate::{gui::ControlId, input::MouseButton};

#[derive(Copy, Clone)]
pub enum Event<CustomEvent> {
	Hover(ControlId, f32, f32),
	Unhover(ControlId),
	Press(ControlId, MouseButton, f32, f32),
	Release(ControlId, MouseButton, f32, f32),
	Click(ControlId, MouseButton, f32, f32),
	Drag(ControlId, MouseButton, f32, f32, f32, f32),
	SetParameter(i32, f32),
	ResetParameter(i32),
	Custom(CustomEvent),
}
