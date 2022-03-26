use napi::bindgen_prelude::*;
use sodiumoxide::{init, crypto::sign};

#[napi]
pub struct Sign {}

#[napi]
impl Sign {
	#[napi(constructor)]
	pub fn new() -> Self {
		init().unwrap();
		Sign {}
	}

	#[napi(js_name = "crypto_sign")]
	pub fn crypto_sign(&self, m: Uint8Array, sk: Uint8Array) -> Uint8Array {
		let sm = sign::sign(&m, &sign::SecretKey::from_slice(&sk).unwrap());

		Uint8Array::new(sm)
	}

	#[napi(js_name = "crypto_sign_detached")]
	pub fn crypto_sign_detached(&self, m: Uint8Array, sk: Uint8Array) -> Uint8Array {
		let sig = sign::sign_detached(&m, &sign::SecretKey::from_slice(&sk).unwrap());

		Uint8Array::new(sig.as_ref().to_vec())
	}
}
