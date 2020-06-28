use ggez::{graphics, Context, GameResult};
use knobs::{
	canvas::{Color, DrawOperation, ShapeStyle},
	component,
	geometry::{rect::Rect, vector::Vector},
	gui::{ElementSettings, Gui},
	input::MouseButton,
	resources::{FontId, ImageId},
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
	fonts: HashMap<FontId, graphics::Font>,
	images: HashMap<ImageId, graphics::Image>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let mut gui = Gui::new();
		let font_data = include_bytes!("resources/Montserrat-Regular.ttf");
		let id = gui.resources.load_font(font_data).unwrap();
		let mut fonts = HashMap::new();
		fonts.insert(id, graphics::Font::new_glyph_font_bytes(ctx, font_data)?);
		gui.add(ElementSettings {
			rect: Rect::from_xywh(50.0, 50.0, 0.0, 0.0),
			components: vec![
				Box::new(component::Text::new(
					id,
					"Hello world!",
					Vector::new(50.0, 50.0),
				)),
				Box::new(component::Rectangle::new().stroke(2.0, Color::new(1.0, 1.0, 1.0, 1.0))),
			],
			..Default::default()
		});
		Ok(Self {
			gui,
			images: HashMap::new(),
			fonts,
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
		for operation in canvas.operations {
			match operation {
				DrawOperation::DrawRectangle(rect, style) => match style {
					ShapeStyle::Fill(color) => {
						let mesh = graphics::Mesh::new_rectangle(
							ctx,
							graphics::DrawMode::fill(),
							to_ggez_rect(rect),
							to_ggez_color(color),
						)?;
						graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
					}
					ShapeStyle::Stroke(width, color) => {
						let mesh = graphics::Mesh::new_rectangle(
							ctx,
							graphics::DrawMode::stroke(width),
							to_ggez_rect(rect),
							to_ggez_color(color),
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
							.dest(to_ggez_vector(position))
							.scale(to_ggez_vector(scale))
							.color(to_ggez_color(color)),
					)?;
				}
				DrawOperation::DrawText(font_id, text, position, scale, color) => {
					let mut text = graphics::Text::new(text);
					text.set_font(
						*self.fonts.get(&font_id).unwrap(),
						graphics::Scale {
							x: scale.x,
							y: scale.y,
						},
					);
					graphics::draw(
						ctx,
						&text,
						graphics::DrawParam::new()
							.dest(to_ggez_vector(position))
							.color(to_ggez_color(color)),
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
