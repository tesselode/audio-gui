use ggez::graphics;

struct MainState;

impl MainState {
	pub fn new() -> Self {
		Self {}
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, graphics::BLACK);
		let text = graphics::Text::new("hi!");
		graphics::draw(ctx, &text, graphics::DrawParam::new())?;
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> ggez::GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("audio-gui", "tesselode").build()?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut MainState::new())?;
	Ok(())
}
