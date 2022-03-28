use napi::bindgen_prelude::*;

#[napi(object)]
pub struct KeyPair {
    #[napi(js_name = "public_key")]
    pub public_key: Uint8Array,
    #[napi(js_name = "secret_key")]
    pub secret_key: Uint8Array,
}
