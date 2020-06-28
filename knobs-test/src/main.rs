use ggez::{graphics, Context, GameResult};
use knobs::{
	behavior,
	canvas::{Color, DrawOperation, ShapeStyle},
	geometry::{rect::Rect, vector::Vector},
	gui::{ElementSettings, Gui},
	input::MouseButton,
	resources::ImageId,
};
use std::collections::HashMap;

fn to_ggez_color(color: Color) -> graphics::Color {
	graphics::Color::new(color.red, color.green, color.blue, color.alpha)
}

fn to_ggez_rect(rect: Rect) -> graphics::Rect {
	graphics::Rect::new(rect.position.x, rect.position.y, rect.size.x, rect.size.y)
}

fn to_ggez_vector(vector: Vector) -> ggez::mint::Point2<f32> {
	ggez::mint::Point2 {
		x: vector.x,
		y: vector.y,
	}
}

struct MainState {
	gui: Gui,
	images: HashMap<ImageId, graphics::Image>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let mut gui = Gui::new();
		gui.add(ElementSettings {
			rect: Rect::from_xywh(50.0, 50.0, 0.0, 500.0),
			behaviors: vec![
				Box::new(behavior::Rectangle::new().fill(Color::new(0.25, 0.25, 0.25, 1.0))),
				Box::new(behavior::Flex::new(
					behavior::flex::Axis::Vertical,
					behavior::flex::Distribution::SpaceEvenly,
					0.5,
				)),
			],
			children: vec![
				ElementSettings {
					rect: Rect::from_xywh(50.0, 50.0, 50.0, 50.0),
					behaviors: vec![Box::new(
						behavior::Rectangle::new().stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)],
					..Default::default()
				},
				ElementSettings {
					rect: Rect::from_xywh(90.0, 150.0, 100.0, 75.0),
					behaviors: vec![Box::new(
						behavior::Rectangle::new().stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)],
					..Default::default()
				},
				ElementSettings {
					rect: Rect::from_xywh(150.0, 40.0, 25.0, 125.0),
					behaviors: vec![Box::new(
						behavior::Rectangle::new().stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0)),
					)],
					..Default::default()
				},
			],
			..Default::default()
		});
		Ok(Self {
			gui,
			images: HashMap::new(),
		})
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
				DrawOperation::DrawImage(image_id, position, scale, color) => {
					let image = self.images.get(&image_id).unwrap();
					graphics::draw(
						ctx,
						image,
						graphics::DrawParam::new()
							.dest(to_ggez_vector(*position))
							.scale(to_ggez_vector(*scale))
							.color(to_ggez_color(*color)),
					)?;
				}
			}
		}
		graphics::present(ctx)?;
		Ok(())
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("knobs-test", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx)?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)?;
	Ok(())
}
