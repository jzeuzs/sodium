#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use sodiumoxide::{crypto::box_::curve25519xsalsa20poly1305 as sodium_box, init};
use std::ops::DerefMut;

#[napi(object)]
pub struct KeyPair {
    pub public_key: Uint8Array,
    pub secret_key: Uint8Array,
}

#[napi(js_name = "Box")]
pub struct SodiumBox {}

#[napi]
impl SodiumBox {
    #[napi(constructor)]
    pub fn new() -> Self {
        init().unwrap();
        SodiumBox {}
    }

    #[napi(js_name = "gen_keypair")]
    pub fn gen_keypair(&self) -> Result<KeyPair> {
        let (pubkey, seckey) = sodium_box::gen_keypair();

        Ok(KeyPair {
            public_key: Uint8Array::new(pubkey.as_ref().to_vec()),
            secret_key: Uint8Array::new(seckey.as_ref().to_vec()),
        })
    }

    #[napi(js_name = "gen_nonce")]
    pub fn gen_nonce(&self) -> Result<Uint8Array> {
        let nonce = sodium_box::gen_nonce();

        Ok(Uint8Array::new(nonce.as_ref().to_vec()))
    }

	#[napi(js_name = "keypair_from_seed")]
	pub fn keypair_from_seed(&self, seed: Uint8Array) -> Result<KeyPair> {
		let (pubkey, seckey) = sodium_box::keypair_from_seed(&sodium_box::Seed::from_slice(&seed).unwrap());

		Ok(KeyPair {
			public_key: Uint8Array::new(pubkey.as_ref().to_vec()),
			secret_key: Uint8Array::new(seckey.as_ref().to_vec()),
		})
	}

    #[napi]
    pub fn open(
        &self,
        cipher_text: Uint8Array,
        nonce: Uint8Array,
        public_key: Uint8Array,
        secret_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let decrypted = sodium_box::open(
            &cipher_text,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PublicKey::from_slice(&public_key).unwrap(),
            &sodium_box::SecretKey::from_slice(&secret_key).unwrap(),
        )
        .unwrap();

        Ok(Uint8Array::new(decrypted))
    }

    #[napi(js_name = "open_detached")]
    pub fn open_detached(
        &self,
        mut cipher_text: Uint8Array,
        mac: Uint8Array,
        nonce: Uint8Array,
        public_key: Uint8Array,
        secret_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let ct = cipher_text.deref_mut();

        sodium_box::open_detached(
            ct,
            &sodium_box::Tag::from_slice(&mac).unwrap(),
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PublicKey::from_slice(&public_key).unwrap(),
            &sodium_box::SecretKey::from_slice(&secret_key).unwrap(),
        )
        .unwrap();

        Ok(Uint8Array::new(ct.to_vec()))
    }

    #[napi(js_name = "open_detached_precomputed")]
    pub fn open_detached_precomputed(
        &self,
        mut cipher_text: Uint8Array,
        mac: Uint8Array,
        nonce: Uint8Array,
        precomputed_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let ct = cipher_text.deref_mut();

        sodium_box::open_detached_precomputed(
            ct,
            &sodium_box::Tag::from_slice(&mac).unwrap(),
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PrecomputedKey::from_slice(&precomputed_key).unwrap(),
        )
        .unwrap();

        Ok(Uint8Array::new(ct.to_vec()))
    }

    #[napi(js_name = "open_precomputed")]
    pub fn open_precomputed(
        &self,
        cipher_text: Uint8Array,
        nonce: Uint8Array,
        precomputed_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let decrypted = sodium_box::open_precomputed(
            &cipher_text,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PrecomputedKey::from_slice(&precomputed_key).unwrap(),
        )
        .unwrap();

        Ok(Uint8Array::new(decrypted))
    }

    #[napi]
    pub fn precompute(&self, public_key: Uint8Array, secret_key: Uint8Array) -> Result<Uint8Array> {
        let key = sodium_box::precompute(
            &sodium_box::PublicKey::from_slice(&public_key).unwrap(),
            &sodium_box::SecretKey::from_slice(&secret_key).unwrap(),
        );

        Ok(Uint8Array::new(key.as_ref().to_vec()))
    }

    #[napi]
    pub fn seal(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        public_key: Uint8Array,
        secret_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let cipher_text = sodium_box::seal(
            &message,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PublicKey::from_slice(&public_key).unwrap(),
            &sodium_box::SecretKey::from_slice(&secret_key).unwrap(),
        );

        Ok(Uint8Array::new(cipher_text))
    }

    #[napi(js_name = "seal_detached")]
    pub fn seal_detached(
        &self,
        mut message: Uint8Array,
        nonce: Uint8Array,
        public_key: Uint8Array,
        secret_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let m = message.deref_mut();
        let tag = sodium_box::seal_detached(
            m,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PublicKey::from_slice(&public_key).unwrap(),
            &sodium_box::SecretKey::from_slice(&secret_key).unwrap(),
        );

        Ok(Uint8Array::new(tag.as_ref().to_vec()))
    }

    #[napi(js_name = "seal_detached_precomputed")]
    pub fn seal_detached_precomputed(
        &self,
        mut message: Uint8Array,
        nonce: Uint8Array,
        precomputed_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let m = message.deref_mut();
        let tag = sodium_box::seal_detached_precomputed(
            m,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PrecomputedKey::from_slice(&precomputed_key).unwrap(),
        );

        Ok(Uint8Array::new(tag.as_ref().to_vec()))
    }

    #[napi(js_name = "seal_precomputed")]
    pub fn seal_precomputed(
        &self,
        message: Uint8Array,
        nonce: Uint8Array,
        precomputed_key: Uint8Array,
    ) -> Result<Uint8Array> {
        let cipher_text = sodium_box::seal_precomputed(
            &message,
            &sodium_box::Nonce::from_slice(&nonce).unwrap(),
            &sodium_box::PrecomputedKey::from_slice(&precomputed_key).unwrap(),
        );

        Ok(Uint8Array::new(cipher_text))
    }

    #[napi(getter)]
    #[allow(non_snake_case)]
    pub fn MACBYTES(&self) -> u32 {
        sodium_box::MACBYTES as u32
    }

    #[napi(getter)]
    #[allow(non_snake_case)]
    pub fn NONCEBYTES(&self) -> u32 {
        sodium_box::NONCEBYTES as u32
    }

    #[napi(getter)]
    #[allow(non_snake_case)]
    pub fn PRECOMPUTEDKEYBYTES(&self) -> u32 {
        sodium_box::PRECOMPUTEDKEYBYTES as u32
    }

    #[napi(getter)]
    #[allow(non_snake_case)]
    pub fn PUBLICKEYBYTES(&self) -> u32 {
        sodium_box::PUBLICKEYBYTES as u32
    }

    #[napi(getter)]
    #[allow(non_snake_case)]
    pub fn SECRETKEYBYTES(&self) -> u32 {
        sodium_box::SECRETKEYBYTES as u32
    }

	#[napi(getter)]
	#[allow(non_snake_case)]
	pub fn SEEDBYTES(&self) -> u32 {
		sodium_box::SEEDBYTES as u32
	}
}
