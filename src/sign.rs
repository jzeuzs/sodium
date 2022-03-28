use crate::{vec_arr_func, types::KeyPair};
use dryoc::constants::*;
use libc::c_ulonglong;
use napi::bindgen_prelude::*;
use sodiumoxide::{crypto::sign, init, ffi};
use std::mem::MaybeUninit;

vec_arr_func!(to_state, u64, 8);
vec_arr_func!(to_count, u64, 2);
vec_arr_func!(to_buf, u8, 128);

#[napi(object)]
pub struct Hash {
    pub state: BigUint64Array,
    pub count: BigUint64Array,
    pub buf: Uint8Array,
}

#[napi(object)]
pub struct State {
    pub hs: Hash,
}

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

    #[napi(js_name = "crypto_sign_ed25519_pk_to_curve25519")]
    pub fn crypto_sign_ed25519_pk_to_curve25519(&self, ed25519_pk: Uint8Array) -> Uint8Array {
        let pk =
            sign::to_curve25519_pk(&sign::PublicKey::from_slice(&ed25519_pk).unwrap()).unwrap();

        Uint8Array::new(pk.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_sign_ed25519_sk_to_curve25519")]
    pub fn crypto_sign_ed25519_sk_to_curve25519(&self, ed25519_sk: Uint8Array) -> Uint8Array {
        let sk =
            sign::to_curve25519_sk(&sign::SecretKey::from_slice(&ed25519_sk).unwrap()).unwrap();

        Uint8Array::new(sk.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_sign_final_create")]
    pub fn crypto_sign_final_create(&mut self, state: State, sk: Uint8Array) -> Uint8Array {
        let mut sig = [0u8; CRYPTO_SIGN_BYTES];
        let mut siglen: c_ulonglong = 0;
        let mut st = ffi::crypto_sign_state {
            hs: ffi::crypto_hash_sha512_state {
                state: to_state(&state.hs.state),
                count: to_count(&state.hs.count),
                buf: to_buf(&state.hs.buf),
            },
        };

        unsafe {
            ffi::crypto_sign_final_create(&mut st, sig.as_mut_ptr(), &mut siglen, sk.as_ptr());
        }

        Uint8Array::new(sig.to_vec())
    }

    #[napi(js_name = "crypto_sign_final_verify")]
    pub fn crypto_sign_final_verify(
        &mut self,
        state: State,
        mut sig: Uint8Array,
        pk: Uint8Array,
    ) -> bool {
        let mut st = ffi::crypto_sign_state {
            hs: ffi::crypto_hash_sha512_state {
                state: to_state(&state.hs.state),
                count: to_count(&state.hs.count),
                buf: to_buf(&state.hs.buf),
            },
        };

        let ret = unsafe { ffi::crypto_sign_final_verify(&mut st, sig.as_mut_ptr(), pk.as_ptr()) };

        ret == 0
    }

    #[napi(js_name = "crypto_sign_init")]
    pub fn crypto_sign_init(&mut self) -> State {
        let mut s = MaybeUninit::uninit();
        let state = unsafe {
            ffi::crypto_sign_init(s.as_mut_ptr());
            s.assume_init()
        };

        State {
            hs: Hash {
                state: BigUint64Array::new(state.hs.state.to_vec()),
                count: BigUint64Array::new(state.hs.count.to_vec()),
                buf: Uint8Array::new(state.hs.buf.to_vec()),
            },
        }
    }

    #[napi(js_name = "crypto_sign_keypair")]
    pub fn crypto_sign_keypair(&self) -> KeyPair {
        let (publickey, secretkey) = sign::gen_keypair();

        KeyPair {
            public_key: Uint8Array::new(publickey.as_ref().to_vec()),
            secret_key: Uint8Array::new(secretkey.as_ref().to_vec()),
        }
    }

    #[napi(js_name = "crypto_sign_open")]
    pub fn crypto_sign_open(&self, sm: Uint8Array, pk: Uint8Array) -> Uint8Array {
        let m = sign::verify(&sm, &sign::PublicKey::from_slice(&pk).unwrap()).unwrap();

        Uint8Array::new(m)
    }

    #[napi(js_name = "crypto_sign_seed_keypair")]
    pub fn crypto_sign_seed_keypair(&self, seed: Uint8Array) -> KeyPair {
        let (publickey, secretkey) =
            sign::keypair_from_seed(&sign::Seed::from_slice(&seed).unwrap());

        KeyPair {
            public_key: Uint8Array::new(publickey.as_ref().to_vec()),
            secret_key: Uint8Array::new(secretkey.as_ref().to_vec()),
        }
    }

    #[napi(js_name = "crypto_sign_update")]
    pub fn crypto_sign_update(&self, state: State, m: Uint8Array) -> State {
        let mut st = ffi::crypto_sign_state {
            hs: ffi::crypto_hash_sha512_state {
                state: to_state(&state.hs.state),
                count: to_count(&state.hs.count),
                buf: to_buf(&state.hs.buf),
            },
        };

        unsafe {
            ffi::crypto_sign_update(&mut st, m.as_ptr(), m.len() as c_ulonglong);
        }

        State {
            hs: Hash {
                state: BigUint64Array::new(st.hs.state.to_vec()),
                count: BigUint64Array::new(st.hs.count.to_vec()),
                buf: Uint8Array::new(st.hs.buf.to_vec()),
            },
        }
    }

    #[napi(js_name = "crypto_sign_verify_detached")]
    pub fn crypto_sign_verify_detached(
        &self,
        sig: Uint8Array,
        m: Uint8Array,
        pk: Uint8Array,
    ) -> bool {
        sign::verify_detached(
            &sign::Signature::from_bytes(&sig).unwrap(),
            &m,
            &sign::PublicKey::from_slice(&pk).unwrap(),
        )
    }

	#[napi(getter)]
	pub fn crypto_sign_bytes(&self) -> u32 {
		CRYPTO_SIGN_BYTES as u32
	}

	#[napi(getter)]
	pub fn crypto_sign_messagebytes_max(&self) -> u32 {
		CRYPTO_SIGN_MESSAGEBYTES_MAX as u32
	}

	#[napi(getter)]
	pub fn crypto_sign_publickeybytes(&self) -> u32 {
		CRYPTO_SIGN_PUBLICKEYBYTES as u32
	}

	#[napi(getter)]
	pub fn crypto_sign_secretkeybytes(&self) -> u32 {
		CRYPTO_SIGN_SECRETKEYBYTES as u32
	}

	#[napi(getter)]
	pub fn crypto_sign_seedbytes(&self) -> u32 {
		CRYPTO_SIGN_SEEDBYTES as u32
	}
}
