use ggez::{
	graphics::{Font, MeshBuilder, Scale, Text},
	Context, GameResult,
};
use knobby::{
	canvas::{ArcKind, Canvas, Color, DrawMode, DrawOperation, Style, TextStyle},
	geometry::{Point, Rectangle},
	gui::Gui,
	input::MouseButton,
};

fn convert_draw_mode(mode: DrawMode) -> ggez::graphics::DrawMode {
	match mode {
		DrawMode::Fill => ggez::graphics::DrawMode::fill(),
		DrawMode::Stroke(width) => ggez::graphics::DrawMode::stroke(width),
	}
}

fn convert_color(color: Color) -> ggez::graphics::Color {
	ggez::graphics::Color::new(color.red, color.green, color.blue, color.alpha)
}

fn convert_rectangle(rectangle: Rectangle) -> ggez::graphics::Rect {
	ggez::graphics::Rect::new(rectangle.x, rectangle.y, rectangle.width, rectangle.height)
}

fn convert_point(point: Point) -> ggez::mint::Point2<f32> {
	ggez::mint::Point2 {
		x: point.x,
		y: point.y,
	}
}

pub struct GgezBackend<CustomEvent> {
	pub gui: Gui<CustomEvent>,
	fonts: Vec<Font>,
}

impl<CustomEvent> GgezBackend<CustomEvent>
where
	CustomEvent: Copy + Clone,
{
	pub fn new() -> Self {
		Self {
			gui: Gui::new(),
			fonts: vec![],
		}
	}

	pub fn load_font(&mut self, ctx: &mut Context, font_data: &'static [u8]) -> GameResult {
		self.fonts.push(Font::new_glyph_font_bytes(ctx, font_data)?);
		self.gui.resources.load_font(font_data).unwrap();
		Ok(())
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

	fn draw_rectangle(
		&mut self,
		mesh_builder: &mut MeshBuilder,
		rectangle: Rectangle,
		style: Style,
	) {
		mesh_builder.rectangle(
			convert_draw_mode(style.mode),
			convert_rectangle(rectangle),
			convert_color(style.color),
		);
	}

	fn draw_circle(
		&mut self,
		mesh_builder: &mut MeshBuilder,
		position: Point,
		radius: f32,
		style: Style,
	) {
		mesh_builder.circle(
			convert_draw_mode(style.mode),
			convert_point(position),
			radius,
			0.1,
			convert_color(style.color),
		);
	}

	fn draw_arc(
		&mut self,
		mesh_builder: &mut MeshBuilder,
		kind: ArcKind,
		position: Point,
		radius: f32,
		angle1: f32,
		angle2: f32,
		style: Style,
	) -> GameResult {
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
		let mint_points: Vec<ggez::mint::Point2<f32>> =
			points.iter().map(|point| convert_point(*point)).collect();
		match style.mode {
			DrawMode::Fill => {
				mesh_builder.polygon(
					convert_draw_mode(style.mode),
					&mint_points,
					convert_color(style.color),
				)?;
			}
			DrawMode::Stroke(width) => {
				mesh_builder.line(&mint_points, width, convert_color(style.color))?;
			}
		}
		Ok(())
	}

	pub fn draw_text(
		&mut self,
		ctx: &mut Context,
		text: String,
		mut position: Point,
		style: TextStyle,
	) -> GameResult {
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
					.dest(convert_point(position))
					.color(convert_color(style.color)),
			)?;
		}
		Ok(())
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
					self.draw_rectangle(&mut mesh_builder, rectangle, style);
					created_meshes = true;
				}
				DrawOperation::Circle(position, radius, style) => {
					self.draw_circle(&mut mesh_builder, position, radius, style);
					created_meshes = true;
				}
				DrawOperation::Arc(kind, position, radius, angle1, angle2, style) => {
					self.draw_arc(
						&mut mesh_builder,
						kind,
						position,
						radius,
						angle1,
						angle2,
						style,
					)?;
					created_meshes = true;
				}
				DrawOperation::Polyline(_, _) => todo!(),
				DrawOperation::Polygon(_, _) => todo!(),
				DrawOperation::Text(text, position, style) => {
					self.draw_text(ctx, text, position, style)?;
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
