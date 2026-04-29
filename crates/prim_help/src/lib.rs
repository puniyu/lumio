mod options;
pub use options::*;

use prim_common::Error;
use skia_safe::{ISize, surfaces};

pub fn help(options: Options) -> Result<Vec<u8>, Error> {
	let mut surface = surfaces::raster_n32_premul(ISize::new(800, 600)).ok_or(Error::Encode)?;
	let canvas = surface.canvas();

	Ok(vec![])
}
