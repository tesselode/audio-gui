#[derive(Debug)]
pub struct LoadFontError;

impl std::fmt::Display for LoadFontError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Error loading font")?;
		Ok(())
	}
}

impl std::error::Error for LoadFontError {}
