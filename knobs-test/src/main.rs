use ggez::{graphics, Context, GameResult};
use knobs::{
	behavior::{
		flex::{Axis, Flex},
		rectangle::Rectangle,
	},
	canvas::{Color, DrawOperation, ShapeStyle},
	geometry::rect::Rect,
	gui::{ElementSettings, Gui},
	input::MouseButton,
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
			behavior: Some(Box::new(Flex::new(Axis::Horizontal))),
			children: vec![
				ElementSettings {
					rect: Rect::new(50.0, 50.0, 25.0, 25.0),
					behavior: Some(Box::new(
						Rectangle::new()
							.fill(Color::new(0.25, 0.25, 0.25, 0.25))
							.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)),
					..Default::default()
				},
				ElementSettings {
					rect: Rect::new(200.0, 300.0, 100.0, 150.0),
					behavior: Some(Box::new(
						Rectangle::new()
							.fill(Color::new(0.25, 0.25, 0.25, 0.25))
							.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)),
					..Default::default()
				},
				ElementSettings {
					rect: Rect::new(400.0, 0.0, 50.0, 25.0),
					behavior: Some(Box::new(
						Rectangle::new()
							.fill(Color::new(0.25, 0.25, 0.25, 0.25))
							.stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)),
					..Default::default()
				},
			],
			..Default::default()
		});
		Self { gui }
	}
}

impl ggez::event::EventHandler for MainState {
	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
		self.gui.on_move_mouse(x, y, dx, dy);
	}

	fn mouse_button_down_event(
		&mut self,
		_ctx: &mut Context,
		button: ggez::event::MouseButton,
		_x: f32,
		_y: f32,
	) {
		self.gui.on_press_mouse_button(match button {
			ggez::event::MouseButton::Left => MouseButton::Left,
			ggez::event::MouseButton::Right => MouseButton::Right,
			ggez::event::MouseButton::Middle => MouseButton::Middle,
			ggez::event::MouseButton::Other(_) => {
				return;
			}
		})
	}

	fn mouse_button_up_event(
		&mut self,
		_ctx: &mut Context,
		button: ggez::event::MouseButton,
		_x: f32,
		_y: f32,
	) {
		self.gui.on_release_mouse_button(match button {
			ggez::event::MouseButton::Left => MouseButton::Left,
			ggez::event::MouseButton::Right => MouseButton::Right,
			ggez::event::MouseButton::Middle => MouseButton::Middle,
			ggez::event::MouseButton::Other(_) => {
				return;
			}
		})
	}

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
