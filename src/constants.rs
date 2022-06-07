#![allow(non_upper_case_globals, non_snake_case)]

use crate::{create_constant, create_constant_fn};

create_constant!(crypto_aead_chacha20poly1305_ABYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_ietf_ABYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_IETF_ABYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_ietf_KEYBYTES, u32);
create_constant_fn!(
    crypto_aead_chacha20poly1305_ietf_MESSAGEBYTES_MAX,
    crypto_aead_chacha20poly1305_ietf_messagebytes_max,
    u32
);
create_constant_fn!(
    crypto_aead_chacha20poly1305_IETF_MESSAGEBYTES_MAX,
    crypto_aead_chacha20poly1305_ietf_messagebytes_max,
    u32
);
create_constant!(crypto_aead_chacha20poly1305_ietf_NPUBBYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_IETF_NPUBBYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_ietf_NSECBYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_IETF_NSECBYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_KEYBYTES, u32);
create_constant_fn!(
    crypto_aead_chacha20poly1305_MESSAGEBYTES_MAX,
    crypto_aead_chacha20poly1305_messagebytes_max,
    u32
);
create_constant!(crypto_aead_chacha20poly1305_NPUBBYTES, u32);
create_constant!(crypto_aead_chacha20poly1305_NSECBYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_ietf_ABYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_IETF_ABYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_ietf_KEYBYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_IETF_KEYBYTES, u32);
create_constant_fn!(
    crypto_aead_xchacha20poly1305_ietf_MESSAGEBYTES_MAX,
    crypto_aead_xchacha20poly1305_ietf_messagebytes_max,
    u32
);
create_constant_fn!(
    crypto_aead_xchacha20poly1305_IETF_MESSAGEBYTES_MAX,
    crypto_aead_xchacha20poly1305_ietf_messagebytes_max,
    u32
);
create_constant!(crypto_aead_xchacha20poly1305_ietf_NPUBBYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_IETF_NPUBBYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_ietf_NSECBYTES, u32);
create_constant!(crypto_aead_xchacha20poly1305_IETF_NSECBYTES, u32);
create_constant!(crypto_auth_BYTES, u32);
create_constant!(crypto_auth_KEYBYTES, u32);
create_constant!(crypto_box_BEFORENMBYTES, u32);
create_constant!(crypto_box_MACBYTES, u32);
create_constant_fn!(crypto_box_MESSAGEBYTES_MAX, crypto_box_messagebytes_max, u32);
create_constant!(crypto_box_NONCEBYTES, u32);
create_constant!(crypto_box_PUBLICKEYBYTES, u32);
create_constant!(crypto_box_SEALBYTES, u32);
create_constant!(crypto_box_SECRETKEYBYTES, u32);
create_constant!(crypto_box_SEEDBYTES, u32);
create_constant!(crypto_generichash_BYTES, u32);
create_constant!(crypto_generichash_BYTES_MAX, u32);
create_constant!(crypto_generichash_BYTES_MIN, u32);
create_constant!(crypto_generichash_KEYBYTES, u32);
create_constant!(crypto_generichash_KEYBYTES_MAX, u32);
create_constant!(crypto_generichash_KEYBYTES_MIN, u32);
create_constant!(crypto_hash_BYTES, u32);
create_constant!(crypto_kdf_BYTES_MAX, u32);
create_constant!(crypto_kdf_BYTES_MIN, u32);
create_constant!(crypto_kdf_CONTEXTBYTES, u32);
create_constant!(crypto_kdf_KEYBYTES, u32);
create_constant!(crypto_kx_PUBLICKEYBYTES, u32);
create_constant!(crypto_kx_SECRETKEYBYTES, u32);
create_constant!(crypto_kx_SEEDBYTES, u32);
create_constant!(crypto_kx_SESSIONKEYBYTES, u32);
create_constant!(crypto_pwhash_ALG_ARGON2I13, u32);
create_constant!(crypto_pwhash_ALG_ARGON2ID13, u32);
create_constant!(crypto_pwhash_ALG_DEFAULT, u32);
create_constant_fn!(crypto_pwhash_BYTES_MAX, crypto_pwhash_bytes_max, u32);
create_constant!(crypto_pwhash_BYTES_MIN, u32);
create_constant!(crypto_pwhash_MEMLIMIT_INTERACTIVE, u32);
create_constant_fn!(crypto_pwhash_MEMLIMIT_MAX, crypto_pwhash_memlimit_max, u32);
create_constant!(crypto_pwhash_MEMLIMIT_MIN, u32);
create_constant!(crypto_pwhash_MEMLIMIT_MODERATE, u32);
create_constant!(crypto_pwhash_MEMLIMIT_SENSITIVE, u32);
create_constant!(crypto_pwhash_OPSLIMIT_INTERACTIVE, u32);
create_constant!(crypto_pwhash_OPSLIMIT_MAX, u32);
create_constant!(crypto_pwhash_OPSLIMIT_MIN, u32);
create_constant!(crypto_pwhash_OPSLIMIT_MODERATE, u32);
create_constant!(crypto_pwhash_OPSLIMIT_SENSITIVE, u32);
create_constant!(crypto_pwhash_PASSWD_MAX, u32);
create_constant!(crypto_pwhash_PASSWD_MIN, u32);
create_constant!(crypto_pwhash_SALTBYTES, u32);
create_constant!(crypto_pwhash_STRBYTES, u32);
create_constant!(crypto_scalarmult_BYTES, u32);
create_constant!(crypto_scalarmult_SCALARBYTES, u32);
create_constant!(crypto_secretbox_KEYBYTES, u32);
create_constant!(crypto_secretbox_MACBYTES, u32);
create_constant_fn!(crypto_secretbox_MESSAGEBYTES_MAX, crypto_secretbox_messagebytes_max, u32);
create_constant!(crypto_secretbox_NONCEBYTES, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_ABYTES, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_HEADERBYTES, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_KEYBYTES, u32);
create_constant_fn!(
    crypto_secretstream_xchacha20poly1305_MESSAGEBYTES_MAX,
    crypto_secretstream_xchacha20poly1305_messagebytes_max,
    u32
);
create_constant!(crypto_secretstream_xchacha20poly1305_TAG_FINAL, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_TAG_MESSAGE, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_TAG_PUSH, u32);
create_constant!(crypto_secretstream_xchacha20poly1305_TAG_REKEY, u32);
create_constant!(crypto_shorthash_BYTES, u32);
create_constant!(crypto_shorthash_KEYBYTES, u32);
create_constant!(crypto_sign_BYTES, u32);
create_constant_fn!(crypto_sign_MESSAGEBYTES_MAX, crypto_sign_messagebytes_max, u32);
create_constant!(crypto_sign_PUBLICKEYBYTES, u32);
create_constant!(crypto_sign_SECRETKEYBYTES, u32);
create_constant!(crypto_sign_SEEDBYTES, u32);
create_constant!(SODIUM_LIBRARY_VERSION_MAJOR, u32);
create_constant!(SODIUM_LIBRARY_VERSION_MINOR, u32);

#[napi(js_name = "crypto_pwhash_STRPREFIX")]
pub fn crypto_pwhash_STRPREFIX() -> String {
    unsafe { String::from_utf8_unchecked(sodiumoxide::ffi::crypto_pwhash_STRPREFIX.to_vec()) }
}

#[napi(js_name = "SODIUM_VERSION_STRING")]
pub fn SODIUM_VERSION_STRING() -> String {
    unsafe { String::from_utf8_unchecked(sodiumoxide::ffi::SODIUM_VERSION_STRING.to_vec()) }
}
