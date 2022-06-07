#![allow(clippy::new_without_default)]

use std::ops::{Deref, DerefMut};

use dryoc::classic::crypto_box::{
    crypto_box_seal as box_seal,
    crypto_box_seal_open as box_seal_open,
};
use dryoc::constants::*;
use dryoc::types::*;
use napi::bindgen_prelude::*;
use sodiumoxide::crypto::box_;
use sodiumoxide::init;

use crate::types::{CryptoBox, KeyPair};
use crate::vec_arr_func;

vec_arr_func!(to_public_key, u8, CRYPTO_BOX_PUBLICKEYBYTES);
vec_arr_func!(to_secret_key, u8, CRYPTO_BOX_SECRETKEYBYTES);

#[napi(js_name = "Box")]
pub struct SodiumBox {}

#[napi]
impl SodiumBox {
    #[napi(constructor)]
    pub fn new() -> Self {
        init().unwrap();
        SodiumBox {}
    }

    #[napi(js_name = "crypto_box_beforenm")]
    pub fn crypto_box_beforenm(&self, pk: Uint8Array, sk: Uint8Array) -> Uint8Array {
        let key = box_::precompute(
            &box_::PublicKey::from_slice(&pk).unwrap(),
            &box_::SecretKey::from_slice(&sk).unwrap(),
        );

        Uint8Array::new(key.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_box_detached")]
    pub fn crypto_box_detached(
        &self,
        mut m: Uint8Array,
        n: Uint8Array,
        pk: Uint8Array,
        sk: Uint8Array,
    ) -> CryptoBox {
        let ms = m.deref_mut();
        let mac = box_::seal_detached(
            ms,
            &box_::Nonce::from_slice(&n).unwrap(),
            &box_::PublicKey::from_slice(&pk).unwrap(),
            &box_::SecretKey::from_slice(&sk).unwrap(),
        );

        CryptoBox {
            ciphertext: Uint8Array::new(ms.to_vec()),
            mac: Uint8Array::new(mac.as_ref().to_vec()),
        }
    }

    #[napi(js_name = "crypto_box_easy")]
    pub fn crypto_box_easy(
        &self,
        m: Uint8Array,
        n: Uint8Array,
        pk: Uint8Array,
        sk: Uint8Array,
    ) -> Uint8Array {
        let c = box_::seal(
            &m,
            &box_::Nonce::from_slice(&n).unwrap(),
            &box_::PublicKey::from_slice(&pk).unwrap(),
            &box_::SecretKey::from_slice(&sk).unwrap(),
        );

        Uint8Array::new(c)
    }

    #[napi(js_name = "crypto_box_keypair")]
    pub fn crypto_box_keypair(&self) -> KeyPair {
        let (publickey, secretkey) = box_::gen_keypair();

        KeyPair {
            public_key: Uint8Array::new(publickey.as_ref().to_vec()),
            secret_key: Uint8Array::new(secretkey.as_ref().to_vec()),
        }
    }

    #[napi(js_name = "crypto_box_nonce")]
    pub fn crypto_box_nonce(&self) -> Uint8Array {
        let nonce = box_::gen_nonce();

        Uint8Array::new(nonce.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_box_open_detached")]
    pub fn crypto_box_open_detached(
        &self,
        mut c: Uint8Array,
        mac: Uint8Array,
        n: Uint8Array,
        pk: Uint8Array,
        sk: Uint8Array,
    ) -> Uint8Array {
        let ct = c.deref_mut();

        box_::open_detached(
            ct,
            &box_::Tag::from_slice(&mac).unwrap(),
            &box_::Nonce::from_slice(&n).unwrap(),
            &box_::PublicKey::from_slice(&pk).unwrap(),
            &box_::SecretKey::from_slice(&sk).unwrap(),
        )
        .unwrap();

        Uint8Array::new(ct.to_vec())
    }

    #[napi(js_name = "crypto_box_open_detached_afternm")]
    pub fn crypto_box_open_detached_afternm(
        &self,
        mut c: Uint8Array,
        mac: Uint8Array,
        n: Uint8Array,
        k: Uint8Array,
    ) -> Uint8Array {
        let ct = c.deref_mut();

        box_::open_detached_precomputed(
            ct,
            &box_::Tag::from_slice(&mac).unwrap(),
            &box_::Nonce::from_slice(&n).unwrap(),
            &box_::PrecomputedKey::from_slice(&k).unwrap(),
        )
        .unwrap();

        Uint8Array::new(ct.to_vec())
    }

    #[napi(js_name = "crypto_box_open_easy")]
    pub fn crypto_box_open_easy(
        &self,
        c: Uint8Array,
        n: Uint8Array,
        pk: Uint8Array,
        sk: Uint8Array,
    ) -> Uint8Array {
        let pt = box_::open(
            &c,
            &box_::Nonce::from_slice(&n).unwrap(),
            &box_::PublicKey::from_slice(&pk).unwrap(),
            &box_::SecretKey::from_slice(&sk).unwrap(),
        )
        .unwrap();

        Uint8Array::new(pt)
    }

    #[napi(js_name = "crypto_box_seal")]
    pub fn crypto_box_seal(&self, m: Uint8Array, pk: Uint8Array) -> Uint8Array {
        let mut c: Vec<u8> = vec![Default::default(); m.len() + CRYPTO_BOX_SEALBYTES];

        box_seal(&mut c, m.deref(), &to_public_key(&pk)).unwrap();

        Uint8Array::new(c.to_vec())
    }

    #[napi(js_name = "crypto_box_seal_open")]
    pub fn crypto_box_seal_open(
        &self,
        c: Uint8Array,
        pk: Uint8Array,
        sk: Uint8Array,
    ) -> Uint8Array {
        let mut m: Vec<u8> = vec![Default::default(); c.len() - CRYPTO_BOX_SEALBYTES];

        box_seal_open(&mut m, c.deref(), &to_public_key(&pk), &to_secret_key(&sk)).unwrap();

        Uint8Array::new(m)
    }

    #[napi(js_name = "crypto_box_seed_keypair")]
    pub fn crypto_box_seed_keypair(&self, seed: Uint8Array) -> KeyPair {
        let (publickey, secretkey) =
            box_::keypair_from_seed(&box_::Seed::from_slice(&seed).unwrap());

        KeyPair {
            public_key: Uint8Array::new(publickey.as_ref().to_vec()),
            secret_key: Uint8Array::new(secretkey.as_ref().to_vec()),
        }
    }
}
