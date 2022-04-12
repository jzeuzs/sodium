#![allow(clippy::new_without_default)]

use std::ops::DerefMut;
use std::ptr::{null, null_mut};

use libc::c_ulonglong;
use napi::bindgen_prelude::*;
use sodiumoxide::crypto::aead::{
    chacha20poly1305 as aead,
    chacha20poly1305_ietf as aead_ietf,
    xchacha20poly1305_ietf as aead_xchacha,
};
use sodiumoxide::{ffi, init};

use crate::types::CryptoBox;

#[napi]
pub struct Aead {}

#[napi]
impl Aead {
    #[napi(constructor)]
    pub fn new() -> Self {
        init().unwrap();
        Aead {}
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_decrypt")]
    pub fn crypto_aead_chacha20poly1305_decrypt(
        &self,
        sn: Option<Uint8Array>,
        c: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        if c.len() < aead::TAGBYTES {
            return Err(Error::new(
                Status::GenericFailure,
                format!("ciphertext is less than {}", aead::TAGBYTES),
            ));
        };

        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut m = Vec::with_capacity(c.len() - aead::TAGBYTES);
        let mut m_len = m.len() as c_ulonglong;

        unsafe {
            let ret = ffi::crypto_aead_chacha20poly1305_decrypt(
                m.as_mut_ptr(),
                &mut m_len,
                sn.map(|mut s| s.as_mut_ptr()).unwrap_or_else(null_mut),
                c.as_ptr(),
                c.len() as c_ulonglong,
                ad_p,
                ad_len,
                aead::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            if ret != 0 {
                return Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret)));
            }

            m.set_len(m_len as usize);
        };

        Ok(Uint8Array::new(m))
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_decrypt_detached")]
    pub fn crypto_aead_chacha20poly1305_decrypt_detached(
        &self,
        sn: Option<Uint8Array>,
        mut c: Uint8Array,
        mac: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let cipher = c.deref_mut();
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let ret = unsafe {
            ffi::crypto_aead_chacha20poly1305_decrypt_detached(
                cipher.as_mut_ptr(),
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                cipher.as_ptr(),
                cipher.len() as c_ulonglong,
                aead::Tag::from_slice(&mac).unwrap().0.as_ptr(),
                ad_p,
                ad_len,
                aead::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead::Key::from_slice(&key).unwrap().0.as_ptr(),
            )
        };

        match ret {
            0 => Ok(Uint8Array::new(cipher.to_vec())),
            _ => Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret))),
        }
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_encrypt")]
    pub fn crypto_aead_chacha20poly1305_encrypt(
        &self,
        m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut c = Vec::with_capacity(m.len() + aead::TAGBYTES);
        let mut c_len = c.len() as c_ulonglong;

        unsafe {
            ffi::crypto_aead_chacha20poly1305_encrypt(
                c.as_mut_ptr(),
                &mut c_len,
                m.as_ptr(),
                m.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            c.set_len(c_len as usize);
        };

        Ok(Uint8Array::new(c))
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_encrypt_detached")]
    pub fn crypto_aead_chacha20poly1305_encrypt_detached(
        &self,
        mut m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<CryptoBox> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let msg = m.deref_mut();
        let mut tag = aead::Tag([0u8; aead::TAGBYTES]);
        let mut mac_len = aead::TAGBYTES as c_ulonglong;

        unsafe {
            ffi::crypto_aead_chacha20poly1305_encrypt_detached(
                msg.as_mut_ptr(),
                tag.0.as_mut_ptr(),
                &mut mac_len,
                msg.as_ptr(),
                msg.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead::Key::from_slice(&key).unwrap().0.as_ptr(),
            );
        };

        Ok(CryptoBox {
            ciphertext: Uint8Array::new(msg.to_vec()),
            mac: Uint8Array::new(tag.as_ref().to_vec()),
        })
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_ietf_decrypt")]
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt(
        &self,
        sn: Option<Uint8Array>,
        c: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        if c.len() < aead_ietf::TAGBYTES {
            return Err(Error::new(
                Status::GenericFailure,
                format!("ciphertext is less than {}", aead_ietf::TAGBYTES),
            ));
        };

        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut m = Vec::with_capacity(c.len() - aead_ietf::TAGBYTES);
        let mut m_len = m.len() as c_ulonglong;

        unsafe {
            let ret = ffi::crypto_aead_chacha20poly1305_ietf_decrypt(
                m.as_mut_ptr(),
                &mut m_len,
                sn.map(|mut s| s.as_mut_ptr()).unwrap_or_else(null_mut),
                c.as_ptr(),
                c.len() as c_ulonglong,
                ad_p,
                ad_len,
                aead_ietf::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_ietf::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            if ret != 0 {
                return Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret)));
            }

            m.set_len(m_len as usize);
        };

        Ok(Uint8Array::new(m))
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_ietf_decrypt_detached")]
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
        &self,
        sn: Option<Uint8Array>,
        mut c: Uint8Array,
        mac: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let cipher = c.deref_mut();
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let ret = unsafe {
            ffi::crypto_aead_chacha20poly1305_ietf_decrypt_detached(
                cipher.as_mut_ptr(),
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                cipher.as_ptr(),
                cipher.len() as c_ulonglong,
                aead_ietf::Tag::from_slice(&mac).unwrap().0.as_ptr(),
                ad_p,
                ad_len,
                aead_ietf::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_ietf::Key::from_slice(&key).unwrap().0.as_ptr(),
            )
        };

        match ret {
            0 => Ok(Uint8Array::new(cipher.to_vec())),
            _ => Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret))),
        }
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_ietf_encrypt")]
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt(
        &self,
        m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut c = Vec::with_capacity(m.len() + aead_ietf::TAGBYTES);
        let mut c_len = c.len() as c_ulonglong;

        unsafe {
            ffi::crypto_aead_chacha20poly1305_ietf_encrypt(
                c.as_mut_ptr(),
                &mut c_len,
                m.as_ptr(),
                m.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead_ietf::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_ietf::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            c.set_len(c_len as usize);
        };

        Ok(Uint8Array::new(c))
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_ietf_encrypt_detached")]
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
        &self,
        mut m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<CryptoBox> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let msg = m.deref_mut();
        let mut tag = aead_ietf::Tag([0u8; aead_ietf::TAGBYTES]);
        let mut mac_len = aead_ietf::TAGBYTES as c_ulonglong;

        unsafe {
            ffi::crypto_aead_chacha20poly1305_ietf_encrypt_detached(
                msg.as_mut_ptr(),
                tag.0.as_mut_ptr(),
                &mut mac_len,
                msg.as_ptr(),
                msg.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead_ietf::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_ietf::Key::from_slice(&key).unwrap().0.as_ptr(),
            );
        };

        Ok(CryptoBox {
            ciphertext: Uint8Array::new(msg.to_vec()),
            mac: Uint8Array::new(tag.as_ref().to_vec()),
        })
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_ietf_keygen")]
    pub fn crypto_aead_chacha20poly1305_ietf_keygen(&self) -> Result<Uint8Array> {
        let key = aead_ietf::gen_key();

        Ok(Uint8Array::new(key.as_ref().to_vec()))
    }

    #[napi(js_name = "crypto_aead_chacha20poly1305_keygen")]
    pub fn crypto_aead_chacha20poly1305_keygen(&self) -> Result<Uint8Array> {
        let key = aead::gen_key();

        Ok(Uint8Array::new(key.as_ref().to_vec()))
    }

    #[napi(js_name = "crypto_aead_xchacha20poly1305_ietf_decrypt")]
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        &self,
        sn: Option<Uint8Array>,
        c: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        if c.len() < aead_xchacha::TAGBYTES {
            return Err(Error::new(
                Status::GenericFailure,
                format!("ciphertext is less than {}", aead_xchacha::TAGBYTES),
            ));
        };

        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut m = Vec::with_capacity(c.len() - aead_xchacha::TAGBYTES);
        let mut m_len = m.len() as c_ulonglong;

        unsafe {
            let ret = ffi::crypto_aead_xchacha20poly1305_ietf_decrypt(
                m.as_mut_ptr(),
                &mut m_len,
                sn.map(|mut s| s.as_mut_ptr()).unwrap_or_else(null_mut),
                c.as_ptr(),
                c.len() as c_ulonglong,
                ad_p,
                ad_len,
                aead_xchacha::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_xchacha::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            if ret != 0 {
                return Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret)));
            }

            m.set_len(m_len as usize);
        };

        Ok(Uint8Array::new(m))
    }

    #[napi(js_name = "crypto_aead_xchacha20poly1305_ietf_decrypt_detached")]
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
        &self,
        sn: Option<Uint8Array>,
        mut c: Uint8Array,
        mac: Uint8Array,
        ad: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let cipher = c.deref_mut();
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let ret = unsafe {
            ffi::crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
                cipher.as_mut_ptr(),
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                cipher.as_ptr(),
                cipher.len() as c_ulonglong,
                aead_xchacha::Tag::from_slice(&mac).unwrap().0.as_ptr(),
                ad_p,
                ad_len,
                aead_xchacha::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_xchacha::Key::from_slice(&key).unwrap().0.as_ptr(),
            )
        };

        match ret {
            0 => Ok(Uint8Array::new(cipher.to_vec())),
            _ => Err(Error::new(Status::GenericFailure, format!("ret {} != 0", ret))),
        }
    }

    #[napi(js_name = "crypto_aead_xchacha20poly1305_ietf_encrypt")]
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt(
        &self,
        m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<Uint8Array> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let mut c = Vec::with_capacity(m.len() + aead_xchacha::TAGBYTES);
        let mut c_len = c.len() as c_ulonglong;

        unsafe {
            ffi::crypto_aead_xchacha20poly1305_ietf_encrypt(
                c.as_mut_ptr(),
                &mut c_len,
                m.as_ptr(),
                m.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead_xchacha::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_xchacha::Key::from_slice(&key).unwrap().0.as_ptr(),
            );

            c.set_len(c_len as usize);
        };

        Ok(Uint8Array::new(c))
    }

    #[napi(js_name = "crypto_aead_xchacha20poly1305_ietf_encrypt_detached")]
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
        &self,
        mut m: Uint8Array,
        ad: Option<Uint8Array>,
        sn: Option<Uint8Array>,
        pn: Uint8Array,
        key: Uint8Array,
    ) -> Result<CryptoBox> {
        let (ad_p, ad_len) =
            ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or_else(|| (null(), 0));

        let msg = m.deref_mut();
        let mut tag = aead_xchacha::Tag([0u8; aead_xchacha::TAGBYTES]);
        let mut mac_len = aead_xchacha::TAGBYTES as c_ulonglong;

        unsafe {
            ffi::crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
                msg.as_mut_ptr(),
                tag.0.as_mut_ptr(),
                &mut mac_len,
                msg.as_ptr(),
                msg.len() as c_ulonglong,
                ad_p,
                ad_len,
                sn.map(|mut n| n.as_mut_ptr()).unwrap_or_else(null_mut),
                aead_xchacha::Nonce::from_slice(&pn).unwrap().0.as_ptr(),
                aead_xchacha::Key::from_slice(&key).unwrap().0.as_ptr(),
            );
        };

        Ok(CryptoBox {
            ciphertext: Uint8Array::new(msg.to_vec()),
            mac: Uint8Array::new(tag.as_ref().to_vec()),
        })
    }

    #[napi(js_name = "crypto_aead_xchacha20poly1305_ietf_keygen")]
    pub fn crypto_aead_xchacha20poly1305_ietf_keygen(&self) -> Result<Uint8Array> {
        let key = aead_xchacha::gen_key();

        Ok(Uint8Array::new(key.as_ref().to_vec()))
    }
}
