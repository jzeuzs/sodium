use crate::vec_arr_func;
use dryoc::constants::*;
use libc::c_ulonglong;
use napi::bindgen_prelude::*;
use sodiumoxide::{crypto::generichash, ffi, init};

vec_arr_func!(to_opaque, u8, 384);

#[napi(object)]
pub struct HashState {
    pub opaque: Uint8Array,
}

#[napi]
pub struct GenericHash {}

#[napi]
impl GenericHash {
    #[napi(constructor)]
    pub fn new() -> Self {
        init().unwrap();
        GenericHash {}
    }

    #[napi(js_name = "crypto_generichash")]
    pub fn crypto_generichash(
        &self,
        data: Uint8Array,
        out_len: Option<u32>,
        key: Option<Uint8Array>,
    ) -> Uint8Array {
        let ol = match out_len {
            Some(l) => Some(l as usize),
            None => None,
        };

        let k = match key {
            Some(ky) => Some(ky.to_vec()),
            None => None,
        };

        let d = generichash::hash(&data, ol, k).unwrap();

        Uint8Array::new(d.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_generichash_final")]
    pub fn crypto_generichash_final(&self, state: HashState, out_len: u32) -> Result<Uint8Array> {
        let mut result = generichash::Digest::new(out_len as usize);
        let mut st = ffi::crypto_generichash_state {
            opaque: to_opaque(&state.opaque),
        };
        let rc =
            unsafe { ffi::crypto_generichash_final(&mut st, result.data.as_mut_ptr(), result.len) };

        match rc {
            0 => Ok(Uint8Array::new(result.data.to_vec())),
            _ => Err(Error::new(
                Status::GenericFailure,
                "Failed to execute".to_string(),
            )),
        }
    }

    #[napi(js_name = "crypto_generichash_init")]
    pub fn crypto_generichash_init(
        &self,
        out_len: Option<u32>,
        key: Option<Uint8Array>,
    ) -> HashState {
        let ol = match out_len {
            Some(l) => Some(l as usize),
            None => None,
        };

        let k = match key {
            Some(ky) => Some(ky.to_vec()),
            None => None,
        };

        let s = generichash::State::new(ol, k).unwrap();

        HashState {
            opaque: Uint8Array::new(s.state.opaque.to_vec()),
        }
    }

    #[napi(js_name = "crypto_generichash_keygen")]
    pub fn crypto_generichash_keygen(&self) -> Uint8Array {
        let mut k = [0u8; CRYPTO_GENERICHASH_KEYBYTES];

        unsafe {
            ffi::crypto_generichash_keygen(k.as_mut_ptr());
        };

        Uint8Array::new(k.to_vec())
    }

    #[napi(js_name = "crypto_generichash_update")]
    pub fn crypto_generichash_update(
        &self,
        state: HashState,
        data: Uint8Array,
    ) -> Result<HashState> {
        let mut st = ffi::crypto_generichash_state {
            opaque: to_opaque(&state.opaque),
        };

        let rc = unsafe {
            ffi::crypto_generichash_update(&mut st, data.as_ptr(), data.len() as c_ulonglong)
        };

        match rc {
            0 => Ok(HashState {
                opaque: Uint8Array::new(st.opaque.to_vec()),
            }),
            _ => Err(Error::new(
                Status::GenericFailure,
                "Failed to execute".to_string(),
            )),
        }
    }
}
