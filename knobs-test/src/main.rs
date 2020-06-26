use ggez::{graphics, Context, GameResult};
use knobs::gui::{ElementSettings, Gui};

struct MainState {
	gui: Gui,
}

impl MainState {
	pub fn new() -> Self {
		let mut gui = Gui::new();
		gui.add(ElementSettings {
			..Default::default()
		});
		println!("{:#?}", gui.elements);
		Self { gui }
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("knobs-test", "tesselode").build()?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut MainState::new())?;
	Ok(())
}
