use skia_safe::{FontMgr, Typeface};
use std::collections::HashMap;

const DOUYIN_SANS_BOLD: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/DouyinSansBold.ttf"));

// const HARMONY_OS_SANS: &[u8] =
// 	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/HarmonyOS_Sans.ttf"));

#[derive(Debug, Default, Clone)]
pub struct FontStore(HashMap<String, Typeface>);

impl FontStore {
	pub fn new() -> Self {
		let mut store = Self::default();
		store.load_fonts();
		store
	}

	fn load_fonts(&mut self) {
		let font_mgr = FontMgr::default();

		if let Some(typeface) = font_mgr.new_from_data(DOUYIN_SANS_BOLD, None) {
			self.0.insert("DouyinSansBold".to_string(), typeface);
		}

		// if let Some(typeface) = font_mgr.new_from_data(HARMONY_OS_SANS, None) {
		// 	self.0.insert("HarmonyOSSans".to_string(), typeface);
		// }
	}

	pub fn get(&self, font_family: &str) -> Option<&Typeface> {
		self.0.get(font_family)
	}

	pub fn names(&self) -> Vec<String> {
		self.0.keys().cloned().collect()
	}
}

impl<'a> IntoIterator for &'a FontStore {
	type Item = (&'a String, &'a Typeface);
	type IntoIter = std::collections::hash_map::Iter<'a, String, Typeface>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}
