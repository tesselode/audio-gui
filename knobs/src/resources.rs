use crate::error::LoadFontError;
use image::{ImageResult, RgbaImage};
use rusttype::Font;

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct ImageId {
	index: usize,
}

impl PartialEq for ImageId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct FontId {
	index: usize,
}

impl PartialEq for FontId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

pub struct Resources {
	images: Vec<RgbaImage>,
	fonts: Vec<Font<'static>>,
}

impl Resources {
	pub fn new() -> Self {
		Self {
			images: vec![],
			fonts: vec![],
		}
	}

	pub fn load_image(&mut self, data: &[u8]) -> ImageResult<ImageId> {
		let id = ImageId {
			index: self.images.len(),
		};
		self.images.push(match image::load_from_memory(data) {
			Ok(image) => image.to_rgba(),
			Err(error) => {
				return Err(error);
			}
		});
		Ok(id)
	}

	pub fn get_image(&self, id: ImageId) -> &RgbaImage {
		self.images.get(id.index).unwrap()
	}

	pub fn load_font(&mut self, data: &'static [u8]) -> Result<FontId, LoadFontError> {
		let id = FontId {
			index: self.images.len(),
		};
		self.fonts.push(match Font::try_from_bytes(data) {
			Some(font) => font,
			None => {
				return Err(LoadFontError {});
			}
		});
		Ok(id)
	}

	pub fn get_font(&self, id: FontId) -> &Font {
		self.fonts.get(id.index).unwrap()
	}
}
