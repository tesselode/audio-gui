use ggez::graphics;
use knobby::{
	behavior::Behavior,
	canvas::{Alignment, ArcKind, Canvas, Color, DrawMode, Style, TextStyle},
	control::{Control, ControlSettings},
	event::Event,
	geometry::{Point, Rectangle},
	gui::{Controls, EventQueue, Resources},
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
	fn draw(&self, control: &Control, _resources: &Resources, canvas: &mut Canvas) {
		canvas.draw_rectangle(
			control.rectangle,
			Style {
				mode: DrawMode::Stroke(2.0),
				color: self.color,
			},
		);
	}
}

struct Text {
	font_index: usize,
	text: String,
	size: f32,
}

impl Text {
	pub fn new(font_index: usize, text: String, size: f32) -> Self {
		Self {
			font_index,
			text,
			size,
		}
	}
}

impl Behavior<CustomEvent> for Text {
	fn draw(&self, control: &Control, _resources: &Resources, canvas: &mut Canvas) {
		canvas.draw_text(
			self.text.clone(),
			Point::new(control.rectangle.x, control.rectangle.y),
			TextStyle {
				font_id: self.font_index,
				size: self.size,
				horizontal_alignment: Alignment::Start,
				vertical_alignment: Alignment::Start,
				color: Color::new(1.0, 1.0, 1.0, 1.0),
			},
		)
	}
}

struct MainState {
	backend: GgezBackend<CustomEvent>,
}

impl MainState {
	pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
		let mut backend = GgezBackend::new();
		backend.load_font(ctx, include_bytes!("resources/Roboto-Regular.ttf"))?;
		let rect_1 = Rectangle::around_text(
			backend.gui.resources.get_font(0).unwrap(),
			"Hello world!",
			40.0,
			Point::new(100.0, 200.0),
		);
		backend.gui.add_control(
			ControlSettings {
				rectangle: rect_1,
				height: 0,
			},
			vec![
				Box::new(Outline::new(Color::new(1.0, 1.0, 1.0, 1.0))),
				Box::new(Text::new(0, "Hello world!".into(), 40.0)),
			],
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
