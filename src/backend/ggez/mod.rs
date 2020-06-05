use crate::gui::{
	display::{Color, Display, DrawMode, Style},
	mouse_button::MouseButton,
	point::Point,
	rectangle::Rectangle,
	Gui,
};
use ggez::{graphics::MeshBuilder, Context, GameResult};

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

pub struct GgezBackendDisplay {
	mesh_builder: MeshBuilder,
	created_meshes: bool,
}

impl GgezBackendDisplay {
	pub fn new() -> Self {
		Self {
			mesh_builder: MeshBuilder::new(),
			created_meshes: false,
		}
	}
}

impl Display for GgezBackendDisplay {
	fn draw_rectangle(&mut self, rectangle: Rectangle, style: Style) {
		self.mesh_builder
			.rectangle(style.mode.into(), rectangle.into(), style.color.into());
		self.created_meshes = true;
	}

	fn draw_circle(&mut self, position: Point, radius: f32, style: Style) {
		self.mesh_builder
			.circle(style.mode.into(), position, radius, 0.1, style.color.into());
		self.created_meshes = true;
	}

	fn draw_polyline(&mut self, points: Vec<Point>, style: Style) {
		todo!()
	}

	fn draw_polygon(&mut self, points: Vec<Point>, style: Style) {
		todo!()
	}
}

pub struct GgezBackend {
	pub gui: Gui,
}

impl GgezBackend {
	pub fn new() -> Self {
		Self { gui: Gui::new() }
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

	pub fn draw_debug(&mut self, ctx: &mut Context) -> GameResult {
		let mut display = GgezBackendDisplay::new();
		self.gui.draw_debug(&mut display);
		if !display.created_meshes {
			return Ok(());
		}
		let mesh = display.mesh_builder.build(ctx)?;
		ggez::graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::new())?;
		Ok(())
	}
}
