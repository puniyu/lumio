#![allow(dead_code)]

mod options;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use options::Options;
use prim::Render as _;

#[napi]
pub fn help(options: Options) -> napi::Result<Buffer> {
    match prim::help::Help::new(options.into()).render() {
        Ok(bytes) => Ok(Buffer::from(bytes.to_vec())),
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
    }
}