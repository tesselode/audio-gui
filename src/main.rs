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
	fn on_hover(&mut self, controls: &mut Controls, id: &ControlId, x: f32, y: f32) {
		println!("hover: {}, {}", x, y);
	}

	fn on_unhover(&mut self, _controls: &mut Controls, _id: &ControlId) {
		println!("unhover");
	}
}

struct MainState {
	backend: GgezBackend,
}

impl MainState {
	pub fn new() -> Self {
		let mut backend = GgezBackend::new();
		let id = backend.gui.add_control(ControlSettings {
			rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
			height: 0,
		});
		backend.gui.attach_behavior(id, Box::new(TestBehavior {}));
		let id = backend.gui.add_control(ControlSettings {
			rectangle: Rectangle::new(100.0, 100.0, 100.0, 100.0),
			height: 1,
		});
		backend.gui.attach_behavior(id, Box::new(TestBehavior {}));
		Self { backend }
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		Ok(())
	}

	fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, dx: f32, dy: f32) {
		self.backend.gui.on_mouse_move(x, y, dx, dy);
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
