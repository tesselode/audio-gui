//! # Knobby
//!
//! *Knobby* (working title) is a platform-agnostic GUI library
//! specifically designed for audio plugins written in Rust.
//! It defines a pleasant interface for plugin developers to lay out
//! widgets that control the parameters of a plugin. It does not
//! actually draw anything to the screen - that's the job of a backend
//! crate.
//!
//! Basic usage involves defining behaviors for controls, like this
//! Knob behavior...
//!
//! ```rust
//! struct Knob {
//! 	parameter_index: i32,
//! 	parameter_value: f32,
//! }
//!
//! impl Knob {
//! 	fn new(parameter_index: i32) -> Self {
//! 		Self {
//! 			parameter_index,
//! 			parameter_value: 0.0,
//! 		}
//! 	}
//! }
//!
//! impl Behavior<CustomEvent> for Knob {
//! 	fn on(
//! 		&mut self,
//! 		event: Event<CustomEvent>,
//! 		_controls: &mut Controls,
//! 		event_queue: &mut EventQueue<CustomEvent>,
//! 	) {
//! 		match event {
//! 			// modify the parameter value when the knob is dragged up or down
//! 			Event::Drag(id, button, x, y, dx, dy) => {
//! 				event_queue.push(Event::SetParameter(
//! 					self.parameter_index,
//! 					(self.parameter_value - dy / 100.0).max(0.0).min(1.0),
//! 				));
//! 			}
//! 			// update the internal value when the parameter is changed
//! 			Event::SetParameter(index, value) => {
//! 				if index == self.parameter_index {
//! 					self.parameter_value = value;
//! 				}
//! 			}
//! 			_ => {}
//! 		}
//! 	}
//!
//! 	fn draw(&self, control: &Control, canvas: &mut Canvas) {
//! 		let center = control.rectangle.get_center();
//! 		let radius = control.rectangle.height / 2.0;
//! 		let nub_angle = 0.75 * PI + self.parameter_value * 1.5 * PI;
//! 		let style = Style {
//! 			mode: DrawMode::Stroke(4.0),
//! 			color: Color::new(1.0, 1.0, 1.0, 1.0),
//! 		};
//! 		// draw the outside of the circle
//! 		canvas.draw_circle(center, radius, style);
//! 		// draw the "nub" representing where the knob is pointing
//! 		canvas.draw_arc(
//! 			ArcKind::Open,
//! 			center,
//! 			radius * 0.75,
//! 			nub_angle - 0.25,
//! 			nub_angle + 0.25,
//! 			style,
//! 		);
//! 		// draw a label below the knob
//! 		canvas.draw_text(
//! 			"Label".to_string(),
//! 			center.shifted(0.0, radius + 10.0),
//! 			TextStyle {
//! 				font_id: 0,
//! 				size: 20.0,
//! 				horizontal_alignment: Alignment::Middle,
//! 				vertical_alignment: Alignment::Start,
//! 				color: Color::new(1.0, 1.0, 1.0, 1.0),
//! 			},
//! 		)
//! 	}
//! }
//! ```
//!
//! ...creating a GUI to hold the controls...
//!
//! ```rust
//! let mut gui = Gui::new();
//! ```
//!
//! ...and adding controls to the GUI with the appropriate behaviors.
//!
//! ```rust
//! gui.add_control(
//! 	ControlSettings {
//! 		rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
//! 		height: 0,
//! 	},
//! 	vec![Box::new(Knob::new(0))],
//! );
//! ```

pub mod behavior;
pub mod canvas;
pub mod control;
pub mod event;
pub mod geometry;
pub mod gui;
pub mod input;
