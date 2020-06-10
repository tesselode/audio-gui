use ggez::graphics;
use knobby::{
	behavior::Behavior,
	canvas::{Alignment, ArcKind, Canvas, Color, DrawMode, Style, TextStyle},
	control::{Control, ControlSettings},
	event::Event,
	geometry::{Point, Rectangle},
	gui::{Controls, EventQueue},
};
use knobby_ggez_backend::GgezBackend;
use std::{collections::HashMap, f32::consts::PI};

#[derive(Copy, Clone)]
enum CustomEvent {
	Test,
}

struct Outline {
	color: Color,
}

impl Outline {
	pub fn new(color: Color) -> Self {
		Self { color }
	}
}

impl Behavior<CustomEvent> for Outline {
	fn draw(&self, control: &Control, canvas: &mut Canvas) {
		canvas.draw_rectangle(
			control.rectangle,
			Style {
				mode: DrawMode::Stroke(2.0),
				color: self.color,
			},
		);
	}
}

struct MainState {
	backend: GgezBackend<CustomEvent>,
}

impl MainState {
	pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
		let mut backend = GgezBackend::new();
		let rect_1 = Rectangle::new(100.0, 100.0, 50.0, 50.0);
		let rect_2 = rect_1.pad(10.0);
		backend.gui.add_control(
			ControlSettings {
				rectangle: rect_1,
				height: 0,
			},
			vec![Box::new(Outline::new(Color::new(1.0, 1.0, 1.0, 1.0)))],
		);
		backend.gui.add_control(
			ControlSettings {
				rectangle: rect_2,
				height: 0,
			},
			vec![Box::new(Outline::new(Color::new(1.0, 1.0, 0.0, 1.0)))],
		);
		Ok(Self { backend })
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

	fn key_down_event(
		&mut self,
		ctx: &mut ggez::Context,
		keycode: ggez::event::KeyCode,
		_keymods: ggez::event::KeyMods,
		_repeat: bool,
	) {
		match keycode {
			ggez::event::KeyCode::Space => {
				self.backend
					.gui
					.emit(Event::Custom(CustomEvent::Test), None);
			}
			_ => {}
		}
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, graphics::BLACK);
		self.backend.draw(ctx)?;
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> ggez::GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("audio-gui", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx)?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)?;
	Ok(())
}
