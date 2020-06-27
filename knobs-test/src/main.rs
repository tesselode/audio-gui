use ggez::{graphics, Context, GameResult};
use knobs::{
	behavior::rectangle::Rectangle,
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
	images: HashMap<ImageId, graphics::Image>,
	gui: Gui,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let bean_man = include_bytes!("resources/bean man.jpg");
		let mut gui = Gui::new();
		let id = gui.resources.load_image(bean_man).unwrap();
		let image = gui.resources.get_image(id);
		let mut images = HashMap::new();
		images.insert(
			id,
			graphics::Image::from_rgba8(
				ctx,
				image.width() as u16,
				image.height() as u16,
				image.as_flat_samples().samples,
			)?,
		);
		gui.add(ElementSettings {
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
		Ok(Self { gui, images })
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
		let mut canvas = self.gui.draw();
		canvas.draw_image(ImageId { index: 0 }, Vector::new(50.0, 50.0));
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
				DrawOperation::DrawImage(image_id, position) => {
					let image = self.images.get(&image_id).unwrap();
					graphics::draw(
						ctx,
						image,
						graphics::DrawParam::new().dest(to_ggez_vector(*position)),
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
