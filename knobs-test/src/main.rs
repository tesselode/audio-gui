use ggez::{graphics, Context, GameResult};
use knobs::{
	behavior::rectangle::Rectangle,
	canvas::{Color, DrawOperation, ShapeStyle},
	geometry::rect::Rect,
	gui::{ElementSettings, Gui},
};

fn to_ggez_color(color: Color) -> graphics::Color {
	graphics::Color::new(color.red, color.green, color.blue, color.alpha)
}

fn to_ggez_rect(rect: Rect) -> graphics::Rect {
	graphics::Rect::new(rect.position.x, rect.position.y, rect.size.x, rect.size.y)
}

struct MainState {
	gui: Gui,
}

impl MainState {
	pub fn new() -> Self {
		let mut gui = Gui::new();
		gui.add(ElementSettings {
			rect: Rect::new(50.0, 50.0, 100.0, 200.0),
			behavior: Some(Box::new(
				Rectangle::new()
					.fill(Color::new(0.25, 0.25, 0.25, 0.25))
					.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
			)),
			children: vec![ElementSettings {
				rect: Rect::new(50.0, 50.0, 25.0, 25.0),
				behavior: Some(Box::new(
					Rectangle::new()
						.fill(Color::new(0.25, 0.25, 0.25, 0.25))
						.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
				)),
				..Default::default()
			}],
			..Default::default()
		});
		gui.add(ElementSettings {
			rect: Rect::new(400.0, 50.0, 100.0, 200.0),
			behavior: Some(Box::new(
				Rectangle::new()
					.fill(Color::new(0.25, 0.25, 0.25, 0.25))
					.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
			)),
			children: vec![ElementSettings {
				rect: Rect::new(50.0, 50.0, 25.0, 25.0),
				behavior: Some(Box::new(
					Rectangle::new()
						.fill(Color::new(0.25, 0.25, 0.25, 0.25))
						.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
				)),
				..Default::default()
			}],
			..Default::default()
		});
		Self { gui }
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		let canvas = self.gui.draw();
		for operation in &canvas.operations {
			match operation {
				DrawOperation::DrawRectangle(rect, style) => match style {
					ShapeStyle::Fill(color) => {
						let mesh = graphics::Mesh::new_rectangle(
							ctx,
							graphics::DrawMode::fill(),
							to_ggez_rect(*rect),
							to_ggez_color(*color),
						)?;
						graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
					}
					ShapeStyle::Stroke(width, color) => {
						let mesh = graphics::Mesh::new_rectangle(
							ctx,
							graphics::DrawMode::stroke(*width),
							to_ggez_rect(*rect),
							to_ggez_color(*color),
						)?;
						graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
					}
				},
			}
		}
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("knobs-test", "tesselode").build()?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut MainState::new())?;
	Ok(())
}
