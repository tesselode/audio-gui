use ggez::graphics;
use knobby::{
	behavior::Behavior,
	canvas::{Alignment, ArcKind, Canvas, Color, DrawMode, Style, TextStyle},
	control::{Control, ControlSettings},
	event::Event,
	geometry::Rectangle,
	gui::{Controls, EventQueue},
};
use knobby_ggez_backend::GgezBackend;
use std::{collections::HashMap, f32::consts::PI};

#[derive(Copy, Clone)]
enum CustomEvent {
	Test,
}

struct Knob {
	parameter_index: i32,
	parameter_value: f32,
}

impl Knob {
	fn new(parameter_index: i32) -> Self {
		Self {
			parameter_index,
			parameter_value: 0.0,
		}
	}
}

impl Behavior<CustomEvent> for Knob {
	fn on(
		&mut self,
		event: Event<CustomEvent>,
		_controls: &mut Controls,
		event_queue: &mut EventQueue<CustomEvent>,
	) {
		match event {
			Event::Drag(id, button, x, y, dx, dy) => {
				event_queue.push(Event::SetParameter(
					self.parameter_index,
					(self.parameter_value - dy / 100.0).max(0.0).min(1.0),
				));
			}
			Event::SetParameter(index, value) => {
				if index == self.parameter_index {
					self.parameter_value = value;
				}
			}
			Event::Custom(custom_event) => match custom_event {
				CustomEvent::Test => {
					println!("hi!");
				}
			},
			_ => {}
		}
	}

	fn draw(&self, control: &Control, canvas: &mut Canvas) {
		let center = control.rectangle.get_center();
		let radius = control.rectangle.height / 2.0;
		let nub_angle = 0.75 * PI + self.parameter_value * 1.5 * PI;
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
	backend: GgezBackend<CustomEvent>,
}

impl MainState {
	pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
		let mut backend = GgezBackend::new();
		backend.load_font(ctx, include_bytes!("resources/Roboto-Regular.ttf"))?;
		backend.gui.add_control(
			ControlSettings {
				rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
				height: 0,
			},
			vec![Box::new(Knob::new(0))],
		);
		backend.gui.emit(Event::SetParameter(0, 0.5), None);
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
		let text = graphics::Text::new(self.parameters.get(&0).unwrap().to_string());
		graphics::draw(ctx, &text, graphics::DrawParam::new())?;
		graphics::present(ctx)?;
		for event in self.backend.gui.drain_events() {
			match event {
				Event::SetParameter(index, value) => {
					self.parameters.insert(index, value);
					self.backend.gui.emit(event, None);
				}
				_ => {}
			}
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
