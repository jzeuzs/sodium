use crate::{create_arr_with_length, vec_arr_func};
use dryoc::classic::crypto_box;
use dryoc::constants::*;
use dryoc::types::*;
use napi::bindgen_prelude::*;
use std::ops::Deref;

vec_arr_func!(to_public_key, u8, 32);
vec_arr_func!(to_secret_key, u8, 32);
vec_arr_func!(to_mac, u8, 16);
vec_arr_func!(to_nonce, u8, 24);
vec_arr_func!(to_key, u8, 32);

#[napi(object)]
pub struct KeyPair {
    #[napi(js_name = "public_key")]
    pub public_key: Uint8Array,
    #[napi(js_name = "secret_key")]
    pub secret_key: Uint8Array,
}

#[napi(object)]
pub struct CryptoBox {
    pub ciphertext: Uint8Array,
    pub mac: Uint8Array,
}

#[napi(js_name = "Box")]
pub struct SodiumBox {}

#[napi]
impl SodiumBox {
    #[napi(constructor)]
    pub fn new() -> Self {
        SodiumBox {}
    }

    #[napi(js_name = "crypto_box_beforenm")]
    pub fn crypto_box_beforenm(
        &self,
        public_key: Uint8Array,
        secret_key: Uint8Array,
    ) -> Uint8Array {
        let key = crypto_box::crypto_box_beforenm(
            &to_public_key(&public_key),
            &to_secret_key(&secret_key),
        );

        Uint8Array::new(key.to_vec())
    }

    #[napi(js_name = "crypto_box_detached")]
    pub fn crypto_box_detached(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        recipient_public_key: Uint8Array,
        sender_secret_key: Uint8Array,
    ) -> CryptoBox {
        let mut cipher: Vec<u8> = vec![];
        let mut mac = create_arr_with_length!(16);

        crypto_box::crypto_box_detached(
            &mut cipher,
            &mut mac,
            &message,
            &to_nonce(&nonce),
            &to_public_key(&recipient_public_key),
            &to_secret_key(&sender_secret_key),
        );

        CryptoBox {
            ciphertext: Uint8Array::new(cipher),
            mac: Uint8Array::new(mac.to_vec()),
        }
    }

    #[napi(js_name = "crypto_box_easy")]
    pub fn crypto_box_easy(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        recipient_public_key: Uint8Array,
        sender_secret_key: Uint8Array,
    ) -> Uint8Array {
        let mut cipher: Vec<u8> = vec![];

        crypto_box::crypto_box_easy(
            &mut cipher,
            &message,
            &to_nonce(&nonce),
            &to_public_key(&recipient_public_key),
            &to_secret_key(&sender_secret_key),
        )
        .unwrap();

        Uint8Array::new(cipher)
    }

    #[napi(js_name = "crypto_box_keypair")]
    pub fn crypto_box_keypair(&self) -> KeyPair {
        let (publickey, secretkey) = crypto_box::crypto_box_keypair();

        KeyPair {
            public_key: Uint8Array::new(publickey.to_vec()),
            secret_key: Uint8Array::new(secretkey.to_vec()),
        }
    }

    #[napi(js_name = "crypto_box_nonce")]
    pub fn crypto_box_nonce(&self) -> Uint8Array {
        let nonce = crypto_box::Nonce::gen();

        Uint8Array::new(nonce.to_vec())
    }

    #[napi(js_name = "crypto_box_open_detached")]
    pub fn crypto_box_open_detached(
        &self,
        mac: Uint8Array,
        ciphertext: Uint8Array,
        nonce: Uint8Array,
        recipient_public_key: Uint8Array,
        sender_secret_key: Uint8Array,
    ) -> Uint8Array {
        let mut m: Vec<u8> = vec![];

        crypto_box::crypto_box_open_detached(
            &mut m,
            &to_mac(&mac),
            ciphertext.deref(),
            &to_nonce(&nonce),
            &to_public_key(&recipient_public_key),
            &to_secret_key(&sender_secret_key),
        )
        .unwrap();

        Uint8Array::new(m)
    }

    #[napi(js_name = "crypto_box_open_detached_afternm")]
    pub fn crypto_box_open_detached_afternm(
        &self,
        mac: Uint8Array,
        ciphertext: Uint8Array,
        nonce: Uint8Array,
        key: Uint8Array,
    ) -> Uint8Array {
        let mut m: Vec<u8> = vec![];

        crypto_box::crypto_box_open_detached_afternm(
            &mut m,
            &to_mac(&mac),
            ciphertext.deref(),
            &to_nonce(&nonce),
            &to_key(&key),
        )
        .unwrap();

        Uint8Array::new(m)
    }

    #[napi(js_name = "crypto_box_open_easy")]
    pub fn crypto_box_open_easy(
        &self,
        ciphertext: Uint8Array,
        nonce: Uint8Array,
        sender_public_key: Uint8Array,
        recipient_secret_key: Uint8Array,
    ) -> Uint8Array {
        let mut m: Vec<u8> = vec![];

        crypto_box::crypto_box_open_easy(
            &mut m,
            ciphertext.deref(),
            &to_nonce(&nonce),
            &to_public_key(&sender_public_key),
            &to_secret_key(&recipient_secret_key),
        )
        .unwrap();

        Uint8Array::new(m.to_vec())
    }

    #[napi(js_name = "crypto_box_seal")]
    pub fn crypto_box_seal(
        &self,
        message: Uint8Array,
        recipient_public_key: Uint8Array,
    ) -> Uint8Array {
        let mut c: Vec<u8> = vec![];

        crypto_box::crypto_box_seal(
            &mut c,
            message.deref(),
            &to_public_key(&recipient_public_key),
        )
        .unwrap();

        Uint8Array::new(c.to_vec())
    }

    #[napi(js_name = "crypto_box_seal_open")]
    pub fn crypto_box_seal_open(
        &self,
        ciphertext: Uint8Array,
        recipient_public_key: Uint8Array,
        recipient_secret_key: Uint8Array,
    ) -> Uint8Array {
        let mut m: Vec<u8> = vec![];

        crypto_box::crypto_box_seal_open(
            &mut m,
            ciphertext.deref(),
            &to_public_key(&recipient_public_key),
            &to_secret_key(&recipient_secret_key),
        )
        .unwrap();

        Uint8Array::new(m)
    }

    #[napi(js_name = "crypto_box_seed_keypair")]
    pub fn crypto_box_seed_keypair(&self, seed: Uint8Array) -> KeyPair {
        let (publickey, secretkey) = crypto_box::crypto_box_seed_keypair(seed.deref());

        KeyPair {
            public_key: Uint8Array::new(publickey.to_vec()),
            secret_key: Uint8Array::new(secretkey.to_vec()),
        }
    }

    #[napi(getter)]
    pub fn crypto_box_beforenmbytes(&self) -> u32 {
        CRYPTO_BOX_BEFORENMBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_macbytes(&self) -> u32 {
        CRYPTO_BOX_MACBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_messagebytes_max(&self) -> u32 {
        CRYPTO_BOX_MESSAGEBYTES_MAX as u32
    }

    #[napi(getter)]
    pub fn crypto_box_noncebytes(&self) -> u32 {
        CRYPTO_BOX_NONCEBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_publickeybytes(&self) -> u32 {
        CRYPTO_BOX_PUBLICKEYBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_sealbytes(&self) -> u32 {
        CRYPTO_BOX_SEALBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_secretkeybytes(&self) -> u32 {
        CRYPTO_BOX_SECRETKEYBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_box_seedbytes(&self) -> u32 {
        CRYPTO_BOX_SEEDBYTES as u32
    }
}
