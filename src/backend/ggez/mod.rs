use crate::gui::{
	canvas::{ArcKind, Canvas, Color, DrawMode, DrawOperation, Style},
	mouse_button::MouseButton,
	point::Point,
	rectangle::Rectangle,
	Gui,
};
use ggez::{
	graphics::{Font, MeshBuilder, Scale, Text},
	Context, GameResult,
};

impl From<DrawMode> for ggez::graphics::DrawMode {
	fn from(mode: DrawMode) -> Self {
		match mode {
			DrawMode::Fill => ggez::graphics::DrawMode::fill(),
			DrawMode::Stroke(width) => ggez::graphics::DrawMode::stroke(width),
		}
	}
}

impl From<Color> for ggez::graphics::Color {
	fn from(color: Color) -> Self {
		ggez::graphics::Color::new(color.red, color.green, color.blue, color.alpha)
	}
}

impl From<Rectangle> for ggez::graphics::Rect {
	fn from(rectangle: Rectangle) -> Self {
		ggez::graphics::Rect::new(rectangle.x, rectangle.y, rectangle.width, rectangle.height)
	}
}

impl From<Point> for ggez::mint::Point2<f32> {
	fn from(point: Point) -> Self {
		Self {
			x: point.x,
			y: point.y,
		}
	}
}

pub struct GgezBackendOptions {
	pub fonts: Vec<Vec<u8>>,
}

pub struct GgezBackend {
	pub gui: Gui,
	fonts: Vec<Font>,
}

impl GgezBackend {
	pub fn new(ctx: &mut Context, mut options: GgezBackendOptions) -> GameResult<Self> {
		let mut fonts = vec![];
		for bytes in options.fonts.drain(..) {
			fonts.push(Font::new_glyph_font_bytes(ctx, &bytes)?);
		}
		Ok(Self {
			gui: Gui::new(),
			fonts,
		})
	}

	pub fn mouse_motion_event(
		&mut self,
		_ctx: &mut ggez::Context,
		x: f32,
		y: f32,
		dx: f32,
		dy: f32,
	) {
		self.gui.on_mouse_move(x, y, dx, dy);
	}

	pub fn mouse_button_down_event(
		&mut self,
		_ctx: &mut ggez::Context,
		button: ggez::event::MouseButton,
		x: f32,
		y: f32,
	) {
		let button = match button {
			ggez::event::MouseButton::Left => MouseButton::Left,
			ggez::event::MouseButton::Right => MouseButton::Right,
			ggez::event::MouseButton::Middle => MouseButton::Middle,
			ggez::event::MouseButton::Other(_) => {
				return;
			}
		};
		self.gui.on_mouse_down(button, x, y);
	}

	pub fn mouse_button_up_event(
		&mut self,
		_ctx: &mut ggez::Context,
		button: ggez::event::MouseButton,
		x: f32,
		y: f32,
	) {
		let button = match button {
			ggez::event::MouseButton::Left => MouseButton::Left,
			ggez::event::MouseButton::Right => MouseButton::Right,
			ggez::event::MouseButton::Middle => MouseButton::Middle,
			ggez::event::MouseButton::Other(_) => {
				return;
			}
		};
		self.gui.on_mouse_up(button, x, y);
	}

	pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::new();
		self.gui.draw(&mut canvas);
		if !canvas.operations.len() == 0 {
			return Ok(());
		}
		let mut mesh_builder = MeshBuilder::new();
		let mut created_meshes = false;
		for operation in canvas.operations {
			match operation {
				DrawOperation::Rectangle(rectangle, style) => {
					mesh_builder.rectangle(style.mode.into(), rectangle.into(), style.color.into());
					created_meshes = true;
				}
				DrawOperation::Circle(position, radius, style) => {
					mesh_builder.circle(
						style.mode.into(),
						position,
						radius,
						0.1,
						style.color.into(),
					);
					created_meshes = true;
				}
				DrawOperation::Arc(kind, position, radius, angle1, angle2, style) => {
					let mut points = vec![];
					let segments = radius.ceil() as usize;
					for i in 0..=segments {
						let angle = angle1 + (angle2 - angle1) * (i as f32 / segments as f32);
						points.push(Point::new(
							position.x + radius * angle.cos(),
							position.y + radius * angle.sin(),
						));
					}
					match kind {
						ArcKind::Pie => {
							points.push(position);
							points.push(Point::new(
								position.x + radius * angle1.cos(),
								position.y + radius * angle1.sin(),
							));
						}
						ArcKind::Open => {}
						ArcKind::Closed => {
							points.push(Point::new(
								position.x + radius * angle1.cos(),
								position.y + radius * angle1.sin(),
							));
						}
					}
					match style.mode {
						DrawMode::Fill => {
							mesh_builder.polygon(style.mode.into(), &points, style.color.into())?;
						}
						DrawMode::Stroke(width) => {
							mesh_builder.line(&points, width, style.color.into())?;
						}
					}
					created_meshes = true;
				}
				DrawOperation::Polyline(_, _) => todo!(),
				DrawOperation::Polygon(_, _) => todo!(),
				DrawOperation::Text(text, mut position, style) => {
					if let Some(font) = self.fonts.get(style.font_id) {
						let mut t = Text::new(text);
						t.set_font(
							*font,
							Scale {
								x: style.size,
								y: style.size,
							},
						);
						position.x -= t.width(ctx) as f32 * style.horizontal_alignment.as_f32();
						position.y -= t.height(ctx) as f32 * style.vertical_alignment.as_f32();
						ggez::graphics::draw(
							ctx,
							&t,
							ggez::graphics::DrawParam::new()
								.dest(position)
								.color(style.color.into()),
						)?;
					}
				}
			}
		}
		if !created_meshes {
			return Ok(());
		}
		let mesh = mesh_builder.build(ctx)?;
		ggez::graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::new())?;
		Ok(())
	}
}
