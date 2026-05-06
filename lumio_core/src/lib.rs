mod font;
pub use font::FontManger;
mod error;
pub use error::Error;

use bytes::Bytes;

pub trait Render {
	fn render(&self) -> Result<Bytes, Error>;
}
