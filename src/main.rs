mod backend;
mod gui;

use backend::ggez::GgezBackend;
use ggez::graphics;
use gui::{
	control::{behavior::ControlBehavior, ControlSettings},
	rectangle::Rectangle,
	ControlId, Controls,
};

struct TestBehavior;

impl ControlBehavior for TestBehavior {
	fn on(&mut self, event: gui::Event, _controls: &mut Controls, _id: &ControlId) {
		match event {
			gui::Event::Hover(x, y) => {
				println!("Hover: {}, {}", x, y);
			}
			gui::Event::Unhover => {
				println!("Unhover");
			}
			gui::Event::Press(mouse_button, x, y) => {
				println!("Press: {:?}, {}, {}", mouse_button, x, y);
			}
			gui::Event::Release(mouse_button, x, y) => {
				println!("Release: {:?}, {}, {}", mouse_button, x, y);
			}
			gui::Event::Click(mouse_button, x, y) => {
				println!("Click: {:?}, {}, {}", mouse_button, x, y);
			}
		}
	}
}

struct MainState {
	backend: GgezBackend,
}

impl MainState {
	pub fn new() -> Self {
		let mut backend = GgezBackend::new();
		backend.gui.add_control(ControlSettings {
			rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
			height: 0,
			behaviors: vec![Box::new(TestBehavior {})],
		});
		backend.gui.add_control(ControlSettings {
			rectangle: Rectangle::new(100.0, 100.0, 100.0, 100.0),
			height: 1,
			behaviors: vec![Box::new(TestBehavior {})],
		});
		Self { backend }
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
		self.backend.draw_debug(ctx)?;
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> ggez::GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("audio-gui", "tesselode").build()?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut MainState::new())?;
	Ok(())
}
