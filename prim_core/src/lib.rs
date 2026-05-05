mod font;
pub use font::FontManger;
#[doc(inline)]
pub use prim_common::Error;

use bytes::Bytes;

pub trait Render {
	fn render(&self) -> Result<Bytes, Error>;
}
