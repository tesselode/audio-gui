use crate::geometry::vector::Vector;
use rusttype::{Font, Scale};

pub fn size_of_text(font: &Font, text: &str, scale: Vector) -> Vector {
	let mut max_x: f32 = 0.0;
	let mut max_y: f32 = 0.0;
	for glyph in font.layout(
		text,
		Scale {
			x: scale.x,
			y: scale.y,
		},
		rusttype::Point { x: 0.0, y: 0.0 },
	) {
		let v_metrics = font.v_metrics(glyph.scale());
		if let Some(bounds) = glyph.pixel_bounding_box() {
			max_x = max_x.max(bounds.max.x as f32);
			max_y = max_y.max(bounds.max.y as f32 + v_metrics.ascent);
		}
	}
	Vector::new(max_x, max_y)
}
