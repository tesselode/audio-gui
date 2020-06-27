use image::{ImageResult, RgbaImage};

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct ImageId {
	pub index: usize,
}

impl PartialEq for ImageId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

pub struct Resources {
	images: Vec<RgbaImage>,
}

impl Resources {
	pub fn new() -> Self {
		Self { images: vec![] }
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
}
