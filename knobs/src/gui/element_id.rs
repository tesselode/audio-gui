#[derive(Debug, Eq, Hash, Copy, Clone)]
pub struct ElementId {
	pub(super) index: usize,
}

impl PartialEq for ElementId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

pub struct ElementIdIter {
	index: usize,
	len: usize,
}

impl ElementIdIter {
	pub(super) fn new(len: usize) -> Self {
		Self { index: 0, len }
	}
}

impl Iterator for ElementIdIter {
	type Item = ElementId;

	fn next(&mut self) -> Option<Self::Item> {
		let item = if self.index < self.len {
			Some(ElementId { index: self.index })
		} else {
			None
		};
		self.index += 1;
		item
	}
}
