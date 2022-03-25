use crate::{create_arr_with_length, vec_arr_func};
use dryoc::classic::crypto_secretbox;
use sodiumoxide::crypto::secretbox;
use dryoc::constants::*;
use napi::bindgen_prelude::*;

vec_arr_func!(to_nonce, u8, 24);
vec_arr_func!(to_key, u8, 32);

#[napi(object, js_name = "Secret_Box")]
#[allow(non_camel_case_types)]
pub struct Secret_Box {
    pub ciphertext: Uint8Array,
    pub mac: Uint8Array,
}

#[napi]
pub struct SecretBox {}

#[napi]
impl SecretBox {
    #[napi(constructor)]
    pub fn new() -> Self {
        SecretBox {}
    }

    #[napi(js_name = "crypto_secretbox_detached")]
    pub fn crypto_secretbox_detached(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        key: Uint8Array,
    ) -> Secret_Box {
        let mut cipher: Vec<u8> = vec![];
        let mut mac = create_arr_with_length!(16);

        crypto_secretbox::crypto_secretbox_detached(
            &mut cipher,
            &mut mac,
            &message,
            &to_nonce(&nonce),
            &to_key(&key),
        );

        Secret_Box {
            ciphertext: Uint8Array::new(cipher),
            mac: Uint8Array::new(mac.to_vec()),
        }
    }

    #[napi(js_name = "crypto_secretbox_easy")]
    pub fn crypto_secretbox_easy(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        key: Uint8Array,
    ) -> Uint8Array {
        let mut c: Vec<u8> = vec![];

        crypto_secretbox::crypto_secretbox_easy(&mut c, &message, &to_nonce(&nonce), &to_key(&key))
            .unwrap();

        Uint8Array::new(c.to_vec())
    }
}
