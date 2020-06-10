use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidFontError;

impl Error for InvalidFontError {}

impl Display for InvalidFontError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Invalid font data")?;
		Ok(())
	}
}
