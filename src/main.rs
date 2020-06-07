mod backend;
mod gui;

use backend::ggez::{GgezBackend, GgezBackendOptions};
use ggez::graphics;
use gui::{
	canvas::{Alignment, ArcKind, Canvas, Color, DrawMode, Style, TextStyle},
	control::{behavior::ControlBehavior, Control, ControlSettings},
	rectangle::Rectangle,
	Controls, Parameters,
};
use std::{collections::HashMap, f32::consts::PI};

struct Knob {
	parameter_index: i32,
}

impl ControlBehavior for Knob {
	fn on(&mut self, event: gui::Event, _controls: &mut Controls, parameters: &mut Parameters) {
		match event {
			gui::Event::Drag(id, button, x, y, dx, dy) => {
				parameters.set(
					self.parameter_index,
					(parameters.get(self.parameter_index) - dy / 100.0)
						.max(0.0)
						.min(1.0),
				);
			}
			_ => {}
		}
	}

	fn draw(&self, control: &Control, parameters: &Parameters, canvas: &mut Canvas) {
		let center = control.rectangle.get_center();
		let radius = control.rectangle.height / 2.0;
		let nub_angle = 0.75 * PI + parameters.get(self.parameter_index) * 1.5 * PI;
		let style = Style {
			mode: DrawMode::Stroke(4.0),
			color: Color::new(1.0, 1.0, 1.0, 1.0),
		};
		canvas.draw_circle(center, radius, style);
		canvas.draw_arc(
			ArcKind::Open,
			center,
			radius * 0.75,
			nub_angle - 0.25,
			nub_angle + 0.25,
			style,
		);
		canvas.draw_text(
			"Label".to_string(),
			center.shifted(0.0, radius + 10.0),
			TextStyle {
				font_id: 0,
				size: 20.0,
				horizontal_alignment: Alignment::Middle,
				vertical_alignment: Alignment::Start,
				color: Color::new(1.0, 1.0, 1.0, 1.0),
			},
		)
	}
}

struct MainState {
	parameters: HashMap<i32, f32>,
	backend: GgezBackend,
}

impl MainState {
	pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
		let mut backend = GgezBackend::new(
			ctx,
			GgezBackendOptions {
				fonts: vec![include_bytes!("resources/Roboto-Regular.ttf").to_vec()],
			},
		)?;
		backend.gui.add_control(ControlSettings {
			rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
			height: 0,
			behaviors: vec![Box::new(Knob { parameter_index: 0 })],
		});
		backend.gui.on_change_parameter(0, 0.5);
		let mut parameters = HashMap::new();
		parameters.insert(0, 0.5);
		Ok(Self {
			backend,
			parameters,
		})
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		Ok(())
	}

	fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, dx: f32, dy: f32) {
		self.backend.mouse_motion_event(ctx, x, y, dx, dy);
	}

	fn mouse_button_down_event(
		&mut self,
		ctx: &mut ggez::Context,
		button: ggez::event::MouseButton,
		x: f32,
		y: f32,
	) {
		self.backend.mouse_button_down_event(ctx, button, x, y);
	}

	fn mouse_button_up_event(
		&mut self,
		ctx: &mut ggez::Context,
		button: ggez::event::MouseButton,
		x: f32,
		y: f32,
	) {
		self.backend.mouse_button_up_event(ctx, button, x, y);
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, graphics::BLACK);
		self.backend.draw(ctx)?;
		let text = graphics::Text::new(self.parameters.get(&0).unwrap().to_string());
		graphics::draw(ctx, &text, graphics::DrawParam::new())?;
		graphics::present(ctx)?;
		for (index, value) in self.backend.gui.get_parameter_changes() {
			self.parameters.insert(index, value);
			self.backend.gui.on_change_parameter(index, value);
		}
		Ok(())
	}
}

fn main() -> ggez::GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("audio-gui", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx)?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)?;
	Ok(())
}
