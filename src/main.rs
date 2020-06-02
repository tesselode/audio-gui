mod gui;

use ggez::graphics;
use gui::{
	control::{behavior::ControlBehavior, ControlSettings},
	rectangle::Rectangle,
	Gui,
};

struct TestBehavior;

impl ControlBehavior for TestBehavior {
	fn on_hover(&mut self, x: f32, y: f32) {
		println!("hover: {}, {}", x, y);
	}

	fn on_unhover(&mut self) {
		println!("unhover");
	}
}

struct MainState {
	gui: Gui,
}

impl MainState {
	pub fn new() -> Self {
		let mut gui = Gui::new();
		gui.add_control(ControlSettings {
			rectangle: Rectangle::new(50.0, 50.0, 100.0, 100.0),
			height: 0,
			behaviors: vec![Box::new(TestBehavior {})],
		});
		gui.add_control(ControlSettings {
			rectangle: Rectangle::new(100.0, 100.0, 100.0, 100.0),
			height: 1,
			behaviors: vec![Box::new(TestBehavior {})],
		});
		Self { gui }
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		Ok(())
	}

	fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, dx: f32, dy: f32) {
		self.gui.on_mouse_move(x, y, dx, dy);
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, graphics::BLACK);
		let mut mesh_builder = graphics::MeshBuilder::new();
		self.gui.draw_debug(&mut mesh_builder);
		let mesh = mesh_builder.build(ctx)?;
		graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> ggez::GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("audio-gui", "tesselode").build()?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut MainState::new())?;
	Ok(())
}
