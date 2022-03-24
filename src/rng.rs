use dryoc::rng;
use napi::bindgen_prelude::*;

#[napi(js_name = "randombytes_buf")]
pub fn randombytes_buf(len: u32) -> Uint8Array {
    let r = rng::randombytes_buf(len.try_into().unwrap());

    Uint8Array::new(r)
}
